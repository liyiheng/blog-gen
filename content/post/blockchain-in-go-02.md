---
date: 2017-09-22
title: "[译]用go进行区块链开发2:工作量证明"
draft: false
categories:
  - golang
  - blockchain
tags:
  - golang
  - blockchain
thumbnailImagePosition: left
---

>原文地址 https://jeiwan.cc/posts/building-blockchain-in-go-part-2/

### 简介
[上一篇文章](https://xanthusl.github.io/blog/2017/09/%E8%AF%91%E7%94%A8go%E8%BF%9B%E8%A1%8C%E5%8C%BA%E5%9D%97%E9%93%BE%E5%BC%80%E5%8F%911%E5%9F%BA%E6%9C%AC%E5%8E%9F%E5%9E%8B/)我们根据区块链的本质做了一个简单的数据结构，并且实现了像它添加有连锁关系的区块：每一区块连接着上一区块。不过我们的区块链实现存在严重问题：向链上添加区块简单又便宜。难于添加新区块是区块链和比特币的一个重要特点。<br>
今天我们来解决这个问题

<!--more-->
### 工作量证明(`Proof-of-Work`) 
如果要往区块链中存放数据，必须执行一些高难度的工作，这是区块链的一个核心观点。正是这种高难度工作使区块链安全一致。另外，这些高难度工作会得到奖励（这就是挖矿获取比特币的原理）。<br><br>
这个机制跟现实生活中的很像：一个人必须努力工作获得报酬以维持生计。区块链中，一些想通过添加新区块以获得报酬的参与者（矿工）会维持网络。他们的工作结果就是，一个区块以不影响整个区块链数据库稳定性的安全方式被合并到区块链中。值得注意的是，完成这项工作的人需要为之证明。<br><br>
这个“努力工作并证明”的机制被称为工作量证明。它难就难在需要超强的运算能力：就算是高性能计算机也无法快速完成。再者，为了保持每小时6个新区块的产生速度这项工作的难度会时不时地增加。比特币中，这项工作的目标是找到满足一些要求的区块的哈希值。这个哈希值就是工作量的证明。因此，寻找一个证明就是实际上的工作。<br><br>
最后一点需要注意的是，工作量证明算法必须符合一个要求：工作难度大，但验证简单。一个证明通常会交给其他人，因此对他们来说不宜花太多时间来验证。

### 哈希
我们将在这一部分讨论哈希。如果你熟悉这个概念，可以跳过这部分。<br><br>
哈希是为特定数据取哈希值的过程。一个哈希值是用于计算它的数据的唯一表现。哈希函数是用任意长度的数据生成指定长度的哈希值。一下是哈希的部分特性：<br>
1. 通过哈希值不能得到原始数据。因此，哈希不是加密
2. 特定的数据只能有一个哈希值并且哈希值是唯一的
3. 改变数据的任意一个字节，它的哈希值就截然不同
```
"I like donutes"
	↓
SHA256(...)
	↓
f80867f6efd4484c23b0e7184e53fe4af6ab49b97f5293fcd50d5b2bfa73a4d0
```

哈希函数广泛应用于数据一致性校验。一些软件在软件包中提供发布版本的校验值。下载好一个文件后就可以用哈希函数取它的哈希值并和软件开发者提供的哈希值进行比对。<br><br>
在区块链中，哈希用来保证区块的一致性。哈希算法的输入数据包含了上一区块的哈希值，因此不可能（或者说难度极大）篡改链中的区块：改变区块就要重新计算它的以及它后面的所有区块的哈希值。<br><br>

### 哈希现金（`Hashcash`）
比特币用了[哈希现金](https://en.wikipedia.org/wiki/Hashcash)，这是一个最初用来阻止垃圾邮件的工作量证明算法。可以把它分为一下几步：<br>

1. 获取一部分公开数据（在电子邮件中，是收件人的地址；在比特币中，是区块的头） 
2. 为它添加一个计数器。计数器从0开始计数
3. 为`数据+计数器`的组合体取哈希值
4. 检查哈希值是否满足条件
	1. 满足，完成了。
	2. 不满足，计数器计数加一，重复第3步和第4步.

 
因此这是一个暴力的算法：改变计数，计算新的哈希值，检查，增加计数，计算哈希等等。这便是它计算昂贵的原因。<br><br>
现在我们仔细看一下哈希需要满足的条件。在原版哈希现金实现中的条件听起来像“哈希值前20个位（bit）必须是0”。比特币中，这个需求会时不时地调整，因为在设计上，不管运算能力如何提升、越来越多的矿工加入，必须每10分钟产生一个区块<br><br>
为了演示这个算法，我用上一个例子中的数据（“I like donuts”）发现了一个开头3个字节为0的哈希值：<br>
```
"I like donutesca07ca"
	↓
SHA256(...)
	↓
0000002f7c1fe31cb82acdc082cfec47620b7e4ab94f2bf9e096c436fc8cee06	
```
`ca07ca`是计数器值的十六进制表示，在十进制中是`13240266`。

### 实现
那么，我们已经完成了理论部分，现在开始写代码。首先定义挖矿难度：
```go
const targetBits = 24
```
在比特币中，“目标位”是存放区块产生时挖掘难度的区块头。我们暂时不实现目标调整算法，因此把挖矿难度定义为一个共用的常量。<br><br>
24是随便选的一个数字，我们的目的是要有一个占用内存少于256个位的目标。我们希望这个差别足够大，但也不能太大，因为差别越大就越难找到一个合适的哈希值。<br>
```go
type ProofOfWork struct {
	block  *Block
	target *big.Int
}

func NewProofOfWork(b *Block) *ProofOfWork {
	target := big.NewInt(1)
	target.Lsh(target, uint(256-targetBits))

	pow := &ProofOfWork{b, target}

	return pow
}
```
这里创建了一个持有区块指针和目标指针的`ProofOfWork`结构体。“目标”是上一部分中说到的需要满足的要求的另一个名字。我们用了一个[大](https://golang.org/pkg/math/big/)整型，因为要和目标对比哈希：把一个哈希值转换成大整型并检查它是否比目标小。<br><br>
在`NewProofOfWork`函数中，我们用数值1初始化一个`big.Int`并把它左移`256 - targetBits`位。256是SHA-256哈希值所占的位数，而且我们打算用的哈希算法就是SHA-256。目标的十六进制表示是：
```
0x10000000000000000000000000000000000000000000000000000000000
```
它在内存中占29个字节。这是与之前例子中哈希值的直观对照：
```
0fac49161af82ed938add1d8725835cc123a1a87b1b196488360e58d4bfb51e3
0000010000000000000000000000000000000000000000000000000000000000
0000008b0f41ec78bab747864db66bcb9fb89920ee75f43fdaaeb5544f7f76ca
```
第一个哈希值（用“I like donuts”算出）比目标要大，因此它不是有效的工作量证明。第二个哈希值（用“I like donutsca07ca”算出）比目标值小，所以它是一个有效的证明。<br><br>
可以想到，目标是一个区间的上限：如果一个数（哈希值）比它小就是有效的，反之亦然。上限低的结果是有效的数少，所以，需要更加困难的工作来找到一个有效的。<br><br>
现在需要对数据取哈希值，我们来准备一下数据：
```go
func (pow *ProofOfWork) prepareData(nonce int) []byte {
	data := bytes.Join(
		[][]byte{
			pow.block.PrevBlockHash,
			pow.block.Data,
			IntToHex(pow.block.Timestamp),
			IntToHex(int64(targetBits)),
			IntToHex(int64(nonce)),
		},
		[]byte{},
	)

	return data
}
```
这段代码很简单：我们只是把区块的字段和目标、当前计数合并到一块。`nonce`就是上面介绍的哈希现金中的计数器，它是一个密码学术语。<br><br>
好了，一切就绪，接下来我们实现工作量证明算法的核心部分：
```go
func (pow *ProofOfWork) Run() (int, []byte) {
	var hashInt big.Int
	var hash [32]byte
	nonce := 0

	fmt.Printf("Mining the block containing \"%s\"\n", pow.block.Data)
	for nonce < maxNonce {
		data := pow.prepareData(nonce)
		hash = sha256.Sum256(data)
		fmt.Printf("\r%x", hash)
		hashInt.SetBytes(hash[:])

		if hashInt.Cmp(pow.target) == -1 {
			break
		} else {
			nonce++
		}
	}
	fmt.Print("\n\n")

	return nonce, hash[:]
}
```
首先，我们初始化变量：`hashInt`是哈希值的整数表示；`nonce`是计数器。接下来，跑一个“无限”循环：它受`maxNonce`限制，即`mathMaxInt64`；这是为了避免`nonce`溢出。尽管我们的PoW实现对`nonce`溢出来说难度太小了，不过最好加上这个检查，以防万一。<br><br>
循环体中做了这些：<br>
1. 准备数据。
2. 用SHA-256计算哈希值。
3. 把哈希值转换成大整型。
4. 与目标进行比较。


这与之前说的一样简单。现在我们可以删掉`Block`的`SetHash`方法并修改`NewBlock`函数：
```go
func NewBlock(data string, prevBlockHash []byte) *Block {
	block := &Block{time.Now().Unix(), []byte(data), prevBlockHash, []byte{}, 0}
	pow := NewProofOfWork(block)
	nonce, hash := pow.Run()

	block.Hash = hash[:]
	block.Nonce = nonce

	return block
}
```
现在可以看到，`nonce`作为`Block`的属性保存了下来。验证一个证明的时候要用到`nonce`，因此这是有必要的。现在的`Block`结构体是这样的：
```go
type Block struct {
	Timestamp     int64
	Data          []byte
	PrevBlockHash []byte
	Hash          []byte
	Nonce         int
}
```
好，我们运行一下程序看看是否一切正常：
```
Mining the block containing "Genesis Block"
00000041662c5fc2883535dc19ba8a33ac993b535da9899e593ff98e1eda56a1

Mining the block containing "Send 1 BTC to Ivan"
00000077a856e697c69833d9effb6bdad54c730a98d674f73c0b30020cc82804

Mining the block containing "Send 2 more BTC to Ivan"
000000b33185e927c9a989cc7d5aaaed739c56dad9fd9361dea558b9bfaf5fbe

Prev. hash:
Data: Genesis Block
Hash: 00000041662c5fc2883535dc19ba8a33ac993b535da9899e593ff98e1eda56a1

Prev. hash: 00000041662c5fc2883535dc19ba8a33ac993b535da9899e593ff98e1eda56a1
Data: Send 1 BTC to Ivan
Hash: 00000077a856e697c69833d9effb6bdad54c730a98d674f73c0b30020cc82804

Prev. hash: 00000077a856e697c69833d9effb6bdad54c730a98d674f73c0b30020cc82804
Data: Send 2 more BTC to Ivan
Hash: 000000b33185e927c9a989cc7d5aaaed739c56dad9fd9361dea558b9bfaf5fbe
```
耶！你可以看到每个哈希值都是三个零开头，得到这些哈希值是要花费一定时间的。<br><br>
还有一件事需要做：验证工作量证明。
```go
func (pow *ProofOfWork) Validate() bool {
	var hashInt big.Int

	data := pow.prepareData(pow.block.Nonce)
	hash := sha256.Sum256(data)
	hashInt.SetBytes(hash[:])

	isValid := hashInt.Cmp(pow.target) == -1

	return isValid
}
```
这就是我们要用之前保存的nonce的地方。<br><br>
再次检查是否一切正常：
```go
func main() {
	...

	for _, block := range bc.blocks {
		...
		pow := NewProofOfWork(block)
		fmt.Printf("PoW: %s\n", strconv.FormatBool(pow.Validate()))
		fmt.Println()
	}
}
```
输出：
```
...

Prev. hash:
Data: Genesis Block
Hash: 00000093253acb814afb942e652a84a8f245069a67b5eaa709df8ac612075038
PoW: true

Prev. hash: 00000093253acb814afb942e652a84a8f245069a67b5eaa709df8ac612075038
Data: Send 1 BTC to Ivan
Hash: 0000003eeb3743ee42020e4a15262fd110a72823d804ce8e49643b5fd9d1062b
PoW: true

Prev. hash: 0000003eeb3743ee42020e4a15262fd110a72823d804ce8e49643b5fd9d1062b
Data: Send 2 more BTC to Ivan
Hash: 000000e42afddf57a3daa11b43b2e0923f23e894f96d1f24bfd9b8d2d494c57a
PoW: true
```

### 结语
我们的区块链离实际的架构更近了一步：现在添加区块需要困难的工作，因而可以挖矿。但它还缺少一些关键特性：数据库不是持久化的，没有钱包、地址、交易，没有共识机制。这些特性我们将在后续文章中实现，目前的话，愉快地挖矿吧！<br><br><br>

---

链接：

1. [完整源码](https://github.com/Jeiwan/blockchain_go/tree/part_2)
2. [区块链哈希算法](https://en.bitcoin.it/wiki/Block_hashing_algorithm)
3. [工作量证明](https://en.bitcoin.it/wiki/Proof_of_work)
4. [哈希现金](https://en.bitcoin.it/wiki/Hashcash)
