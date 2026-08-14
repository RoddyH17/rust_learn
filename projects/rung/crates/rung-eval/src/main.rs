//! Rung 性能评测程序。
//!
//! **你不需要修改这个文件。** 它和课程作业里的 eval 程序是一个定位:
//! 提供一个固定的、可复现的测量口径,好让不同实现之间的数字可以直接比。
//!
//! # 用法
//!
//! ```text
//! cargo run --release -p rung-eval              # 全部模式 × 全部规模
//! cargo run --release -p rung-eval -- high-cross    # 只跑一种模式
//! cargo run --release -p rung-eval -- high-cross 5000
//! ```
//!
//! 一般不直接跑,走 `./scripts/eval.sh` —— 它会先验证测试通过。
//!
//! # 为什么必须 --release
//!
//! debug build 关掉了所有优化,还开着整数溢出检查和额外的边界检查。
//! 用 debug build 测出来的数字和真实性能没有关系,得出的结论是无意义的。
//!
//! # 为什么按「模式 × 规模」两维测
//!
//! 只看规模会漏掉最重要的东西。一个订单簿在「全是挂单不成交」和
//! 「每笔都穿越成交」两种负载下的表现可能差一个数量级,而真实市场
//! 在开盘、盘中、收盘是三种完全不同的模式。
//!
//! 三种模式各自对应真实场景:
//!
//! | 模式 | 模拟什么 | 压的是哪条路径 |
//! |---|---|---|
//! | `insert-only` | 开盘前集合竞价堆单 | 挂单 + 最优价查询 |
//! | `high-cross` | 盘中活跃成交 | 撮合主循环 |
//! | `cancel-heavy` | 做市商高频改价 | 撤单定位 |

use std::process::ExitCode;
use std::time::Instant;

use rung_core::{
    ArenaLevel, BoxListLevel, DequeLevel, LevelQueue, NaiveBook, Order, OrderId, Price, Qty, Side,
};

// ---------------------------------------------------------------------------
// 确定性随机数
// ---------------------------------------------------------------------------

/// xorshift64*。够随机,而且**完全确定** —— 同一个种子永远给同一串数。
///
/// 不用 `rand` crate:评测必须可复现,而且这个 workspace 现在还不该有第三方依赖。
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed | 1) // 种子不能是 0
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// `[0, n)` 区间内的一个数。
    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

// ---------------------------------------------------------------------------
// 订单流
// ---------------------------------------------------------------------------

enum Op {
    Insert(Order),
    Submit(Order),
    Cancel(OrderId),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pattern {
    InsertOnly,
    HighCross,
    CancelHeavy,
}

impl Pattern {
    fn name(self) -> &'static str {
        match self {
            Pattern::InsertOnly => "insert-only",
            Pattern::HighCross => "high-cross",
            Pattern::CancelHeavy => "cancel-heavy",
        }
    }

    fn parse(s: &str) -> Option<Pattern> {
        match s {
            "insert-only" => Some(Pattern::InsertOnly),
            "high-cross" => Some(Pattern::HighCross),
            "cancel-heavy" => Some(Pattern::CancelHeavy),
            _ => None,
        }
    }

    fn describe(self) -> &'static str {
        match self {
            Pattern::InsertOnly => "纯挂单,价格散布在宽区间,几乎不成交",
            Pattern::HighCross => "价格集中在窄区间,大量穿越成交",
            Pattern::CancelHeavy => "挂了就撤,模拟做市商高频改价",
        }
    }
}

