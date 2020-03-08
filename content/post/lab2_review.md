---
title: "6.824 Lab2 总结"
author: "liyiheng"
date: 2020-03-07T11:44:44+08:00
subtitle: "记录实验中的挖坑与填坑"
image: ""
tags: [rust]
type: ""
---

6.824课程的Lab2是实现Raft，分为2A、2B、2C三部分。
2A实现leader选举与心跳；2B实现日志复制；2C实现持久化。


<!--more-->

# 2A

2A部分比较简单，当时的实现相当粗暴：
采用互斥锁的并发范式，Node持有`Arc<Mutex<Raft>>`。
Raft线程执行一个无尽的循环，每次循环都睡200ms。
如果该节点为leader，则给其他节点发送心跳；
否则检查是否选举超时，若已超时，则从follower成为candidate并发起选举。

```rust
/// Create a new raft service.
pub fn new(raft: Raft) -> Node {
    let raft = Arc::new(Mutex::new(raft));
    let rf = raft.clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(200));
        if rf.lock().unwrap().state.is_leader() {
            let raft = rf.lock().unwrap();
            Self::send_heartbeat(raft);
        } else {
            let millis = rand::thread_rng().gen_range(1000, 4000);
            let election_timeout = Duration::from_millis(millis);
            if rf.lock().unwrap().last_heartbeat.elapsed() > election_timeout {
                Self::start_election(rf.clone());
            }
        }
    });
    Node { raft }
}
```
现在看来，这个实现有2个明显的问题：
1. Raft线程长期出于休眠状态，能做的事有限，不方便后续增加新的逻辑
2. 发起选举的时机不对：比如，选举超时是410ms，该Node并不会在410ms后发起选举，而是在600ms时发起。
因为400ms时检查尚未超时，下次检查则是200ms后。这就大大增加了多个节点同时发起选举的概率。

针对第一个问题，改进方案是用`futures_timer`中的`Interval`代替`thread::sleep()`, 
`Interval`实现了`Stream`，后续通过`select(event_receiver)`(mailbox)，采用Actor模型，可以方便地增加其他逻辑。
第一个问题解决后，第二个问题则迎刃而解：随机出一个选举超时时间，通过`Delay`准时将发起选举的事件发到Raft线程的`mailbox`。

初步改进：
```rust
std::thread::spawn(move || {
    Interval::new(Duration::from_millis(200))
        .for_each(|_| {
            if rf.lock().unwrap().state.is_leader() {
                let raft = rf.lock().unwrap();
                Self::send_heartbeat(raft);
            } else {
                let millis = rand::thread_rng().gen_range(1000, 4000);
                let election_timeout = Duration::from_millis(millis);
                if rf.lock().unwrap().last_heartbeat.elapsed() > election_timeout {
                    Self::start_election(rf.clone());
                }
            }
            Ok(())
        })
        .wait()
        .unwrap();
});
```

# 2B

2B部分为日志复制，实现的过程中有部分精力用于重构2A。
2B本身不复杂，但需要注意一些细节：`term`、`prev_log_term`、`match_index`、`next_index`。
例如，由于粗心，`AppendEntries`参数中把`term`做为`prev_log_term`传递；
以及`next_index`维护逻辑、日志`up-to-date`的判断条件等。
总之，精力浪费在了寻找各种小细节导致的bug和重构，不过过程中思路愈加清晰。

