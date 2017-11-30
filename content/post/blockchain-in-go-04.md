---
date: 2017-10-10
title: "[译]用go进行区块链开发4:交易1"
draft: true 
categories:
  - golang
  - blockchain
tags:
  - golang
  - blockchain
---

>原文地址 https://jeiwan.cc/posts/building-blockchain-in-go-part-4/

### 简介
交易是比特币的心脏，并且为了让交易创建后无法更改，以安全可靠的方式存储交易是区块链的唯一目标。今天我们开始实现交易。由于这是一个比较大的话题，我把它分为两部分：在这一部分我们实现交易的基本机制，在第二部分完成细节。<br><br>
另外，由于代码变化较大，就不再一一描述。你可以在[这里](https://github.com/Jeiwan/blockchain_go/compare/part_3...part_4#files_bucket)看到全部改动

<!--more-->
### There is no spoon
如果你曾开发过web应用，为了实现支付你可能会在数据库中创建这些表：`accounts`和`transactions`。账户存储用户个人信息和余额，交易存储资金从一个账户转移到另一个账户的相关信息。在比特币中，支付用完全不同的方式实现：<br>

1. 没有账户
2. 没有余额
3. 没有地址
4. 没有钱币
5. 没有发送方和接收方

由于区块链是公共开放的数据库，我们不想保存钱包所有者的敏感信息。钱币不收集在账户中。交易不把钱从一个地址转到另一个。没有保存余额的字段或属性。只有交易。然而交易内部是什么呢？<br><br>

### 比特币交易
交易是输入和输出的组合：
```go
type Transaction struct {
	ID   []byte
	Vin  []TXInput
	Vout []TXOutput
}
```
---
