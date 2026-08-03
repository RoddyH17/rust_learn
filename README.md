# rust_learn — 每日 Rust 学习

一天一个文件夹:可运行的代码 + 学习笔记。配套博客(反思 + C++ 对比)在 [RoddyH17.github.io](https://github.com/RoddyH17/RoddyH17.github.io)(本地 `~/blog`)。

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
cd ~/rust_learn && ./new_day.sh 2 ownership
# ... 写代码 (cd day2/ownership && cargo run)、写 day2/NOTES.md ...

# 2. 建博客草稿(写完删掉 draft: true 行)
cd ~/blog && ./new_post.sh rust_learn day-2-ownership "Day 2 · 所有权"

# 3. 收工:两边各推一次
cd ~/rust_learn && ./sync.sh "day 2: ownership"
cd ~/blog && ./sync.sh "post: day 2 ownership"
```

## 学习进度

| Day | 日期 | 主题 | 博客 |
|-----|------|------|------|
| 1 | 2026-07-31 | 环境安装 · hello world · cargo | [Day 1](https://roddyh17.github.io/posts/rust_learn/day-1-hello-rust/) |