2B部分完成后的Raft线程除了发送心跳外，可以接收其他事件并进行相应的逻辑，大致如下：
```rust
fn start_raft_thread(raft: Arc<Mutex<Raft>>) -> UnboundedSender<Event> {
    let (event_tx, event_rx) = futures::sync::mpsc::unbounded();
    raft.lock().unwrap().event_ch = Some(event_tx.clone());
    std::thread::spawn(move || {
        let event_rx = event_rx.map_err(|_| ()).map(Some);
        Interval::new(INTERVAL_PERIOD)
            .map(|_| None)
            .map_err(|_| ())
            .select(event_rx)
            .for_each(|event| {
                if let Some(event) = event {
                    raft.lock().unwrap().handle_event(event);
                } else {
                    raft.lock().unwrap().step();
                }
                Ok(())
            })
            .wait()
            .unwrap();
    });
    event_tx
}
```
`Node`结构体则持有`Sender`, 由于个别场景直接用锁执行`Raft`方法非常方便，
没有完全去除`Arc<Mutex<Raft>>`:
```rust
#[derive(Clone)]
pub struct Node {
    sender: UnboundedSender<Event>,
    raft: Arc<Mutex<Raft>>,
}
```
目前来看，`Node`可以通过`sender`给`Raft`发送事件，但很多时候需要拿到事件的处理结果，
于是在`Event`上做文章：
```rust
enum Event {
    RequestVote(RequestVoteArgs, oneshot::Sender<Reply>),
    AppendEntries(AppendEntriesArgs, oneshot::Sender<Reply>),
    VoteResult(u64, usize),
    // ... 省略其他
}

#[derive(Clone)]
enum Reply {
    RequestVote(RequestVoteReply),
    AppendEntries(AppendEntriesReply),
}
```
`Event`可以同时携带参数和用于发送处理结果的`Sender`。
如果需要某事件的处理结果，则创建新的`channel`，将参数和`sender`一并发给`Raft`，
而`Node`则将`receiver`作为一个`RpcFuture`返回给调用者，`Raft`处理完后，用`send`发送处理结果。
例如：
```rust
let (tx, rx) = oneshot::channel();
let result = self
    .sender
    .unbounded_send(Event::AppendEntries(args, tx))
    .map_err(|e| RpcError::Other(e.to_string()));
if let Err(e) = result {
    return Box::new(future::err(e));
}
Box::new(rx.then(|reply| match reply {
    Ok(Reply::AppendEntries(reply)) => Ok(reply),
    _ => Err(RpcError::Timeout),
}))
```

# 2C

接下来实现持久化部分。本以为，2C最简单，
只要在论文中所说的`persistent state`发生变化后及时调用`persist`即可。
然而完成后发现多个用例不通过。

第一个原因是，目前的实现存在一个bug：没有判断事件的时效性。
用例模拟了网络不稳定，RPC调用有快有慢、以及特定概率地超时等各种场景。
节点收到的事件可能不是本个`term`的，或者节点角色已发生变化，此时应丢弃改事件。

第二个原因是没有实现`kill`方法。该方法用于模拟节点崩溃的场景，
测试用例使某节点崩溃方式是调用其`kill`方法。此方法没有实现，
会导致本该崩溃的节点仍在运行，干扰正常流程。


第一个问题的解决比较简单，增加判断即可，第二个问题需要通知Raft线程退出。
在循环中跳出比较简单，`break`即可，而Raft线程的逻辑在`Interval`中， 
似乎是无穷的。经查阅文档发现`take_while`，完美解决：
```rust
Interval::new(INTERVAL_PERIOD)
    .map(|_| None)
    .map_err(|_| ())
    .select(event_rx)
    .take_while(|event| {
        let has_next = if let Some(Event::Shutdown) = event {
            info!("Peer {} shutdown", raft.lock().unwrap().me);
            false
        } else {
            true
        };
        future::ok(has_next)
    })
    .for_each(|event| {
        if let Some(event) = event {
            raft.lock().unwrap().handle_event(event);
        } else {
            raft.lock().unwrap().step();
        }
        Ok(())
    })
    .wait()
    .unwrap();
```

到这里，事情并未结束。仍有个别用例失败。经过观察发现，
`unreliable`的情况下，RPC是非常的unreliable, 超时概率非常高，
这就导致leader选举频繁失败，通常数次才能成功选举，
而刚选举成功的leader极有可能连续多次往某个节点发送心跳失败，该节点重新发起选举。

目前的解决方案是增加心跳的频率，比如原来的200ms一次，到现在的50ms一次,
心跳成功率不够，用次数弥补。
加大选举超时时长，允许一定次数的心跳丢失。不过这种方式会使RPC次数增多，影响效率。

