# rust_learn — 每日 Rust 学习

一天一个文件夹:可运行的代码 + 学习笔记。配套博客(反思 + C++ 对比)在独立仓库 [RoddyH17/blog](https://github.com/RoddyH17/blog)。

- ✍️ **博客**: https://roddyh17.github.io/blog/
- 📁 **本仓库**: `dayN/` — 每天的可运行 crate + `NOTES.md`

## 目录结构

```
rust_learn/
├── day1/
│   ├── hello_world/        # rustc 裸编译示例
│   ├── hello_cargo/        # cargo 标准项目
│   └── NOTES.md            # 当天学习笔记
├── day2/ ...               # 以后每天照此新建
└── sync.sh                 # 一键提交+推送
```

## 每日工作流

```bash
# 1. 写代码 + 笔记(本仓库)
cd ~/rust_learn && mkdir dayN && cd dayN
cargo new <topic>
# ... 写代码、写 dayN/NOTES.md ...
./sync.sh "day N: <topic>"

# 2. 写博客(~/blog 仓库)
#    新建 ~/blog/content/day-N-<topic>.mdx,然后:
cd ~/blog && ./sync.sh "post: day N <topic>"
```

## 学习进度

| Day | 日期 | 主题 | 博客 |
|-----|------|------|------|
| 1 | 2026-07-31 | 环境安装 · hello world · cargo | [Day 1](https://roddyh17.github.io/blog/blog/day-1-hello-rust/) |
