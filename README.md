# rust_learn — 每日 Rust 学习

一天一个文件夹:可运行的代码 + 学习笔记。把 Rust 当作第一门编程语言,从零学起。配套博客在 [RoddyH17.github.io](https://github.com/RoddyH17/RoddyH17.github.io)(本地 `~/blog`)。

**内容分工**:`NOTES.md` 是技术细节的唯一来源(命令、概念、踩坑),博客只写学习叙事与反思,两边互相链接、不重复内容。

- ✍️ **博客**: https://roddyh17.github.io/
- 📁 **本仓库**: `dayN/` — 每天的可运行 crate + `NOTES.md`

## 目录结构

```
rust_learn/
├── day1/
│   ├── hello_world/        # rustc 裸编译示例
│   ├── hello_cargo/        # cargo 标准项目
│   └── NOTES.md            # 当天学习笔记
├── day2/ ...
├── new_day.sh              # 新建一天的脚手架
└── sync.sh                 # 一键提交+推送
```

## 每日工作流(三条命令)

```bash
# 1. 开工:建 dayN 目录 + cargo 项目 + 笔记模板
cd ~/rust_learn && ./new_day.sh 2 variables
# ... 写代码 (cd day2/variables && cargo run)、写 day2/NOTES.md ...

# 2. 建博客草稿(写完删掉 draft: true 行)
cd ~/blog && ./new_post.sh rust day-2-variables "Day 2 · Variables"

# 3. 收工:两边各推一次
cd ~/rust_learn && ./sync.sh "day 2: variables"
cd ~/blog && ./sync.sh "post: day 2 variables"
```

## 学习进度

| Day | 日期 | 主题 | 笔记 | 博客 |
|-----|------|------|------|------|
| 1 | 2026-07-31 | 环境安装 · hello world · cargo | [NOTES](day1/NOTES.md) | [Day 1](https://roddyh17.github.io/posts/rust/day-1-hello-rust/) |
| 2 | 2026-08-04 | Variables · const/static · shadowing · references | [NOTES](day2/NOTES.md) | [Day 2](https://roddyh17.github.io/posts/rust/day-2-variables/) |
| 3 | 2026-08-03 | 结构体 · 方法 · impl | [NOTES](day3/NOTES.md) | [Day 3](https://roddyh17.github.io/posts/rust/day-3-structs/) |
