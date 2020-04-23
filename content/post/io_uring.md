---
title: "io_uring 初探"
author: "liyiheng"
date: 2020-04-22T22:44:44+08:00
subtitle: "通过 Linux 5.1+ 的 io_uring 读取文件"
image: ""
tags: [rust]
type: ""
---

Linux 在 5.1 引入了 `io_uring`，据说性能接近 SPKD。
目前 Linux 已更新到 5.6.6 ，不妨尝试一下 `io_uring`。


<!--more-->

`io_uring`API 主要围绕着两个队列和一个数组：
SubmissionQueue(SQ) 、CompletionQueue(CQ) 、SubmissionQueueEntries(SQEs)。

SQEs 中为实际的请求，SQ 与 CQ 中节点保存的是请求在 SQEs 中的偏移量。

大致流程为，应用程序从 SQEs 中取空闲的 SQE，根据需要设置各项参数，
并将其偏移量存入 SQ， 最后提交 SQ。
请求完成后，内核将其放入 CQ。

也就是说，应用程序是 SQ 的生产者，是 CQ 的消费者；
内核是 SQ 的消费者，CQ 的生产者。

尽管与 `epoll`、`libaio` 相比，`io_uring` 简单易用了很多，
但还涉及到内存屏障等若干细节。Jens Axboe 为此开发了 `liburing`，
进一步简化了 `io_uring` 的使用。

这里通过 `liburing` 的 Rust 绑定 `iou` 尝试对文件进行读操作。

```rust
use iou;
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;

fn main() {
    let mut buffer = [0u8; 1024];
    let buffers = vec![io::IoSlice::new(&buffer)];
    let file = File::open("/etc/pacman.d/mirrorlist").expect("Failed to open file");
    let fds = vec![file.as_raw_fd()];

    // 创建 io_ring, 指定 SQ 与 CQ 长度为 8
    let mut ring = iou::IoUring::new(8).expect("Failed to create io_uring");

    let registrar = ring.registrar();
    registrar
        .register_files(&fds)
        .expect("Failed to register fds");
    registrar
        .register_buffers(&buffers)
        .expect("Failed to register buffers");
    unsafe {
        // 取可用的 SQE 并准备读文件的请求
        ring.next_sqe()
            .unwrap()
            .prep_read_fixed(file.as_raw_fd(), &mut buffer, 0, 0);
    }
    ring.submit_sqes().unwrap();
    ring.wait_for_cqe().unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));
}
```
