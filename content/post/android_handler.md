---
date: 2017-03-27
title: "Android handler机制简陋描述(golang)"
draft: false
categories:
  - Android
tags:
  - Android
  - Handler
thumbnailImagePosition: left
---

Android是消息驱动，在线程间通讯方面为开发者提供了Handler-Looper机制。Android中的Handler机制，blablabla\~\~\~\~此处省略若干字\~\~\~\~，总之就是个生产者消费者模型。

<!--more-->
Handler机制中，要素有3个：

- Looper
- Handler
- Message

##### Looper
Looper有一个MessageQueue，Looper主要工作不停地从MessageQueue中取出消息，根据消息的Target分发。
```go
type Looper struct {
	MessageQueue chan *Message
}

var mainLooper Looper
var once sync.Once

func (l *Looper)GetMainLooper() *Looper {
	once.Do(func() {
		mq := make(chan *Message, 300)
		mainLooper = Looper{MessageQueue:mq}
	})
	return &mainLooper
}

func (l *Looper)Prepare() {
	if l == &mainLooper {
		panic(" - -、")
	}
	l.MessageQueue = make(chan *Message, 300)
}
func (l *Looper)Loop() {
	for {
		m := <-l.MessageQueue
		m.Target.Callback(m)
	}
}
```

#### Handler
Handler主要包含一个Looper和一个回调函数。Handler发送消息时将消息打上标记(Target)，然后将其加入Looper的MessageQueue。
```go
type Handler struct {
	looper   *Looper
	Callback func(*Message)
}

func (h *Handler)Post(msg *Message) {
	msg.Target = h
	h.looper.MessageQueue <- msg
}
```

#### Message
Message为消息，除了包含一些简单的数据外，还持有一个Handler引用（指针）为Target成员变量。
```go
type Message struct {
	What   int
	Arg1   int
	Arg2   int
	Target *Handler
}
```
---
### Have a try
```go
func main() {
	var l Looper
    // 获取MainLooper，当然也可以创建一个新的。这里是强行模拟Android主线程的套路
	mLooper := l.GetMainLooper()
    // 创建一个Handler，它的looper为MainLooper。回调方法只是简单地输出Message中What变量的值
	h := &Handler{looper:mLooper, Callback:func(msg *Message) {
		fmt.Println("Looper goroutine...")
		fmt.Println(msg.What)
	}}
	defer mLooper.Loop()
    // 异步循环获取用户输入
	go func() {
		for {
			var i int
			_, err := fmt.Scan(&i)
			if err != nil {
				continue
			}else {
                // 将用户输入的数值作为消息发送
				h.Post(&Message{What:i})
                if i < 0 {
					os.Exit(0)
                }
			}
		}
	}()
}
```