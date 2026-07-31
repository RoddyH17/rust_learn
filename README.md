# rust_learn — 每日 Rust 学习管道

一天一个文件夹,一篇博客。代码、笔记、博客同仓库,push 即发布。

- 🌐 **博客**: https://roddyh17.github.io/rust_learn/
- 📁 **代码**: `dayN/` — 每天的可运行 crate + `NOTES.md` 学习笔记
- ✍️ **文章**: `blog/content/*.mdx` — 纯 Markdown/MDX,直接编辑即可

## 目录结构

```
rust_learn/
├── day1/
│   ├── hello_world/        # rustc 裸编译示例
│   ├── hello_cargo/        # cargo 标准项目
│   └── NOTES.md            # 当天学习笔记
├── day2/ ...               # 以后每天照此新建
├── blog/                   # Next.js 博客 (dillionverma/portfolio 模板改造)
│   ├── content/            # ★ 博客文章都在这里,MDX 格式
│   └── src/data/resume.tsx # 首页个人信息
├── sync.sh                 # 一键提交+推送(推送后自动部署博客)
└── .github/workflows/deploy.yml  # GitHub Pages 自动部署
```

## 每日工作流

```bash
# 1. 新建当天目录并写代码
mkdir -p dayN && cd dayN
cargo new <topic>
# ... 写代码、写 dayN/NOTES.md ...

# 2. 写当天博客(纯 markdown,frontmatter 照抄前一篇改日期标题)
#    blog/content/day-N-<topic>.mdx

# 3. 一键同步(自动 commit + push + 触发博客部署)
./sync.sh "day N: <topic>"
```

博客文章 frontmatter 格式:

```yaml
---
title: "Day N · 标题"
publishedAt: "2026-08-01"
author: "Roddy"
summary: "一句话摘要"
---
```

## 本地预览博客

```bash
cd blog && npm run dev
# 打开 http://localhost:3000/rust_learn
```

## 学习进度

| Day | 日期 | 主题 | 博客 |
|-----|------|------|------|
| 1 | 2026-07-31 | 环境安装 · hello world · cargo | [Day 1](https://roddyh17.github.io/rust_learn/blog/day-1-hello-rust/) |
