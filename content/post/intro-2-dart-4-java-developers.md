> [Google codelabs](https://codelabs.developers.google.com/codelabs/from-java-to-dart/index.html#1)的Dart笔记


## 定义一个Bicycle类
```Dart
class Bicycle {
  int cadence;
  int speed;
  int gear;
}

void main() {
}
```
#### 观察结果
- Dart的主方法是`main()`或者`main(List<String> args)`
- `main()`方法处于最外层。Dart中可以在类的外部定义代码。变量、函数等都可以处于类的外部
- Java中可以用`private`修饰符声明私有成员，Dart中没有`private`、`public`、`protected`关键字
- `main()`和`Bicycle`都没有用`public`修饰符，Dart中所有把字符默认都是public
- Dart惯例是用两个字符缩进，而不是四个

### 定义一个Bicycle构造器
在Bicycle类中加入如下构造器：
```Dart
Bicycle(this.cadence, this.speed, this.gear);
```
#### 观察结果
- 这个构造方法没有结构体,在Dart中可以这样
- 在构造方法参数列表中使用`this`是给实例变量赋值的便捷方式

### 实例化并打印一个实例
```Dart
void main() {
  var bike = new Bicycle(2, 0, 1);
  print(bike);
}
```
输出结果
```
Instance of 'Bicycle'
```
所有的Dart类都有`toString()`方法，重写该方法：
```Dart
@override
String toString() => 'Bicycle: $speed mph';
```
打印结果：
```
Bicycle: 0 mph
```
#### 观察结果
- `@override`注解，与Java中的类似
- Dart中字符串可以用单引号或双引号
- 字符串插值`$variableName`或`${variableName}`
- 函数或方法可以用箭头标记法简化成单行(`=>`)，类似lambda？

### 添加一个只读变量
Java中只读的实现：private + getter()<br>
Dart中没有private，以下划线开头的变量为私有<br>
#### 把speed改为只读的
`speed`改为`_speed`并为Bicycle类加一个getter:
```Dart
int get speed => _speed;
``` 
#### 观察结果
- Dart中下划线开头的为私有
- Dart默认为所有公开变量提供隐式的getter和setter

### 最终的Bicycle类
```Dart
class Bicycle {
  int cadence;
  int _speed;
  int gear;

  Bicycle(this.cadence, this._speed, this.gear);

  int get speed => _speed;

  void applyBrake(int decrement) {
    _speed -= decrement;
  }

  void speedUp(int increment) {
    _speed += increment;
  }

  @override
  String toString() => 'Bicycle: $_speed mph';
}

main() {
  var bike = new Bicycle(2, 0, 1);
  print(bike);
}
```

## 可选参数
矩形
```Dart
import 'dart:math';

class Rectangle {
  int width;
  int height;
  Point origin;
}
```
### 构造方法
```Dart
Rectangle({this.origin = const Point(0, 0), this.width = 0, this.height = 0});
```
#### 观察结果
- `this.origin`等在构造方法声明中以简写的方式赋值
- `this.origin`，`this.width`，`this.height`是可选的命名参数。命名参数用大括号包围
- `this.origin = const Point(0, 0)`是为`origin`指定默认值`Point(0,0)`.默认值必须是编译时常量

### 改进输出
```Dart
@override
String toString() =>
      'Origin: (${origin.x}, ${origin.y}), width: $width, height: $height';
```
### 构造方法的使用
```Dart
main() {
  print(new Rectangle(origin: const Point(10, 20), width: 100, height: 200));
  print(new Rectangle(origin: const Point(10, 10)));
  print(new Rectangle(width: 200));
  print(new Rectangle());
}
// 输出结果
// Origin: (10, 20), width: 100, height: 200
// Origin: (10, 10), width: 0, height: 0
// Origin: (0, 0), width: 200, height: 0
// Origin: (0, 0), width: 0, height: 0
```

## 创建工厂（工厂模式）

