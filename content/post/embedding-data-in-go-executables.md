---
title: "[译]在go可执行文件中嵌入数据"
date: 2017-09-08
draft: false
categories:
  - golang
tags:
  - golang
---

>原文地址 https://scene-si.org/2017/08/22/embedding-data-in-go-executables/

假如你已经关注了我一段时间，应该知道我正在开发[Pendulum编辑器](https://github.com/titpetric/pendulum)作为每天至少编码一小时的[#100DaysOfCode](http://100daysofcode.com/)挑战。Pendulum是一个非常适合编辑简单文本和markdown文件的基于web的编辑器。
<!--more-->

实际上这篇文章就是用它写的。它由go后端和VueJS前端组成。我希望它易于使用并提供包含一切的单个可执行文件，因此用户不需要下载安装器并解压文件。我需要找个能把所有东西打包到一块的方案。我决定用[go-bindata](https://github.com/jteeuwen/go-bindata)以代码生成的方式来把所有数据通过 go build 添加到可执行文件中。

### 代码生成？

当然这很简单。例如 go-bindata 工具可以帮我们从`public_html`目录生成对应的 .go 文件。这对我的应用场景来说是极好的。不过为什么要用bash脚本或者makefile来生成它呢？因为这样我们在执行`go build`之前就只需要通过执行一下`go generate`利用go的代码生成工具了
如果还不熟悉代码生成，你只需要在你代码的某处加上简单的注释，用`main.go`举个例子：
```go
package main

//go:generate echo "Hello world"

func main() {
}
```
执行`go generate`时，可以看到输出了 “Hello world” 。这不是你用 go generate 生成代码的实际需求。你在`//go:generate`后面写的一切都会执行。如果你想的话，甚至可以执行`go build`。
```go
package main

//go:generate echo "Hello world"
//go:generate go run main.go

func main() {
    println("Hello world from Go")
}
```
运行这个会有预期的输出：
```sh
%go generate
Hello world
Hello world from Go
```
旗开得胜！go generate 很有意思。Node程序通过`babel`来使Node ES5运行时兼容ES6/ES7的语法。人们正尝试用类似的途径为go提供超出语言目前功能的特性。

例如，[genny](https://github.com/cheekybits/genny)主要针对强类型代码的生成，因此不再需要手动复制粘贴。不过[Have](http://havelang.org/)这样的项目更接近Babel对Node的处理--提供转换到go的语言。目前我还不清楚这方面更有吸引力的其他尝试。不过关于Go2及泛型的讨论似乎比较有趣。

这对我们的应用场景来说略显枯燥，我们只是要把一些数据打包到程序中。那么闲话休提，书归正传：
```go
//go:generate go-bindata -prefix front/src -o assets/bindata.go -pkg assets -nomemcopy front/src/dist/...
```
这一行略长，就把它拆分来看：

- `//go:generate` - 为`go generate`作提示
- `go-bindata` - 要执行的主命令
- `-prefix front/src` - 排除“front/src”包
- `-o assets/bindata.go` - 指定输出文件
- `-pkg assets` - 要生成的包名
- `-nomemcopy` - 对[内存占用](https://github.com/jteeuwen/go-bindata#lower-memory-footprint)的优化
- `front/src/dist/...` - 要打包的地方

这会在应用目录下创建一个可以简单的用`app/assets`导入的`assets`包，其中`app`对应的是应用目录。

### 通过HTTP提供嵌入的文件服务
这稍微有点复杂。不过看一下文档之后就简单了。如果要基于本地文件提供服务，你大致需要下面这几行类似的代码：
```go
folder := http.Dir("/")
server := http.FileServer(folder)
http.Handle("/", server)
```
实际上，[go-bindata-assetfs](https://github.com/elazarl/go-bindata-assetfs)包已经提供了一个http.FileServer实现。这个用起来就够简单了:
```go
import "github.com/elazarl/go-bindata-assetfs"
import "app/assets"
// ...
func main() {
    // ...
    files := assetfs.AssetFS{
        Asset:     assets.Asset,
        AssetDir:  assets.AssetDir,
        AssetInfo: assets.AssetInfo,
        Prefix:    "dist",
    }
    server := http.FileServer(&files)
    // ...
}
```
还有一个小问题。我用的是启用了pushHistory的VueJS应用。这就意味着，用户使用时会看到没有释伴符(哈希，#)的类似`/blog/about.md`的普通链接。这些需要被应用处理的链接内容在asset中并不存在。

这个问题也不难解决。`assetfs.AssetFS`结构体有一个`AssetsInfo`方法（相当于`os.Stat`）和一个`Asset`方法（有点像`ioutil.ReadFile`）。这使检查一个文件是否存在于asset，若不存在则输出另一个文件成为可能：
```go
// Serves index.html in case the requested file isn't found
// (or some other os.Stat error)
func serveIndex(serve http.Handler, fs assetfs.AssetFS) http.HandlerFunc {
    return func(w http.ResponseWriter, r *http.Request) {
        _, err := fs.AssetInfo(path.Join(fs.Prefix, r.URL.Path))
        if err != nil {
            contents, err := fs.Asset(path.Join(fs.Prefix, "index.html"))
            if err != nil {
                http.Error(w, err.Error(), http.StatusNotFound)
                return
            }
            w.Header().Set("Content-Type", "text/html")
            w.Write(contents)
            return
        }
        serve.ServeHTTP(w, r)
    }
}
```
如果找到了文件，就用预置的ServeHTTP方法取代我自己的实现。采用这种方法只需要对我们之前定义的handler稍作调整：
```go
http.HandleFunc("/", serveIndex(server, assets))
```
`serveIndex`函数返回一个`http.HandlerFunc`,这行是相应的修改。这就提供了你用 go generate 和 go-bindata 添加到应用中的数据服务的完整实现。如果你想跳过`//go:generate`环节把这些放到CI脚本中也是可以的。

鉴于此我实现了[Pendulum](https://github.com/titpetric/pendulum)的单个可执行发布版本。可以从[GitHub发布页](https://github.com/titpetric/pendulum/releases)获取并尝试。

*编辑：改进serveIndex示例 [感谢@Rdihipone](https://www.reddit.com/r/golang/comments/6vcl8u/embedding_data_in_go_executables_tit_petric/dm0eauu/)*

### 当你看到了这里...
要是你能买本我的书定是极好的：

- [API Foundations in Go](https://leanpub.com/api-foundations)
- [12 Factor Apps with Docker and Go](https://leanpub.com/12fa-docker-golang)
- [The SaaS Handbook (work in progress)](https://leanpub.com/saas-handbook)

I promise you'll learn a lot more if you buy one. Buying a copy supports me writing more about similar topics. Say thank you and buy my books.
Feel free to [send me an email](black@scene-si.org) if you want to book my time for consultancy/freelance services. I'm great at APIs, Go, Docker, VueJS and scaling services, [among many other things](https://scene-si.org/about).
