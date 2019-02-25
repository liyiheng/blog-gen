---
date: 2019-01-23
title: "Rust实现终端中的“PPT”"
draft: true
categories:
  - rust
tags:
  - rust
---

[Tpp(text presentation program)](https://github.com/cbbrowne/tpp)是个有趣的命令行程序，用于在终端中展示信息。基于`ncurses`，用Ruby实现。这里将尝试用Rust实现它的基本功能。<br>
项目地址：https://github.com/XanthusL/tpp-rs

<!--more-->
### 缘由

需要用到tpp，发现它依赖:

- Ruby 1.8 http://www.ruby-lang.org/
- ncurses
- [ncurses-ruby](https://github.com/eclubb/ncurses-ruby) 

从AUR安装后仍提示`ncurses-ruby`未安装（通过gem安装未出现此问题），于是决定用Rust来实现一个，顺便学习`ncurses`的基本使用。


### Tpp文件格式

### 解析Tpp文件

### Ncurses



