---
date: 2017-06-22
title: "[译]Golang中JSON和结构体的组合使用"
draft: false
categories:
  - golang
tags:
  - golang
  - JSON
thumbnailImagePosition: left
---

原文地址：http://attilaolah.eu/2014/09/10/json-and-struct-composition-in-go/
<br>
假设你正在把一个JSON对象解码为Go的结构体。该JSON来自不受你控制的服务，因此你无法操作它的模式。但你想用不同的方式进行编码。
 
<!--more-->

你可以随意使用`json.Marshaler`，但它有一些坑：<br>

- **复杂度**： 为了大型结构体添加大量额外代码
- **内存占用**： 为了不分配不必要的内存需要尽量小心

其实，大多数情况下你可以在`MarshalJSON()`中避免内存分配，但这会增加复杂度，因为这些处理在你的代码中（而不是`encoding/json`），因此你得对它进行单元测试。这就需要撸更多枯燥的代码。<br>

下面是一些处理大结构体的一些小技巧。


## 忽略字段
假设你有这么一个结构体：
```go
type User struct {
    Email    string `json:"email"`
    Password string `json:"password"`
    // 大量字段。。。
}
```
现在你需要对`User`进行转码，但不能包含`password`字段。通过结构体组合处理这种情况的简单方法是用另一个结构体包裹它：
```go
type omit *struct{}

type PublicUser struct {
    *User
    Password omit `json:"password,omitempty"`
}

// 当你想对User进行编码时：
json.Marshal(PublicUser{
    User: user,
})
```
这里的技巧是我们不会设置`PublishUser`的`Password`字段，由于是指针类型，它的默认值是`nil`，并且会被忽略（因为设置了`omitempty`）。<br>
需要注意的是，没有必要声明`omit`类型，我们可以用`*struct{}`甚至`bool`或`int`。但是声明这个类型能够显示地表示我们要在输出中忽略该字段。使用哪个内建类型并不重要，只要它是能被`omitempty`识别的零值。<br>
我们也可以只用匿名值：
```go
json.Marshal(struct {
    *User
    Password bool `json:"password,omitempty"`
}{
    User: user,
})
```
去playground[试试](http://play.golang.org/p/aED6MyYDaJ)<br>
另外需要注意的是，我们只包含了一个原始`User`结构体的指针，这避免了复制一个新的`User`。

## 增加额外字段
添加字段甚至比忽略字段更简单。继续我们之前的栗子，隐藏密码字段再暴露一个额外的`token`属性：
```go
type omit *struct{}

type PublicUser struct {
    *User
    Token    string `json:"token"`
    Password omit   `json:"password,omitempty"`
}

json.Marshal(PublicUser{
    User:  user,
    Token: token,
})
```
去playground[试试](https://play.golang.org/p/ckNtE6FhOt)<br>

## 组合结构体
这在组合来自不同服务的数据时很方便。举个例子，这是个含有统计数据的的`BlogPost`结构体：
```go
type BlogPost struct {
    URL   string `json:"url"`
    Title string `json:"title"`
}

type Analytics struct {
    Visitors  int `json:"visitors"`
    PageViews int `json:"page_views"`
}

json.Marshal(struct{
    *BlogPost
    *Analytics
}{post, analytics})
```
去playground[试试](http://play.golang.org/p/vhE_JJUnsB)<br>

## 切分对象
这根组合结构体正好相反。就像我们对组合的结构体进行编码一样，我们也可以解码到分别使用JSON字段的结构体组合：
```go
json.Unmarshal([]byte(`{
  "url": "attila@attilaolah.eu",
  "title": "Attila's Blog",
  "visitors": 6,
  "page_views": 14
}`), &struct {
  *BlogPost
  *Analytics
}{&post, &analytics})
```
去playground[试试](http://play.golang.org/p/A3Fv9WW9A5)<br>

## 字段重命名
这个是移除字段和添加额外字段的组合使用：我们简单地移除一个字段再用一个不同的`json:`标记。这可以用指针的方式避免额外的内存分配。不过对于小的数据类型来说间接开销可能跟复制一份需要分配的内存相同，还要加上运行时开销。<br>
下面是重命名两个结构体字段的例子，通过指针间接地使用嵌套的结构体，整型则直接复制：
```go
type CacheItem struct {
    Key    string `json:"key"`
    MaxAge int    `json:"cacheAge"`
    Value  Value  `json:"cacheValue"`
}

json.Marshal(struct{
    *CacheItem

    // Omit bad keys
    OmitMaxAge omit `json:"cacheAge,omitempty"`
    OmitValue  omit `json:"cacheValue,omitempty"`

    // Add nice keys
    MaxAge int    `json:"max_age"`
    Value  *Value `json:"value"`
}{
    CacheItem: item,

    // Set the int by value:
    MaxAge: item.MaxAge,

    // Set the nested struct by reference, avoid making a copy:
    Value: &item.Value,
})
```
去playground[试试](https://play.golang.org/p/GWKO7u53WL)<br>
需要注意的是，这只有在需要重命名一个大结构体中一两个字段的时候实用。当需要重命名所有字段时，通常建一个全新的对象（如 serialiser）更简单（代码也更干净），避免实用结构体组合的方式。

相关文章：<br>
- [JSON decoding in Go](http://attilaolah.eu/2013/11/29/json-decoding-in-go/)