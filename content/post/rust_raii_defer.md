---
title: "RAII: Rust 模拟 golang 中的 defer"
author: "liyiheng"
date: 2020-11-04T19:24:44+08:00
subtitle: "dtor finally"
image: ""
tags: [rust]
type: ""
---

Go 中的 `defer` 非常实用。今天尝试在 Rust 中模拟 `defer`。


<!--more-->
Rust 和 C++ 一样采用 RAII 的方式管理资源，需要 `defer` 的场景并不多。
Rust 没有 `defer`， 不过可以通过实现 `Drop trait`，利用 RAII 机制简单模拟一下 `defer`。

大致思路：
- 需要延期执行的代码放入闭包
- 创建结构体持有闭包
- 为该结构体类型实现 `Drop`，在 `drop` 中调用闭包。

这样，在该类型作用域结束时，`drop` 被调用，进而执行闭包中的逻辑。

具体实现：
```rust
pub struct Defer<T: FnOnce()> {
    h: Option<T>,
}
impl<T: FnOnce()> Drop for Defer<T> {
    fn drop(&mut self) {
        self.h.take().map(|f| f()).unwrap_or(());
    }
}

pub fn defer(func: impl FnOnce()) -> Defer<impl FnOnce()> {
    Defer { h: Some(func) }
}

fn main() {
    let _guard = defer(|| {
        println!("main funcion finished");
    });
    println!("Hello world!");
}
```

参考文档：
- [FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)
- [dtor-finally](https://github.com/rust-unofficial/patterns/blob/master/idioms/dtor-finally.md)
