---
title: golang中time.Time“零值”的JSON处理 
date: 2018-05-29
draft: false
---

## 场景

数据库为mongodb，驱动为mgo。从库中取数据后以json格式返给调用者。

```go
type MyStruct struct{
	Time time.Time
}
```
<!--more-->


Time的MarshalJSON实现是转化为RFC3339格式的时间，
若没有赋值，格式化后为0001-01-01T00:00:00Z, 对调用者极不友好

需求：未赋值则返回`null`


### `json:",omitempty"`

time.Time是结构体，不存在`0值`，此路不通

### 换`time.Time`为`*time.Time`

指向0值的指针不是空指针，数据库中现有数据肯定还是`0001-01-01T00:00:00Z`

姿势不优雅


### 实现`json.Marshaler`

如果时间是0, 就直接返回`null`

```go
type CustomTime struct{
	time.Time
}

func (t CustomTime) MarshalJSON() ([]byte, error) {
	fmt.Println(t.String())
	if t.Time.IsZero() {
		return []byte("null"), nil
	}
	return t.Time.MarshalJSON()
}

```

### 大功告成

其实并没有。经测试发现，没赋值的变成了null，有正常值的也变成了null

因为bson非json, mgo解析数据时不会调用json.Unmarshaler，CustomTime不再是`time.Time`

解决：

实现`bson.Getter`和`bson.Setter`

```go
func (t CustomTime) GetBSON() (interface{}, error) {
	if t.Time.IsZero() {
		return nil, nil
	}
	return t.Time, nil
}

func (t *CustomTime) SetBSON(raw bson.Raw) error {
	var tm time.Time
	if err := raw.Unmarshal(&tm); err != nil {
		return err
	}
	t.Time = tm
	return nil
}

```
