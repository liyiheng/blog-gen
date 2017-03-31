---
title: Hello Redis
date: 2017-03-31
draft: true
---

# Hello Redis

😎安装
```
docker pull  redis
```
😎启动server
```
docker run -p 6379:6379  -d redis redis-server --appendonly yes
```
😎启动客户端
```
docker run -it redis redis-cli -h 192.168.1.102
```
常见[命令](https://redis.io/commands)的使用：
```
192.168.1.102:6379> help
redis-cli 3.2.8
To get help about Redis commands type:
      "help @<group>" to get a list of commands in <group>
      "help <command>" for help on <command>
      "help <tab>" to get a list of possible help topics
      "quit" to exit

To set redis-cli perferences:
      ":set hints" enable online hints
      ":set nohints" disable online hints
Set your preferences in ~/.redisclirc
192.168.1.102:6379> 
```
简单的用法，`help <tab>`和`help COMMAND`足矣😆

Redis支持五种数据类型：
- 😒 string（字符串）
- 😒 hash（哈希,k-v）
- 😒 list（列表）
- 😒 set（集合）
- 😒 zset (sorted set：有序集合)

**TODO ：对数据的操作**

两种持久化方式：
- 方式一
- 方式二

## String 
>string是redis最基本的类型，你可以理解成与Memcached一模一样的类型，一个key对应一个value。
string类型是二进制安全的。意思是redis的string可以包含任何数据。比如jpg图片或者序列化的对象 。
string类型是Redis最基本的数据类型，一个键最大能存储512MB。

```
192.168.1.102:6379> SET name "liyiheng"
OK
192.168.1.102:6379> GET name
"liyiheng"
192.168.1.102:6379> 
```
## Hash
>Redis hash 是一个键值对集合。
Redis hash是一个string类型的field和value的映射表，hash特别适合用于存储对象。
每个 hash 可以存储 2^32 -1 键值对（40多亿）

```
192.168.1.102:6379> HMSET hash_key name liyiheng age 666 gender male
OK
192.168.1.102:6379> HGET hash_key name
"liyiheng"
192.168.1.102:6379> HGETALL hash_key
1) "name"
2) "liyiheng"
3) "age"
4) "666"
5) "gender"
6) "male"
192.168.1.102:6379> 
```
## List
>Redis 列表是简单的字符串列表，按照插入顺序排序。你可以添加一个元素到列表的头部或者尾部。
列表最多可存储 2^32 - 1 元素 (4294967295, 每个列表可存储40多亿)。

```
192.168.1.102:6379> LPUSH my_list element0 element1 element2
(integer) 3
192.168.1.102:6379> LPUSH my_list element3
(integer) 4
192.168.1.102:6379> LRANGE my_list 0 3
1) "element3"
2) "element2"
3) "element1"
4) "element0"
```

## Set
>Redis的Set是string类型的无序集合。
集合是通过哈希表实现的，所以添加，删除，查找的复杂度都是O(1)。集合中最大的成员数为 2^32 - 1(4294967295, 每个集合可存储40多亿个成员)
#### sadd 命令
添加一个string元素到key对应的set集合中，成功返回1，如果元素已经在集合中返回0，*key对应的set不存在返回错误?*
```
192.168.1.102:6379> SADD my_set element0
(integer) 1
192.168.1.102:6379> SADD my_set element0
(integer) 0
192.168.1.102:6379> SMEMBERS my_set
1) "element0"
192.168.1.102:6379> 
```
## zset(sorted set)
>Redis zset 和 set 一样也是string类型元素的集合,且不允许重复的成员。
不同的是每个元素都会关联一个double类型的分数。redis正是通过分数来为集合中的成员进行从小到大的排序。
zset的成员是唯一的,但分数(score)却可以重复。
#### zadd 命令`zadd key score member`
添加元素到集合，元素在集合中存在则更新对应score
```
192.168.1.102:6379> ZADD my_sorted_set 0 java 1 golang 2 C/C++ 2 pyhton  
(integer) 4
192.168.1.102:6379> ZRANGEBYSCORE my_sorted_set 0 100
1) "java"
2) "golang"
3) "C/C++"
4) "pyhton"
192.168.1.102:6379> 
```
# todo list
## 进一步学习：持久化，备份，恢复，分区，blablabla...  
> To be continued

## golang中通过[redigo](https://github.com/garyburd/redigo)使用redis
> To be continued

