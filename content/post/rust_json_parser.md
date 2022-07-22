
---
title: "【Rust 表达能力探索】尝试实现一个简单的 JSON 解析器"
author: "liyiheng"
date: 2021-12-06T22:34:24+08:00
subtitle: "JSON 解析器"
image: ""
tags: [rust]
type: ""
---

TLDR：仅基于 Rust 标准库和基本的语言特性可以 200 行代码实现一个玩具级 JSON 解析器，其生产力不逊于常用的其他语言。

 
**本文仅仅是一个 POC，通过实现一个玩具级 JSON 解析器探索、感受 Rust 的表达能力。 JSON 部分没有严格按照 RFC4627 ，解析方式是通过递归实现，在 JSON 数据嵌套层级较多时有爆栈风险**

<!--more-->

## 一、JSON 结构
经过观察，JSON 值有以下几种类型（未参照 RFC，可能有误）：

- 数字
- 布尔值: true 和 false
- 字符串
- 对象
- 数组
- null

其中对象是一组键值对，键为字符串，值为 JSON；数组为一组 JSON 值。
Sum type （Rust 中的 enum ）可以很方便地在这个场景定义其数据类型

```rust
/// JsonValue _
#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    Str(String),
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}
```
## 二、API 定义

JSON 解析器，需要把 JSON 字符串解析为带有类型的值，即事先定义好的 `JsonValue`；

另外，解析出错时可以返回一些简单的错误信息：
```rust
#[derive(Debug)]
pub enum JsonError {
    EarlyEOF,
    Unexpected(char),
    InvalidNum(std::num::ParseFloatError),
    UnknownVal(String),
    EnclosedStr(String),
}
```

有了 JsonValue 和 JsonError， 只需要暴露一个最简单的 API：

```rust
pub fn parse(_data: &str) -> Result<JsonValue> {
    unimplemented!()
}
```

## 三、解析器的实现

这里采取的方案是通过从前到后读取原始数据进行递归解析，因此其核心是原始数据的迭代器：
```rust
struct Context<I: Iterator<Item = char>> {
    iter: Peekable<I>,
}
impl<I: Iterator<Item = char>> Context<I> {
// TODO
}
```
解析思路：

根据跳过数据的空白字符后，根据首个字母判断当前 JSON 值的类型，根据首字母得到的类型执行特定的解析逻辑：

