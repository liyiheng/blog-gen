---
date: 2017-01-05
title: "WebSocket初探"
draft: false
categories:
  - golang
tags:
  - WebSocket
  - golang
  - go
thumbnailImagePosition: left
---



作为一名Android开发狗，目前为止笔者接触到的网络开发几乎全是http请求，即时通讯则是第三方SDK。</br>
http是无状态协议，即时通讯？还是xmpp或者直接怼tcp吧（qq似乎用的udp？）。
网页上实现即时通讯的话，xmpp,tcp,udp似乎都不方便用，似乎还是用http轮询来得简单。不过轮询的方式对资源的消耗比较严重。
<!--more-->

做自由的程序猿，搞喜欢的技术，接下来跟Android就没有直接联系了。</br>
前段时间了解到了WebSocket，在看build-web-application-with-golang的时候发现谢大在书中对WebSocket有了详细的讲解，我就不再搬了，感兴趣的同学点击[传送门](https://github.com/astaxie/build-web-application-with-golang "build-web-application-with-golang")即可。</br>
看完手痒，不妨动手撸一个简易（简陋）的web聊天室。</br>
以下是实现过程。有意见欢迎[提issue](https://github.com/XanthusL/websocket-demo/issues "issues")；有建议欢迎[提issue](https://github.com/XanthusL/websocket-demo/issues "issues")；想喷我欢迎[提issue](https://github.com/XanthusL/websocket-demo/issues "issues")；想砍我欢迎[提issue](https://github.com/XanthusL/websocket-demo/issues "issues")

-------------------------------------------------------------

客户端部分也就是网页，核心部分用JavaScript实现（废话），以下是代码（感谢[小伙伴](https://github.com/moshen1223 "moshen1223")）

	    var sock = null;
        var wsuri = "ws://192.168.1.104:1234/chat/in";
        window.onload = function() {
            sock = new WebSocket(wsuri);
            sock.onmessage = function(e) {
                document.getElementById('view').innerHTML += (e.data +'<br/>');
            }
        };
        function send() {
            var msg = document.getElementById('message').value;
            sock.send(msg);
        };

客户端逻辑比较简单，点击按钮后调用send方法将用户输入的内容通过WebSocket发给后端，
另一方面，接受到后端的数据后展示给用户

-------------------------------------------------------------

服务端逻辑比网页稍多，不过也很简单，服务端用golang实现</br>

##### 大致思路 

1. 每当有人访问时，产生WebSocket连接，将该连接的指针保存起来，暂且称为连接池吧。</br>
2. 而后便是一个无限的循环，跳出条件为从该连接接收数据失败，也就是连接断开后便跳出循环，跳出循环后将该连接的指针从连接池中移除。</br>
3. 循环体不断从连接中接收数据，接收到后将数据发给连接池中的每一个连接。</br>
    
##### 具体实现 

WebSocket需要用到golang.org/x/net/websocket包，

    go get golang.org/x/net/websocket

由于golang.org被墙，这种获取方式需要科学上网，也可以从github.com/golang/net/websocket中获取，
只是获取之后需要mv以下。

定义三个常量，代码中引用，方便修改

    //连接池的容量，若已达到最大值，
    //再有用户建立连接后直接返回加入聊天室失败
    const MAX_CONNECTION int = 100 
    //连接池用map实现，key为int，加入连接池成功后返回一个自增的id，
    //失败则返回-1
    const JOIN_ROOM_FAILED int = -1  
    //用来做一些控制，作用不大
    const Debug = true

定义聊天室(连接池)的结构

    type ChatRoom struct {
        //锁，防止并发时连接数超过最大值	
        sync.Mutex
        //用于存放连接指针的map，key为int型的id
        clients   map[int]*websocket.Conn
        //id为自增，通过currentId控制
        currentId int
    }

有了结构，再来三个方法就能起飞了</br>

- 加入聊天室
- 离开聊天室
- 发送消息

joinRoom方法用来加入聊天室，加入时分配给用户一个id，加入成功后将id返回，加入失败则返回JOIN_ROOM_FAILED

    func (cr *ChatRoom)joinRoom(ws *websocket.Conn) int {
        cr.Lock()		//加锁
        defer cr.Unlock() //解锁用defer，不用defer的话需要在返回前解锁
        if len(cr.clients) >= MAX_CONNECTION {
            return JOIN_ROOM_FAILED
        }
        cr.currentId++
        cr.clients[cr.currentId] = ws
        return cr.currentId
    }

leftRoom方法在退出聊天室时调用，将连接移除。

    func (cr *ChatRoom)leftRoom(id int) {
        delete(cr.clients, id)
    }

发送消息时，sendMessage方法遍历以保存的所有连接并发送。

    func (cr *ChatRoom)sendMessage(msg string) {
        for _, ws := range cr.clients {
            if err := websocket.Message.Send(ws, msg); err != nil {
                log4Demo("发送失败，Err：" + err.Error())
                //continue
            }
        }
    }


现在路面畅通，随时可以开车

    // 先声明一个聊天室类型的变量，也就是所谓的“连接池”
    var room ChatRoom

除此之外，还需要两个Handler或者两个Handler方法：一个用来将页面传给浏览器，另一个专门处理WebSocket连接

    //处理页面
    func Page(writer http.ResponseWriter, request *http.Request) {
        t, _ := template.ParseFiles("test.html")
        err:=t.Execute(writer, nil)
        log4Demo("Page Err:" + err.Error())
    }
    //处理WebSocket连接
    func Chat(ws *websocket.Conn) {
        var id int
        if id = room.joinRoom(ws); id == JOIN_ROOM_FAILED {
            websocket.Message.Send(ws, "加入聊天室失败")
            return
        }
        defer room.leftRoom(id)
        ipAddress := strings.Split(ws.Request().RemoteAddr, ":")[0] + "："
        var err error
        for {
            var msg string
            if err = websocket.Message.Receive(ws, &msg); err != nil {
                log4Demo("Failed to receive. Err:" + err.Error())
                break
            }
            msg = ipAddress + msg
            room.sendMessage(msg)
        }
    }

最后就是main方法了，在main方法中初始化一下room变量，再分别注册两个handler方法，监听1234端口

    func main() {
        roomMap := make(map[int]*websocket.Conn, MAX_CONNECTION)
        room = ChatRoom{clients:roomMap, currentId:0}

        http.Handle("/chat/in", websocket.Handler(Chat))
        http.HandleFunc("/", Page)

        if err := http.ListenAndServe(":1234", nil); err != nil {
            log.Fatal("ListenAndServe:", err)
        }
    }

现在可以运行一下试试火力了

    go run main.go

浏览器打开http://localhost:1234/ 就可以看到一个简(简)洁(陋)美(至)观(极)的聊天页面了</br>
在其他设备同时打开该地址(注意改ip)，多个设备之间就可以实现即时通讯了</br>
[详细代码地址](https://github.com/XanthusL/websocket-demo "WebSocket-demo")




