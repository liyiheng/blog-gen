---
date: 2017-09-21
title: "[译]用go进行区块链开发1:基本原型"
draft: false
categories:
  - golang
  - blockchain
tags:
  - golang
  - blockchain
#thumbnailImagePosition: left
---

>原文地址 https://jeiwan.cc/posts/building-blockchain-in-go-part-1/

### 简介
区块链是21世纪最有革命性的技术之一。尽管人们还没有充分意识到它的潜力,区块链技术正在走向成熟。区块链本质上是一种分布式数据库。它的特殊之处在于它不是私有数据库，而是公共的，例如，每个用户或整体或局部地有一份它的副本。一条新纪录只有得到数据库其他持有者的许可才能添加到数据库。另外，区块链使密码货币和智能合约成为可能。

<!--more-->
在这一系列文章中我们将基于简单的区块链实现建一种简化的密码货币。

### 区块
我们从“区块链”的“区块”部分着手。区块链中有价值的信息存放在区块中。例如，比特币的区块中存放的是交易信息，这是所有密码货币的本质。此外，区块还含有版本号、当前时间戳、上一区块散列值等技术信息。<br><br>
这篇文章中我们不会通过实现区块来详细阐述区块链或者比特币，而是用一个只含有核心信息的简化版。它大概是这样：
```go
type Block struct {
	Timestamp     int64
	Data          []byte
	PrevBlockHash []byte
	Hash          []byte
}
```
`TimeStamp`是当前的时间戳（该区块何时创建），`Data`是实际上在区块中保存的有价值的信息。`PrevBlockHash`存放的是上一区块的散列值，`Hash`是该区块的散列值。在比特币规范中，`TimeStamp`、`PrevBlockHash`和`Hash`存放在独立的区块头信息中，交易（我们的`Data`）也是独立的数据结构。我们为了简化把他们放到了一起。<br><br>
如何计算散列值？散列值的计算方式是保证区块链安全的重要特性。计算哈希是难度很大的运算操作，就算在很快的设备上也要花一些时间（这就是人们买强劲GPU进行比特币挖矿的原因）。这种刻意的架构设计使添加新的区块很难，因此防止区块添加后被篡改。我们将在之后的文章中讨论并实现这种机制。<br><br>
现在，我们仅仅是取区块的字段并把他们连接起来，然后计算连接后数据的SHA-256散列：
```go
func (b *Block) SetHash() {
	timestamp := []byte(strconv.FormatInt(b.Timestamp, 10))
	headers := bytes.Join([][]byte{b.PrevBlockHash, b.Data, timestamp}, []byte{})
	hash := sha256.Sum256(headers)

	b.Hash = hash[:]
}
```
接下来，按照Golang的惯例，实现一个函数简化区块的创建：
```go
func NewBlock(data string, prevBlockHash []byte) *Block {
	block := &Block{time.Now().Unix(), []byte(data), prevBlockHash, []byte{}}
	block.SetHash()
	return block
}
```
以上就是简化的区块。

### 区块链
现在我们来实现区块链。区块链本质上是有特定结构的数据库：有序的、向后链表。这意味着区块按照插入顺序排列并且每个区块连接着上一区块。通过这种结构可以迅速地获取到最近一个区块，也可以通过哈希值（高效地）获取一个区块。<br><br>
这种结构在golang中可以通过一个数组（译者注：作者用的是切片，我平时也常把切片叫成数组，毕竟有些编程语言没有切片）和一个map实现：数组中保存按顺序存放哈希值(Go中数组是有序的)，map中保存`hash → block`的键值对（map是无序的）。不过在我们的区块链原型中只用一个数组，因为目前还不需要通过哈希获取区块。
```go
type Blockchain struct {
	blocks []*Block
}
```
这是我们第一条区块链，从未想过有这么简单😉<br><br>
现在我们让它可以添加区块：
```go
func (bc *Blockchain) AddBlock(data string) {
	prevBlock := bc.blocks[len(bc.blocks)-1]
	newBlock := NewBlock(data, prevBlock.Hash)
	bc.blocks = append(bc.blocks, newBlock)
}
```
这就好了。。。吗？<br><br>
添加一个新的区块时要用到一个已有的区块，但是现在我们的区块链中是没有区块的！因此任何区块链中都至少有一个区块，第一个区块被称为创世区块（`genesis block`）。实现一个创建创世区块的函数：
```go
func NewGenesisBlock() *Block {
	return NewBlock("Genesis Block", []byte{})
}
```
现在我们可以实现一个用创世区块创建区块链的函数了：
```go
func NewBlockchain() *Blockchain {
	return &Blockchain{[]*Block{NewGenesisBlock()}}
}
```
检查一下区块链能否正常工作：
```go
func main() {
	bc := NewBlockchain()

	bc.AddBlock("Send 1 BTC to Ivan")
	bc.AddBlock("Send 2 more BTC to Ivan")

	for _, block := range bc.blocks {
		fmt.Printf("Prev. hash: %x\n", block.PrevBlockHash)
		fmt.Printf("Data: %s\n", block.Data)
		fmt.Printf("Hash: %x\n", block.Hash)
		fmt.Println()
	}
}
```
输出：
```
Prev. hash:
Data: Genesis Block
Hash: aff955a50dc6cd2abfe81b8849eab15f99ed1dc333d38487024223b5fe0f1168

Prev. hash: aff955a50dc6cd2abfe81b8849eab15f99ed1dc333d38487024223b5fe0f1168
Data: Send 1 BTC to Ivan
Hash: d75ce22a840abb9b4e8fc3b60767c4ba3f46a0432d3ea15b71aef9fde6a314e1

Prev. hash: d75ce22a840abb9b4e8fc3b60767c4ba3f46a0432d3ea15b71aef9fde6a314e1
Data: Send 2 more BTC to Ivan
Hash: 561237522bb7fcfbccbc6fe0e98bbbde7427ffe01c6fb223f7562288ca2295d1
```
就这么多了

### 结语
我们建了一个非常简化的区块链原型：它只是一个区块数组，其中每个区块都有前一区块的连接。不过实际上的区块链是十分复杂的。在我们的区块链中添加新区块简单而迅速，但真实的区块链添加区块需要一些工作：得到添加区块的权限之前必须执行一些繁重的运算（这个机制被称为工作量证明,`Proof-of-Work`）。另外，区块链是没有单一决策者的分布式数据库。一个新区块必须被网络中别的参与者确认并承认（这个机制称为共识,`consensus`）。现在你的区块链中还没有交易。<br><br>
我们将在接下来的文章中覆盖这些特性。


链接：<br>
1. 完整源码：https://github.com/Jeiwan/blockchain_go/tree/part_1<br>
2. 区块哈希算法：https://en.bitcoin.it/wiki/Block_hashing_algorithm