/// 生成一串确定性的订单流。种子固定 = 每次跑得到完全相同的一串。
fn gen_ops(pattern: Pattern, n: u64) -> Vec<Op> {
    let mut rng = Rng::new(0xC0FF_EE00 ^ n);
    let mut ops = Vec::with_capacity(n as usize * 2);
    let mut live: Vec<OrderId> = Vec::new();

    // 价格区间的宽度决定了成交概率:窄 = 容易穿越 = 多成交。
    let span: i64 = match pattern {
        Pattern::InsertOnly => 2000, // 20.00 元宽,两边基本碰不到
        Pattern::HighCross => 20,    // 0.20 元宽,频繁穿越
        Pattern::CancelHeavy => 400,
    };
    let mid: i64 = 10_000; // 100.00 元

    for i in 0..n {
        // id 从 1 开始:0 留给「未分配」。
        let id: OrderId = i + 1;

        let side = if rng.below(2) == 0 {
            Side::Buy
        } else {
            Side::Sell
        };
        // 买单挂在中价下方,卖单挂在中价上方,再加一点随机偏移。
        let offset = rng.below(span as u64) as i64;
        let price = match side {
            Side::Buy => mid - offset,
            Side::Sell => mid + offset,
        };
        let qty = 1 + rng.below(100);

        let order = Order::new(id, side, Price::from_ticks(price), Qty::new(qty));

        match pattern {
            Pattern::InsertOnly => {
                ops.push(Op::Insert(order));
                live.push(id);
            }
            Pattern::HighCross => {
                ops.push(Op::Submit(order));
                live.push(id);
            }
            Pattern::CancelHeavy => {
                ops.push(Op::Insert(order));
                live.push(id);
                // 一半的概率立刻撤掉一笔还活着的单 —— 做市商改价的形态
                if live.len() > 4 && rng.below(2) == 0 {
                    let idx = rng.below(live.len() as u64) as usize;
                    ops.push(Op::Cancel(live.swap_remove(idx)));
                }
            }
        }
    }
    ops
}

// ---------------------------------------------------------------------------
// 测量
// ---------------------------------------------------------------------------

struct Measurement {
    elapsed_s: f64,
    ops: usize,
    trades: usize,
    final_len: usize,
    ns_per_op: f64,
}

fn run(pattern: Pattern, n: u64) -> Measurement {
    let ops = gen_ops(pattern, n);
    let op_count = ops.len();

    let mut book = NaiveBook::new();
    let mut trades = 0usize;

    // 计时只覆盖执行,不覆盖生成 —— 生成订单流的成本不属于被测系统。
    let t0 = Instant::now();
    for op in &ops {
        match op {
            Op::Insert(o) => {
                let _ = book.insert(o.clone());
            }
            Op::Submit(o) => {
                if let Ok(ts) = book.submit(o.clone()) {
                    trades += ts.len();
                }
            }
            Op::Cancel(id) => {
                let _ = book.cancel(*id);
            }
        }
    }
    let elapsed = t0.elapsed();

    Measurement {
        elapsed_s: elapsed.as_secs_f64(),
        ops: op_count,
        trades,
        final_len: book.len(),
        ns_per_op: elapsed.as_nanos() as f64 / op_count as f64,
    }
}

fn eval_pattern(pattern: Pattern, sizes: &[u64]) {
    println!();
    println!("═══ {} ═══", pattern.name());
    println!("    {}", pattern.describe());
    println!();
    println!(
        "  {:>8}  {:>10}  {:>12}  {:>12}  {:>10}  {:>10}",
        "n", "ops", "elapsed (s)", "ns/op", "trades", "book len"
    );
    println!("  {}", "─".repeat(72));

    for &n in sizes {
        let m = run(pattern, n);
        println!(
            "  {:>8}  {:>10}  {:>12.6}  {:>12.1}  {:>10}  {:>10}",
            n, m.ops, m.elapsed_s, m.ns_per_op, m.trades, m.final_len
        );
    }
}

// ---------------------------------------------------------------------------
// 阶段 2 · 档位队列基准
// ---------------------------------------------------------------------------

/// 一个档位内的混合负载:挂 n 笔 → 随机摘掉一半 → 剩下的全部出队。
///
/// 负载形状照真实档位设计:
/// - `push_back` 每笔订单都做
/// - `remove` 是撤单(做市商改价),占比很高
/// - `pop_front` 是成交
///
/// 三份实现跑**完全相同**的操作序列(同一个种子),所以数字可以直接比。
/// 打乱顺序在计时之外完成 —— 生成负载的成本不属于被测系统。
fn bench_level<L: LevelQueue>(n: u64) -> (f64, f64) {
    // 确定性地生成「摘除顺序」。三份实现拿到的是同一份。
    let mut rng = Rng::new(0xBEEF_0000 ^ n);
    let mut remove_order: Vec<usize> = (0..n as usize).collect();
    for i in (1..remove_order.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        remove_order.swap(i, j);
    }
    let half = remove_order.len() / 2;

    let mut level = L::new();
    let mut handles: Vec<L::Handle> = Vec::with_capacity(n as usize);

    let t0 = Instant::now();

    for i in 0..n {
        handles.push(level.push_back(i, Qty::new(1)));
    }
    for &k in &remove_order[..half] {
        let _ = level.remove(handles[k]);
    }
    while level.pop_front().is_some() {}

    let elapsed = t0.elapsed();
    let ops = n as usize + half + (n as usize - half);
    (
        elapsed.as_secs_f64(),
        elapsed.as_nanos() as f64 / ops as f64,
    )
}

