---
date: 2017-03-28
title: "Hello protobuf !"
draft: false
categories:
  - golang
tags:
  - protobuf
  - Protocol Buffer
#thumbnailImagePosition: left
---
protobuf，独立于语言和平台的二进制数据传输格式。项目地址 https://github.com/google/protobuf
<!--more-->

>[官网](https://developers.google.com/protocol-buffers/)被墙，贴两段介绍

##### What are protocol buffers?
Protocol buffers are a flexible, efficient, automated mechanism for serializing structured data – think XML, but smaller, faster, and simpler. You define how you want your data to be structured once, then you can use special generated source code to easily write and read your structured data to and from a variety of data streams and using a variety of languages. You can even update your data structure without breaking deployed programs that are compiled against the "old" format.

##### Why not just use XML?
Protocol buffers have many advantages over XML for serializing structured data. Protocol buffers:

- are simpler
- are 3 to 10 times smaller
- are 20 to 100 times faster
- are less ambiguous
- generate data access classes that are easier to use programmatically

### 定义协议格式
创建文件`message.proto`内容如下：<br>
```
syntax = "proto3";
package protobuf;

message SearchRequest {
    string query = 1;// 唯一的数字在二进制数据中用来区分字段，一旦使用，不可轻易更改
    int32 page_size = 2;
    int32 page_number = 3;
}
message User {
    string name = 1;
    int32 age = 2;
    bool gender = 3;
}
message SearchResponse {
    int32 code = 1;
    //singular: a well-formed message can have zero or one of this field (but not more than one).
    repeated User users = 2;
    // repeated: this field can be repeated any number of times (including zero) in a well-formed message.
    // The order of the repeated values will be preserved.
}
```
### 生成代码
先下载编译好的[protoc](https://github.com/google/protobuf)<br>
生成golang代码需要一个插件`protoc-gen-go`<br>
```sh
$ go get github.com/golang/protobuf/protoc-gen-go
```
这样就可以通过protoc生成对应的golang代码`message.pb.go`。同时需要生成相应的Java代码`Message.java`供Android客户端使用。

### 简单使用
服务端
```go
func main() {
	http.HandleFunc("/protobuf",doSth)
	http.ListenAndServe(":8080",nil )
}

func doSth(rw http.ResponseWriter, req *http.Request) {
	resp := protobuf.SearchResponse{Code:0}
	users := make([]*protobuf.User, 20)
	for i := 0; i < 20; i++ {
		u := &protobuf.User{
			Name:"name" + strconv.Itoa(i),
			Age:int32(i),
			Gender:i % 2 == 0,
		}
		users[i] = u
	}
	resp.Users = users
	data, err := proto.Marshal(&resp)
	if err != nil {
		rw.Write([]byte(err.Error()))
	} else {
		rw.Write(data)
	}
}
```
客户端
```java
/**
 * Created by liyiheng on 17/03/07.
 */
public class MainActivity extends AppCompatActivity implements View.OnClickListener {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        findViewById(R.id.textView).setOnClickListener(this);
        EasyVolley.init(this);
    }

    @Override
    public void onClick(View v) {
        EasyVolley.stringGet("http://192.168.1.101:8080/protobuf", new EasyCallBack() {
            @Override
            protected void onResponse(String response) {
                try {
                    Message.SearchResponse parseFrom = Message.SearchResponse.parseFrom(response.getBytes());
                    Log.e("ParsedResponse", "" + parseFrom.toString());
                } catch (InvalidProtocolBufferException e) {
                    e.printStackTrace();
                }

            }

            @Override
            protected void onErrorResponse(VolleyError error) {
                Log.e("Error", error.getMessage());
            }
        });
    }
}
```
