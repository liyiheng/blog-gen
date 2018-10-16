---
date: 2017-11-06
title: "[golang小工具]静态文件服务器"
draft: false
categories:
  - golang
tags:
  - golang
#thumbnailImagePosition: left
---



把电脑上的文件传到手机上，或者传给其他电脑，这是再常见不过的事了。<br>
这种情况通常是用PC端的qq或微信给手机发送文件。<br>
不过对Linux用户来说，wine上的qq多少有些不靠谱，web微信在传输大文件时也经常出问题。<br>

<!--more-->
顺便安利一下[electronic-wechat](https://github.com/geeeeeeeeek/electronic-wechat)。<br>
另外，无论是qq还是微信，手机接收后的文件都藏的比较深（tencent/MicroMsg/file_recv?），要是在Downloads下多好。
 

不妨自己写个静态文件服务器，用手机浏览器下载文件<br><br>

### 0x0000
用golang写这种东西太简单了
```golang
func main() {
        http.Handle("/", http.FileServer(http.Dir("./")))
        e := http.ListenAndServe(":8080", nil)
        if e != nil {
             fmt.Println(e.Error())
        }
}
```
关键代码就2行

### 0x0001
用了几次后觉得不爽，要分享哪个目录就得把程序移到该目录下再执行；而且端口是写死的。于是：
```golang
func main() {
        dir := "./"
        if len(os.Args) > 1 {
                dir = os.Args[1]
        }
        fmt.Println("[Static file server] start, port:8080")
        http.Handle("/", http.FileServer(http.Dir(dir)))
        e := http.ListenAndServe(":8080", nil)
        if e != nil {
                fmt.Println(e.Error())
        }
}
```
这样的话，只需要把编译好的程序放到$PATH任意目录，用的时候:
```sh
file-server path/to/share
```

### 0x0002
似乎好多了，不过还有一个尴尬的问题。IP ！<br>
把本机IP打印出来岂不更好<br>
最终版本:
```golang
//usr/bin/env go run "$0" "$@"; exit "$?"
package main

import (
        "fmt"
        "net"
        "net/http"
        "os"
        "os/signal"
        "syscall"
)

func main() {
        dir := "./"
        if len(os.Args) > 1 {
                dir = os.Args[1]
        }
        ips, _ := localIPs()
        fmt.Println("Local IP addresses:")
        for _, v := range ips {
                fmt.Printf("\t%s\n", v)
        }
        fmt.Println("[Static file server] start, port:8080")
        http.Handle("/", http.FileServer(http.Dir(dir)))
        go func() {
                e := http.ListenAndServe(":8080", nil)
                if e != nil {
                        os.Exit(1)
                }
        }()
        osCh := make(chan os.Signal, 1)
        fmt.Println("Start Signal Hooker!")
        signal.Notify(osCh, syscall.SIGHUP, syscall.SIGQUIT, syscall.SIGTERM, syscall.SIGINT) // , syscall.SIGSTOP) cannot compile on windows
        fmt.Printf("\rGot a signal [%s]\n", <-osCh)

}

// from https://github.com/Akagi201/utilgo/blob/master/ips/ips.go
func localIPs() ([]string, error) {
        var ips []string
        addrs, err := net.InterfaceAddrs()
        if err != nil {
                return ips, err
        }

        for _, a := range addrs {
                if ipnet, ok := a.(*net.IPNet); ok && !ipnet.IP.IsLoopback() && ipnet.IP.To4() != nil {
                        ips = append(ips, ipnet.IP.String())
                }
        }

        return ips, nil
}
```
基本上大概也许算得上好用了吧，虽然端口还是写死的。<br>
```sh
file-server
Local IP addresses:
	192.168.1.67
	172.17.0.1
[Static file server] start, port:8080
Start Signal Hooker!
```
接收目录参数直接用的`os.Args`,<br>
为什么不用`flag`包？<br>
为什么不顺便吧端口号也写成可指定的？<br>

>因为够(tài)用(lǎn)了,觉得不够用请提PR或[issue](https://github.com/XanthusL/blog-gen/issues)
