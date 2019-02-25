---
date: 2017-09-28
title: "[译]用go进行区块链开发3:持久化与CLI"
draft: true
categories:
  - golang
  - blockchain
tags:
  - golang
  - blockchain
---

>原文地址 https://jeiwan.cc/posts/building-blockchain-in-go-part-3/

### 简介
[目前为止](https://xanthusl.github.io/blog/2017/09/%E8%AF%91%E7%94%A8go%E8%BF%9B%E8%A1%8C%E5%8C%BA%E5%9D%97%E9%93%BE%E5%BC%80%E5%8F%911%E5%9F%BA%E6%9C%AC%E5%8E%9F%E5%9E%8B/)，我们做了带有工作量证明系统的区块链，因此它是可以挖矿的。我们的实现与全功能的区块链越来越接近了，但还缺乏一些重要特性。今天我们将把区块链存到一个数据库中，并在那之后做个简单的命令行工具来对它进行操作。本质上，区块链是分布式数据库。我们暂时忽略“分布式”部分专注于“数据库”部分。

<!--more-->
### 数据库选择

我们当前的实现中还没有数据库；而是在每次执行程序的时候创建区块链并保存在内存中。我们无法复用一个区块链，也不能与他人分享，因此需要把它存在磁盘上。<br><br>
我们需要哪个数据库？事实上任意一个都可以。在[比特币白皮书](https://bitcoin.org/bitcoin.pdf)中没有提到具体数据库的使用，因此使用什么数据库取决于开发者。现在普遍的比特币实现参考是Satoshi Nakamoto最初发布的[Bitcoin Core](https://github.com/bitcoin/bitcoin)，它用的[LevelDB](https://github.com/google/leveldb)(尽管2012年才发布)。我们将要用的是...<br><br>

### BoltDB
因为：<br>

1. 它是极简的并易于使用。<br>
2. 用go实现。
3. 不需要运行服务器。
4. 允许创建我们想要的数据结构。

以下摘自BoltDB在[Github上的README](https://github.com/boltdb/bolt)

>Bolt is a pure Go key/value store inspired by Howard Chu’s LMDB project. The goal of the project is to provide a simple, fast, and reliable database for projects that don’t require a full database server such as Postgres or MySQL.<br><br>
Since Bolt is meant to be used as such a low-level piece of functionality, simplicity is key. The API will be small and only focus on getting values and setting values. That’s it.

听起来非常符合我们的需求。花一分钟来仔细看一下。<br><br>
BoltDB是一个键值对存储的数据库，这意味着它没有SQL RDBMS（如MySQL,PostgreSql等）中的表，没有行，没有列。取而代之的是，数据以键值对（类似golang中的map）的形式存储。键值对存放在桶中（`bucket`）,意在给类似的键值对分组（跟RDBMS中的表类似）。因此，获取一个值需要知道它的key以及所在的桶。<br><br>
关于BoltDB的一个重点是没有数据类型：键和值都是字节数组。由于我们要存储go的结构体（尤其是`Block`），需要对它进行序列化，例如实现一个把go结构体转换成字节数组并能把它从字节数据恢复的机制。对于这个我们将用[encoding/gob](https://golang.org/pkg/encoding/gob/)，不过JSON、XML、ProtocolBuffers等都是可以的。我们用`encoding/gob`是因为它简单并且是go标准库中的。<br><br>

### 数据库结构
在我们开始实现持久化逻辑之前，先定下来如何在数据库中存储数据。我们将参考Bitcoin Core的方式。<br><br>
简单地说，Bitcoin Core用了两个“桶”来存数据：<br>

1. `block` 中存放描述链中所有区块的元数据。 
2. `chainstate` 存放链的状态，即当前所有未完成交易的输出及一些元数据。

 
另外，区块存放在磁盘上的独立文件中。这么做是出于性能目的：读取单个区块不需要把所有（或多个）加载到内存中。我们将不实现这个。<br><br>
`block`中有如下键值对：<br>

1. 'b' + 32字节的区块哈希 -> 区块索引记录
2. 'f' + 4字节的文件号 -> 文件信息记录
3. 'l' + 4字节的文件号 -> 最后一个区块用的文件号
4. 'R' + 1字节的布尔值 -> 是否正在重建索引
5. 'F' + 1字节的标识名 + 标识名字符串 -> 1字节的布尔值：可以置为开和关的各种标识
6. 't' + 32字节的交易哈希 -> 交易索引记录

在`chainstate`中的键值对如下：<br>

1. 'c' + 32字节的交易哈希 -> 该交易未处理完的输出记录
2. 'B' -> 32字节的区块哈希:此哈希取决于描述未处理完交易输出的数据库

（详细信息在[这里](https://en.bitcoin.it/wiki/Bitcoin_Core_0.11_(ch_2):_Data_Storage)）

由于我们还没有交易，现在只用一个`blocks`桶。另外，如刚才所说，我们将把整个数据库存到单个文件中而不是把每个区块存到独立文件中。因此与文件号相关的内容都不需要了。现在我们用的`key->value`对是这些：<br>

1. 32字节的区块哈希 -> 区块结构体(序列化后的)
2. 'l' -> 链中最后一个区块的哈希值

这就是我们实现持久化机制需要的所有东西。

### 序列化
如之前所说，在BoltDB中只能用字节数组，我们要往数据库中存储`Block`结构体。我们采用[encoding/gob](https://golang.org/pkg/encoding/gob/)来对结构体进行序列化操作。<br><br>
我们来实现`Block`的`Serialize`方法（为了简洁或略掉了错误处理）：
```go
func (b *Block) Serialize() []byte {
	var result bytes.Buffer
	encoder := gob.NewEncoder(&result)

	err := encoder.Encode(b)

	return result.Bytes()
}
```
这段代码很直观：先声明一个缓冲区用来存放序列化后的数据；然后初始化一个`gob`编码器并对区块进行编码；结果以字节数组的形式返回。<br><br>
接下来，我们需要一个接收字节数组并返回一个`Block`的反序列化方法。这不是方法而是一个独立的函数：<br><br>
```go
func DeserializeBlock(d []byte) *Block {
	var block Block

	decoder := gob.NewDecoder(bytes.NewReader(d))
	err := decoder.Decode(&block)

	return &block
}
```
这就是序列化部分。

### 持久化
我们从`NewBlockchain`方法开始看。目前它是创建一个`Blockchain`实例并加上创世区块。我们想要的是这样：<br><br>

1. 打开一个数据库文件。
2. 检查里面是否存了区块链。
3. 如果里面有：
	1. 创建一个新的`Blockchain`实例。
	2. 把`Blockchain`实例的末端设为数据库中存储的最后一个区块
4. 如果没有区块链：
	1. 创建创世区块
	2. 存入数据库
	3. 把创世区块的哈希存作为最后区块哈希存储
	4. 创建一个尾部指向创世区块的`Blockchain`实例

代码如下：
```go
func NewBlockchain() *Blockchain {
	var tip []byte
	db, err := bolt.Open(dbFile, 0600, nil)

	err = db.Update(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte(blocksBucket))

		if b == nil {
			genesis := NewGenesisBlock()
			b, err := tx.CreateBucket([]byte(blocksBucket))
			err = b.Put(genesis.Hash, genesis.Serialize())
			err = b.Put([]byte("l"), genesis.Hash)
			tip = genesis.Hash
		} else {
			tip = b.Get([]byte("l"))
		}

		return nil
	})

	bc := Blockchain{tip, db}

	return &bc
}

```
我们来重新审视一下这段代码。
```go
db, err := bolt.Open(dbFile, 0600, nil)
```
这是打开BoltDB文件的标准方式。值得注意的是，文件不存在时它不会返回错误。
```go
err = db.Update(func(tx *bolt.Tx) error {
...
})
```
在BoltDB中，对数据库的操作在事务中进行。事务分两种：只读的和读写的。因为我们希望吧创世区块存入数据库，这里我们开启了一个读写事务（`db.Update(...)`）。
```go
b := tx.Bucket([]byte(blocksBucket))

if b == nil {
	genesis := NewGenesisBlock()
	b, err := tx.CreateBucket([]byte(blocksBucket))
	err = b.Put(genesis.Hash, genesis.Serialize())
	err = b.Put([]byte("l"), genesis.Hash)
	tip = genesis.Hash
} else {
	tip = b.Get([]byte("l"))
}
```
上面的代码是这个函数的核心。我们获取到存放区块的桶：如果存在就读取键`l`；如果不存在就生成创世区块，创建桶，并把创世区块存进去，然后更新存放链中最后区块哈希值的`l`键。<br><br>
另外，注意我们创建`Blockchain`的新方式：
```go
bc := Blockchain{tip, db}
```
我们不再把所有区块存进去了，而是只存已保存区块链的末端。另外还保存了一个数据库链接，因为我们希望一旦打开它就让它随着程序的运行一直处于打开的状态。因此现在的`Blockchain`结构体是这样的：
```go
type Blockchain struct {
	tip []byte
	db  *bolt.DB
}
```
下一个要更新的是`AddBlock`方法：现在添加区块不再像往数组里添加元素那么简单了。从现在起要把区块存到数据库：
```go
func (bc *Blockchain) AddBlock(data string) {
	var lastHash []byte

	err := bc.db.View(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte(blocksBucket))
		lastHash = b.Get([]byte("l"))

		return nil
	})

	newBlock := NewBlock(data, lastHash)

	err = bc.db.Update(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte(blocksBucket))
		err := b.Put(newBlock.Hash, newBlock.Serialize())
		err = b.Put([]byte("l"), newBlock.Hash)
		bc.tip = newBlock.Hash

		return nil
	})
}
```
一点一点地看一下：
```go
err := bc.db.View(func(tx *bolt.Tx) error {
	b := tx.Bucket([]byte(blocksBucket))
	lastHash = b.Get([]byte("l"))

	return nil
})
```
这是另一种类型(只读)的BoltDB事务。我们存数据库中获取最后区块的哈希值用来挖掘一个新的区块哈希。
```go
newBlock := NewBlock(data, lastHash)
b := tx.Bucket([]byte(blocksBucket))
err := b.Put(newBlock.Hash, newBlock.Serialize())
err = b.Put([]byte("l"), newBlock.Hash)
bc.tip = newBlock.Hash
```
挖到新区块后，我们把序列化后的数据存放到数据库并更新`l`键存的哈希值。<br><br>
完成！并不难，不是吗

### 检查区块链
现在所有区块都存在了数据库中，因此我们可以重新打开一条区块链并给它添加新区块。但是实现这个之后我们失去了一个不错的特性：由于我们不再把区块存到数组中，不能打印区块信息了。我们来修复这个瑕疵!<br><br>
BoltDB允许迭代一个桶中的所有键，但是键是以字节顺序存储，我们想以区块在区块链中的顺序来打印。另外，由于我们不想把所有区块加载到内存（我们的区块链数据库可能很大...我们假装它很大），我们将逐个读取区块。为此，需要一个区块链迭代器：
```go
type BlockchainIterator struct {
	currentHash []byte
	db          *bolt.DB
}
```
我们想迭代区块链中所有区块时会创建一个迭代器，它保存当前迭代到的区块哈希和一个数据库链接。鉴于后者，一个迭代器在逻辑上依附于一个区块链（保存一个数据库链接的`Blockchain`实例），因此，它在一个`Blockchain`的方法中创建:
```go
func (bc *Blockchain) Iterator() *BlockchainIterator {
	bci := &BlockchainIterator{bc.tip, bc.db}

	return bci
}
```
注意，迭代器初始指向区块链的末端，因此区块是从上而下、从新到旧的顺序获取。事实上**选择末端意味着为一条区块链“投票”**。一条区块链可以有多个分支，其中最长的认为是主分支。取得一个末端后（可以是区块链中任意一个区块）我们可以重建整条区块链并得到它的长度以及建造它需要的工作。这也意味着一个末端是一条区块链的标识符。<br><br>
`BlockchainIterator`只做一件事：返回区块链上的下一个区块。
```go
func (i *BlockchainIterator) Next() *Block {
	var block *Block
	err := i.db.View(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte(blocksBucket))
		encodedBlock := b.Get(i.currentHash)
		block = DeserializeBlock(encodedBlock)
		return nil
	})
	i.currentHash = block.PrevBlockHash
	return block
}
```
以上就是数据库部分！

### CLI
到现在为止我们的实现没有提供任何与程序交互的接口：我们只是在`main`函数简单地执行了`NewBlockchain`、`bc.AddBlock`。是时候改进这个了。我们想要如下命令：
```
blockchain_go addblock "Pay 0.031337 for a coffee"
blockchain_go printchain
```
所有命令行相关的操作将由`CLI`结构体处理：
```go
type CLI struct {
	bc *Blockchain
}
```
它的“入口”是`Run`函数：
```go
func (cli *CLI) Run() {
	cli.validateArgs()

	addBlockCmd := flag.NewFlagSet("addblock", flag.ExitOnError)
	printChainCmd := flag.NewFlagSet("printchain", flag.ExitOnError)

	addBlockData := addBlockCmd.String("data", "", "Block data")

	switch os.Args[1] {
	case "addblock":
		err := addBlockCmd.Parse(os.Args[2:])
	case "printchain":
		err := printChainCmd.Parse(os.Args[2:])
	default:
		cli.printUsage()
		os.Exit(1)
	}

	if addBlockCmd.Parsed() {
		if *addBlockData == "" {
			addBlockCmd.Usage()
			os.Exit(1)
		}
		cli.addBlock(*addBlockData)
	}

	if printChainCmd.Parsed() {
		cli.printChain()
	}
}
```
我们用标准库中的[flag](https://golang.org/pkg/flag/)包来解析命令行参数：
```go
addBlockCmd := flag.NewFlagSet("addblock", flag.ExitOnError)
printChainCmd := flag.NewFlagSet("printchain", flag.ExitOnError)
addBlockData := addBlockCmd.String("data", "", "Block data")
```
首先，键两条子命令，`addblock`和`printchain`，然后为前者添加`-data`标识。`printchain`没有标识。
```go
switch os.Args[1] {
case "addblock":
	err := addBlockCmd.Parse(os.Args[2:])
case "printchain":
	err := printChainCmd.Parse(os.Args[2:])
default:
	cli.printUsage()
	os.Exit(1)
}
```
接下来检查用户提供的命令并解析相关的`flag`子命令。
```go
if addBlockCmd.Parsed() {
	if *addBlockData == "" {
		addBlockCmd.Usage()
		os.Exit(1)
	}
	cli.addBlock(*addBlockData)
}

if printChainCmd.Parsed() {
	cli.printChain()
}
```
然后检查解析的是哪个子命令并执行相关的函数。
```go
func (cli *CLI) addBlock(data string) {
	cli.bc.AddBlock(data)
	fmt.Println("Success!")
}

func (cli *CLI) printChain() {
	bci := cli.bc.Iterator()

	for {
		block := bci.Next()

		fmt.Printf("Prev. hash: %x\n", block.PrevBlockHash)
		fmt.Printf("Data: %s\n", block.Data)
		fmt.Printf("Hash: %x\n", block.Hash)
		pow := NewProofOfWork(block)
		fmt.Printf("PoW: %s\n", strconv.FormatBool(pow.Validate()))
		fmt.Println()

		if len(block.PrevBlockHash) == 0 {
			break
		}
	}
}
```
这段代码跟我们之前写的很像。唯一不同是现在我们用一个`BlockchainIterator`来迭代区块链中的区块。<br><br>
另外不要忘了照着修改`main`函数：
```go
func main() {
	bc := NewBlockchain()
	defer bc.db.Close()

	cli := CLI{bc}
	cli.Run()
}
```
值得一提的是，无论命令行参数是什么都会创建一个新的`Blockchain`。<br><br>
就这么多了！检查一下是否一切正常：
```sh
$ blockchain_go printchain
No existing blockchain found. Creating a new one...
Mining the block containing "Genesis Block"
000000edc4a82659cebf087adee1ea353bd57fcd59927662cd5ff1c4f618109b

Prev. hash:
Data: Genesis Block
Hash: 000000edc4a82659cebf087adee1ea353bd57fcd59927662cd5ff1c4f618109b
PoW: true

$ blockchain_go addblock -data "Send 1 BTC to Ivan"
Mining the block containing "Send 1 BTC to Ivan"
000000d7b0c76e1001cdc1fc866b95a481d23f3027d86901eaeb77ae6d002b13

Success!

$ blockchain_go addblock -data "Pay 0.31337 BTC for a coffee"
Mining the block containing "Pay 0.31337 BTC for a coffee"
000000aa0748da7367dec6b9de5027f4fae0963df89ff39d8f20fd7299307148

Success!

$ blockchain_go printchain
Prev. hash: 000000d7b0c76e1001cdc1fc866b95a481d23f3027d86901eaeb77ae6d002b13
Data: Pay 0.31337 BTC for a coffee
Hash: 000000aa0748da7367dec6b9de5027f4fae0963df89ff39d8f20fd7299307148
PoW: true

Prev. hash: 000000edc4a82659cebf087adee1ea353bd57fcd59927662cd5ff1c4f618109b
Data: Send 1 BTC to Ivan
Hash: 000000d7b0c76e1001cdc1fc866b95a481d23f3027d86901eaeb77ae6d002b13
PoW: true

Prev. hash:
Data: Genesis Block
Hash: 000000edc4a82659cebf087adee1ea353bd57fcd59927662cd5ff1c4f618109b
PoW: true
```
（此处有开啤酒的声音）

### 结语
下次我们将实现地址、钱包、交易（可能）。所以请继续关注！

链接：

1. [完整源码](https://github.com/Jeiwan/blockchain_go/tree/part_3)
2. [Bitcoin Core数据存储](https://en.bitcoin.it/wiki/Bitcoin_Core_0.11_(ch_2):_Data_Storage)
3. [boltdb](https://github.com/boltdb/bolt)
4. [encoding/gob](https://golang.org/pkg/encoding/gob/)
5. [flag](https://golang.org/pkg/flag/)
