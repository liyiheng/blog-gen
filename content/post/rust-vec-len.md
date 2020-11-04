---
title: "[Rust学习]创建Vec指定长度" 
date: 2018-06-27
draft: false
---


golang中可创建指定长度的切片，Rust中的Vec只能在创建时指定容量。

```rust
let v: Vec<i32> = Vec::with_capacity(3);
println!("{}", v[2]);// index out of bounds
```
<!--more-->

### Unsafe 的 `set_len()`

```rust
let mut v: Vec<i32> = Vec::new();
unsafe { v.set_len(3); }
println!("v:{}", v[2]);// SIGSEGV, 段错误
```
点进去发现，
```rust
    /// Sets the length of a vector.
    ///
    /// This will explicitly set the size of the vector, without actually
    /// modifying its buffers, so it is up to the caller to ensure that the
    /// vector is actually the specified size.
```
因此实用 `set_len` 需要确保容量

```rust
 let mut v: Vec<i32> = Vec::with_capacity(3);
 unsafe { v.set_len(3); }
 println!("v:{}", v[2]);  // v:0
```

### 通过 `vec!` 宏创建
```rust
let v = vec![0;10];
```

### 通过 `std::vec::from_elem` 创建
```rust
let v = std::vec::from_elem(0, 10);
```