fn eval_level(sizes: &[u64]) {
    println!();
    println!("═══ 档位队列:三份实现对比(阶段 2) ═══");
    println!("    负载:挂 n 笔 → 随机摘掉一半 → 剩下的全部出队");
    println!();
    println!(
        "  {:>7}  {:>12}  {:>12}  {:>12}  {:>10}  {:>10}",
        "n", "Deque ns/op", "BoxList ns/op", "Arena ns/op", "Box 加速", "Arena 加速"
    );
    println!("  {}", "─".repeat(76));

    for &n in sizes {
        let (_, d) = bench_level::<DequeLevel>(n);
        let (_, b) = bench_level::<BoxListLevel>(n);
        let (_, a) = bench_level::<ArenaLevel>(n);
        println!(
            "  {:>7}  {:>12.1}  {:>12.1}  {:>12.1}  {:>9.2}×  {:>9.2}×",
            n,
            d,
            b,
            a,
            d / b,
            d / a
        );
    }

    println!();
    println!("  「加速」列以 DequeLevel 为基准。小于 1 表示更慢。");
    println!("  预期:BoxList 在每一项上都更慢(push_back 是 O(n));");
    println!("        Arena 的优势随 n 增大而扩大(remove 是 O(1) 而非 O(n))。");
    println!("  如果实测和预期不符,那正是报告里最值得写的一段。");
}

fn usage() {
    eprintln!("rung-eval —— 测量订单簿与档位队列的性能");
    eprintln!();
    eprintln!("用法:");
    eprintln!("  rung-eval                      全部模式 × 全部规模(NaiveBook)");
    eprintln!("  rung-eval <pattern>            指定模式");
    eprintln!("  rung-eval <pattern> <n>        指定模式与规模");
    eprintln!("  rung-eval level                档位队列三份实现对比(阶段 2)");
    eprintln!("  rung-eval level <n>            指定规模");
    eprintln!();
    eprintln!("模式:");
    for p in [
        Pattern::InsertOnly,
        Pattern::HighCross,
        Pattern::CancelHeavy,
    ] {
        eprintln!("  {:<14} {}", p.name(), p.describe());
    }
    eprintln!();
    eprintln!("⚠️  必须用 --release 编译,否则数字无意义。");
    eprintln!("    建议走 ./scripts/eval.sh —— 它会先验证测试通过。");
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // NaiveBook 是 O(n) 全表扫描,规模上去会非常慢。
    // 这几个数字是为了让整轮评测在一分钟内跑完而挑的。
    let default_sizes = [100u64, 200, 500, 1000, 2000];
    let all = [
        Pattern::InsertOnly,
        Pattern::HighCross,
        Pattern::CancelHeavy,
    ];

    match args.len() {
        0 => {
            println!("Rung 评测 · NaiveBook 基线");
            println!("(这些数字是后面每个阶段报告里加速比的分母)");
            for p in all {
                eval_pattern(p, &default_sizes);
            }
        }
        1 if args[0] == "level" => eval_level(&default_sizes),
        1 => match Pattern::parse(&args[0]) {
            Some(p) => eval_pattern(p, &default_sizes),
            None => {
                eprintln!("❌ 未知模式: {}\n", args[0]);
                usage();
                return ExitCode::FAILURE;
            }
        },
        2 if args[0] == "level" => {
            let Ok(n) = args[1].parse::<u64>() else {
                eprintln!("❌ 规模必须是正整数: {}\n", args[1]);
                usage();
                return ExitCode::FAILURE;
            };
            eval_level(&[n]);
        }
        2 => {
            let Some(p) = Pattern::parse(&args[0]) else {
                eprintln!("❌ 未知模式: {}\n", args[0]);
                usage();
                return ExitCode::FAILURE;
            };
            let Ok(n) = args[1].parse::<u64>() else {
                eprintln!("❌ 规模必须是正整数: {}\n", args[1]);
                usage();
                return ExitCode::FAILURE;
            };
            eval_pattern(p, &[n]);
        }
        _ => {
            usage();
            return ExitCode::FAILURE;
        }
    }

    println!();
    println!("把这张表抄进 reports/stage-01.md 的「定量评估」一节。");
    ExitCode::SUCCESS
}
