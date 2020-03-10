---
date: 2019-02-23
title: "从“unsafe”看数据结构对齐"
categories:
  - rust
tags:
  - rust
---

数据结构对齐是什么？维基百科对其有如下定义：

> 数据结构对齐是代码编译后在内存的布局与使用方式。包括三方面内容：数据对齐、数据结构填充（padding）与包入（packing）。<br>
内存地址a被称为n字节对齐，当a是n的倍数（n应是2的幂）。<br>
一次内存访问被称为对齐的，当被访问的数据长度为n 字节且该数据地址为n字节对齐。如果内存未对齐，称作misaligned。显然，字节访问总是对齐的。<br>
内存指针是对齐的，如果它所指的数据是对齐的。指向聚合数据（aggregate data，如struct或数组）是对齐的，当且仅当它的每个组成数据是对齐的。<br>


<!--more-->
以8字节对齐为例：

```go
type foo struct{
	a int64
	b int32
	c int8
}
//  |<-------a------>|
//  |<--b--->|-c-|---|

type bar struct{
	a int64
	b int8
	c int32
}
//  |<-------a------>|
//  |-b-|---|<---c-->|
```
`foo`在内存中占16字节：

1. 前8字节为`foo.a`
2. `foo.b`占接下来的4字节
3. `foo.c`占2字节
4. 最后填充2个空白字节

类似的，`bar`也占16字节，但填充字节不在最后。

### 从裸数据到结构体

根据数据结构对齐规则，可以在相同字节序及对齐数下方便地实现数据结构与它裸数据的相互转换。

例如，在`boltdb`中有如下代码：
```go
// pageInBuffer retrieves a page reference from a given byte array based on the current page size.
func (db *DB) pageInBuffer(b []byte, id pgid) *page {
	return (*page)(unsafe.Pointer(&b[id*pgid(db.pageSize)]))
}
```

### 简单试验

```rust
#[derive(Debug)]
struct Foo {
    a: u64,
    b: u16,
    c: u16,
}

fn convert(dat: &[u8]) -> Foo {
    let mut v = Foo { a: 0, b: 0, c: 0 };
    unsafe {
        std::ptr::copy_nonoverlapping(
            dat.as_ptr(),                  // src ptr
            &mut v as *mut Foo as *mut u8, // dst ptr
            16,                            // ptr length
        );
    }
    return v;
}

fn main() {
    let align = std::mem::align_of::<Foo>();
    let mut dat: Vec<u8> = vec![1, 1, 0, 0, 0, 0, 0, 0]; // 257
    dat.append(&mut vec![0, 1]); // 256
    dat.append(&mut vec![1, 0]); // 1
    let v = convert(&dat[..]);
    println!("Align:{}", align);
    println!("{:?}", v);
}
```

### 结论

1. `unsafe`灵活、强大
2. `unsafe`有风险
3. 慎用`unsafe`
4. 慎用`unsafe`
5. 慎用`unsafe`







