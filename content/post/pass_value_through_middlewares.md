---
date: 2019-04-28
title: "Actix-web中间件、handler间传递数据"
categories:
  - rust
tags:
  - rust
---

背景：中间件A产生的数据，需在后续的其他中间件、Handler中用到。<br>
例如: Handler中使用`Auth`中间件从token中解出的`user_id`。

<!--more-->
在golang的gin框架中很容易实现：

```go
func Auth(ctx *gin.Context){
	// ...
	ctx.Set("uid", uid)
}

func Foo(ctx *gin.Context){
	uid := ctx.GetInt("uid")
	// ...
}
```
Gin 的中间件与普通 handler 的类型相同，核心都是 `gin.Context`

Rust的`actix-web`中没有上下文的概念，其中间件形式为：
```rust
pub trait Middleware<S>: 'static {
    
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        Ok(Started::Done)
    }

    fn response(&self, req: &HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
        Ok(Response::Done(resp))
    }

    fn finish(&self, req: &HttpRequest<S>, resp: &HttpResponse) -> Finished {
        Finished::Done
    }
}
```
Handler则是：
```rust
pub fn index(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
	// ...
}
```
请求没有具体上下文的概念，只能另辟蹊径。

### 1. 状态

Actix-web 的状态由一个 `app` 所有路由共享，并不是以请求为单位的状态。
`HttpRequest::state()` 可以进行只读访问，修改需要通过内部可变性实现。
因此认为此场景不适合用状态实现。

### 2. Session

`SessionStorage`中间件提供会话管理解决方案，默认只有Cookie作为会话存储的实现。

### 3. Extensions

通过文档了解到`extensions`。<br>
用法如下：
```rust
impl<T> middleware::Middleware<T> for Auth {
    fn start(&self, req: &HttpRequest<T>) -> Result<Started> {
	// ...
        req.extensions_mut().insert(uid);
        Ok(Started::Done)
    }
}
pub fn index(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
	// ...
	let uid = req.extensions().get::<i64>();
	// ...
}
```

Extensions大致实现:
```rust
type AnyMap = HashMap<TypeId, Box<Any>, BuildHasherDefault<IdHasher>>;

#[derive(Default)]
/// A type map of request extensions.
pub struct Extensions {
    map: AnyMap,
}
```

### 结论

目前来看，`actix-web`的`extensions`最符合需求，但具体实现不同与go的`map[string]interface{}`,
而是`Map<TypeId, Box<Any>>`，因此无法直接存放相同类型的多个值。传递的值较多、类型有重复时，
需要创建相应的类型来“打包”。