1. “{” 为对象
2. “[” 为数组
3. 引号为字符串
4. “t”，“f”，“n” 分别是字面量 true，false 和 null
5. “0” 到 “9”，“-” 是数字

```rust
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.iter.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.iter.next();
        }
    }

    fn parse_val(&mut self) -> Result<JsonValue> {
        self.skip_whitespace();
        let c = self.iter.peek();
        if c.is_none() {
            return Err(JsonError::EarlyEOF);
        }
        let c = *c.unwrap();
        match c {
            '[' => self.parse_arr(),
            '{' => self.parse_obj(),
            't' | 'f' | 'n' => self.literal_val().and_then(|v| match v.as_str() {
                "true" => Ok(JsonValue::Boolean(true)),
                "false" => Ok(JsonValue::Boolean(false)),
                "null" => Ok(JsonValue::Null),
                _ => Err(JsonError::UnknownVal(v)),
            }),
            '-' | '0'..='9' => self
                .literal_val()?
                .parse::<f64>()
                .map_err(JsonError::InvalidNum)
                .map(JsonValue::Number),
            '"' => self.parse_str().map(JsonValue::Str),
            _ => Err(JsonError::Unexpected(c)),
        }
    }
```
接下来，只需要实现 `parse_obj`, `parse_arr` 和字面量的读取逻辑。

字面量的读取逻辑很简单：逐个读取字符，遇到 "," 、"}"、 "]" 就停止；数组的解析是通过现有逻辑逐个解析其中元素，直到 "]" 出现；对象的解析相对复杂一些：逐个解析键值对，遇到 "}" 后停止，其中值的解析可以直接复用现有逻辑，键的解析需要进一步实现。具体代码如下：

```rust
    fn parse_str(&mut self) -> Result<String> {
        self.skip_whitespace();
        self.iter.next(); // 跳过开头的引号
        let mut s = String::new();
        for c in self.iter.by_ref() {
            if c != '"' {
                s.push(c);
            } else {
                return Ok(s);
            }
        }
        Err(JsonError::EnclosedStr(s))
    }

    // true, false, null, number
    fn literal_val(&mut self) -> Result<String> {
        self.skip_whitespace();
        let mut val = String::new();
        while let Some(&c) = self.iter.peek() {
            if c == ',' || c == '}' || c == ']' {
                return Ok(val);
            } else {
                val.push(c);
                self.iter.next();
            }
        }
        Ok(val.trim_end().to_string())
    }

    fn parse_field(&mut self) -> Result<(String, JsonValue)> {
        let k = self.parse_str()?;
        self.skip_whitespace();
        if let Some(c) = self.iter.next() {
            if c != ':' {
                return Err(JsonError::Unexpected(c));
            }
        } else {
            return Err(JsonError::EarlyEOF);
        }
        let v = self.parse_val()?;
        Ok((k, v))
    }

    fn parse_obj(&mut self) -> Result<JsonValue> {
        self.iter.next(); // 跳过 "{"
        self.skip_whitespace();
        let mut result = HashMap::new();
        loop {
            if let Some(&c) = self.iter.peek() {
                if c == '}' {
                    // 没有任务字段的 JSON 对象
                    self.iter.next();
                    return Ok(JsonValue::Object(result));
                }
            }
            // 逐个解析字段
            let (k, v) = self.parse_field()?;
            result.insert(k, v);
            self.skip_whitespace();
            if let Some(&c) = self.iter.peek() {
                match c {
                    '}' => {
                        // 跳过 "}" 并结束对象的解析
                        self.iter.next();
                        break;
                    }
                    ',' => {
                        // 跳过 "," 并进行下一个字段的解析
                        self.iter.next();
                    }
                    _ => {
                        return Err(JsonError::Unexpected(c));
                    }
                };
            }
        }
        Ok(JsonValue::Object(result))
    }

    fn parse_arr(&mut self) -> Result<JsonValue> {
        self.iter.next(); // 跳过 "["
        self.skip_whitespace();
        let mut result = Vec::new();
        loop {
            if let Some(&c) = self.iter.peek() {
                if c == ']' {
                    // 空数组的场景
                    self.iter.next();
                    return Ok(JsonValue::Array(result));
                }
            }
            let v = self.parse_val()?;
            result.push(v);
            self.skip_whitespace();
            let c = self.iter.next();
            if c.is_none() {
                return Err(JsonError::EarlyEOF);
            }
            let c = c.unwrap();
            match c {
                ']' => break,
                ',' => continue,
                _ => return Err(JsonError::Unexpected(c)),
            }
        }
        Ok(JsonValue::Array(result))
    }
```

最后，补充对外接口实现，以及一个简单场景的测试用例：

```rust
pub fn parse(data: &str) -> Result<JsonValue> {
    let iter = data.chars().peekable();
    Context { iter }.parse_val()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_case() {
        let data = r#"
        {"a":null,"b": true,
            "c": 123,
            "d": "hello",
            "e": {"aaa": []},
            "f": [null,true,123,{},false]
        }"#;
        let v = parse(data).unwrap();
        match v {
            JsonValue::Object(map) => {
                assert_eq!(*map.get("c").unwrap(), JsonValue::Number(123.0));
            }
            _ => {
                assert!(false, "It should be an object");
            }
        }
    }
}
```
一个稍微完善点的例子：

```rust
fn main() {
    let data = r#"
    {
        "姓名": "马保国",
        "年龄": 69,
        "技能": ["混元功法", "闪电五连鞭"],
        "是否男性": true,
        "是否女性": false,
        "Github": null,
        "其他属性":{
            "attr1": "value1",
            "attr2": "哈哈哈"
        }
    }
        "#;
    let v = parser::parse(data).unwrap();
    println!("Original: {}", data);
    println!("Parsed:");
    if let parser::JsonValue::Object(map) = v {
        for (k, v) in map.iter() {
            println!("\t{} => {:?}", k, v);
        }
    }
}
mod parser;

```
输出结果：

```text
Original: 
    {
        "姓名": "马保国",
        "年龄": 69,
        "技能": ["混元功法", "闪电五连鞭"],
        "是否男性": true,
        "是否女性": false,
        "Github": null,
        "其他属性":{
            "attr1": "value1",
            "attr2": "哈哈哈"
        }
    }
        
Parsed:
	是否男性 => Boolean(true)
	是否女性 => Boolean(false)
	Github => Null
	其他属性 => Object({"attr2": Str("哈哈哈"), "attr1": Str("value1")})
	技能 => Array([Str("混元功法"), Str("闪电五连鞭")])
	年龄 => Number(69.0)
	姓名 => Str("马保国")

```
