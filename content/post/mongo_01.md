## mongodb入门
### 安装
在docker中使用，因此：
```sh
docker pull mongo
```
可能是网络不好，在EOF错误出现数次后成功
启动mongo
```sh
docker run -p 27017:27017 mongo
```
连接到docker, `192.168.1.102`为本机ip
```sh
docker run -it mongo mongo --host 192.168.1.102
```
### 一些概念（个人理解）
- 数据库，同关系型数据库中的数据库
- 集合，关系型数据库中的表
- 文档，关系型数据库中的记录、行
### help
```js
> help
	db.help()                    help on db methods
	db.mycoll.help()             help on collection methods
	sh.help()                    sharding helpers
	rs.help()                    replica set helpers
	help admin                   administrative help
	help connect                 connecting to a db help
	help keys                    key shortcuts
	help misc                    misc things to know
	help mr                      mapreduce

	show dbs                     show database names
	show collections             show collections in current database
	show users                   show users in current database
	show profile                 show most recent system.profile entries with time >= 1ms
	show logs                    show the accessible logger names
	show log [name]              prints out the last segment of log in memory, 'global' is default
	use <db_name>                set current database
	db.foo.find()                list objects in collection foo
	db.foo.find( { a : 1 } )     list objects in foo where a == 1
	it                           result of the last line evaluated; use to further iterate
	DBQuery.shellBatchSize = x   set default number of items to display on shell
	exit                         quit the mongo shell
> 
```
### 准备
#### 创建和选择数据库


### CRUD
#### insert
在关系型数据库中(以MySql为例)，通常是`CREATE DB_NAME`创建数据库；`USE DB_NAME`选择数据库；`show databases`查看数据库列表。<br>
mongo中查看数据库是`show dbs`，创建和选择都是`use db_name`。
```js
> show dbs
admin  0.000GB
local  0.000GB
>use play_ground
switched to db play_ground
> db.first_collection.insert({"name":"liyiheng"})
WriteResult({ "nInserted" : 1 })
> show dbs
admin        0.000GB
local        0.000GB
play_ground  0.000GB
> db.first_collection.insert({"name":"liyiheng1"})
WriteResult({ "nInserted" : 1 })
> db.first_collection.insert({"name":"liyiheng2","age":123})
WriteResult({ "nInserted" : 1 })
> db.first_collection.insert({"name":"liyiheng3","age":11})
WriteResult({ "nInserted" : 1 })
> 
```
`use play_ground`是切换到数据库`play_ground`，如果不存在则创建。<br>
`db.first_collection.insert({"name":"liyiheng1"})`是在集合`first_collection`中插入文档`{"name":"liyiheng1"}`，如果集合不存在就创建。
#### find
```js
> db.first_collection.find({"name":"liyiheng2"})
{ "_id" : ObjectId("58db67addbb9744fab3b1d9b"), "name" : "liyiheng2", "age" : 123 }
```
#### update
```js
> db.first_collection.update({"name":"liyiheng"},{"age":666})
WriteResult({ "nMatched" : 1, "nUpserted" : 0, "nModified" : 1 })
> db.first_collection.find()
{ "_id" : ObjectId("58db6716dbb9744fab3b1d99"), "age" : 666 }
{ "_id" : ObjectId("58db679fdbb9744fab3b1d9a"), "name" : "liyiheng1" }
{ "_id" : ObjectId("58db67addbb9744fab3b1d9b"), "name" : "liyiheng2", "age" : 123 }
{ "_id" : ObjectId("58db67b7dbb9744fab3b1d9c"), "name" : "liyiheng3", "age" : 11 }
```
原文档中的name字段没了，因此应该这样：<br>
```js
> db.first_collection.update({"age":666},{"name":"liyiheng","age":6666})
WriteResult({ "nMatched" : 1, "nUpserted" : 0, "nModified" : 1 })
> db.first_collection.find()
{ "_id" : ObjectId("58db6716dbb9744fab3b1d99"), "name" : "liyiheng", "age" : 6666 }
{ "_id" : ObjectId("58db679fdbb9744fab3b1d9a"), "name" : "liyiheng1" }
{ "_id" : ObjectId("58db67addbb9744fab3b1d9b"), "name" : "liyiheng2", "age" : 123 }
{ "_id" : ObjectId("58db67b7dbb9744fab3b1d9c"), "name" : "liyiheng3", "age" : 11 }
```
#### remove

# *To be continued*
