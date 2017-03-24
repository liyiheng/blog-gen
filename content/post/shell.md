---
date: 2017-03-24
title: "shell资料整理"
draft: true
---

## shell资料整理
>来源：http://c.biancheng.net/cpp/view/2737.html<br>
个人学习使用，如有侵权，联系删除

- [变量](#变量)
- [特殊变量](#特殊变量)
- [替换](#替换)


## 变量

#### 定义变量
定义变量时，变量名不加美元符号（$），如：
`variableName="value"`<br>
注意，变量名和等号之间不能有空格，这可能和你熟悉的所有编程语言都不一样。同时，变量名的命名须遵循如下规则：
首个字符必须为字母（a-z，A-Z）。
中间不能有空格，可以使用下划线（_）。
不能使用标点符号。
不能使用bash里的关键字（可用help命令查看保留关键字）。
变量定义举例：<br>
```sh
myUrl="http://see.xidian.edu.cn/cpp/linux/"
myNum=100
```

#### 使用变量
使用一个定义过的变量，只要在变量名前面加美元符号（$）即可，如：
```sh
your_name="mozhiyan"
echo $your_name
echo ${your_name}
```
变量名外面的花括号是可选的，加不加都行，加花括号是为了帮助解释器识别变量的边界，比如下面这种情况：
```sh
for skill in Ada Coffe Action Java 
do
    echo "I am good at ${skill}Script"
done
```
如果不给skill变量加花括号，写成echo "I am good at $skillScript"，解释器就会把$skillScript当成一个变量（其值为空），代码执行结果就不是我们期望的样子了。
推荐给所有变量加上花括号，这是个好的编程习惯。

#### 重新定义变量
已定义的变量，可以被重新定义，如：
```sh
myUrl="http://see.xidian.edu.cn/cpp/linux/"
echo ${myUrl}

myUrl="http://see.xidian.edu.cn/cpp/shell/"
echo ${myUrl}
```
这样写是合法的，但注意，第二次赋值的时候不能写 
`$myUrl="http://see.xidian.edu.cn/cpp/shell/"`
，使用变量的时候才加美元符（$）。

#### 只读变量
使用 readonly 命令可以将变量定义为只读变量，只读变量的值不能被改变。
下面的例子尝试更改只读变量，结果报错：
```sh
#!/bin/bash

myUrl="http://see.xidian.edu.cn/cpp/shell/"
readonly myUrl
myUrl="http://see.xidian.edu.cn/cpp/danpianji/"
```
运行脚本，结果如下：
```sh
/bin/sh: NAME: This variable is read only.
```

#### 删除变量
使用 unset 命令可以删除变量。<br>语法：
`unset variable_name`
变量被删除后不能再次使用；unset 命令不能删除只读变量。
举个例子：
```sh
#!/bin/sh
myUrl="http://see.xidian.edu.cn/cpp/u/xitong/"
unset myUrl
echo $myUrl
```
上面的脚本没有任何输出。

#### 变量类型
运行shell时，会同时存在三种变量：
1. 局部变量 局部变量在脚本或命令中定义，仅在当前shell实例中有效，其他shell启动的程序不能访问局部变量。
2. 环境变量 所有的程序，包括shell启动的程序，都能访问环境变量，有些程序需要环境变量来保证其正常运行。必要的时候shell脚本也可以定义环境变量。
3. shell变量 shell变量是由shell程序设置的特殊变量。shell变量中有一部分是环境变量，有一部分是局部变量，这些变量保证了shell的正常运行

## 特殊变量
- $0 当前脚本的文件名。
- $n 传递给脚本或函数的参数。n 是一个数字，表示第几个参数。例如，第一个参数是$1，第二个参数是$2。
- $# 传递给脚本或函数的参数个数。
- $* 传递给脚本或函数的所有参数。
- $@ 传递给脚本或函数的所有参数。被双引号(" ")包含时，与 $* 稍有不同。
- $? 上个命令的退出状态，或函数的返回值。
- $$ 当前Shell进程ID。对于 Shell 脚本，就是这些脚本所在的进程ID。

>$* 和 $@ 都表示传递给函数或脚本的所有参数，不被双引号(" ")包含时，都以"$1" "$2" … "$n" 的形式输出所有参数。
但是当它们被双引号(" ")包含时，"$*" 会将所有的参数作为一个整体，以"$1 $2 … $n"的形式输出所有参数；"$@" 会将各个参数分开，以"$1" "$2" … "$n" 的形式输出所有参数。

栗子：
```sh
#!/bin/bash
echo "\$*=" $*
echo "\"\$*\"=" "$*"

echo "\$@=" $@
echo "\"\$@\"=" "$@"

echo "print each param from \$*"
for var in $*
do
    echo "$var"
done

echo "print each param from \$@"
for var in $@
do
    echo "$var"
done

echo "print each param from \"\$*\""
for var in "$*"
do
    echo "$var"
done

echo "print each param from \"\$@\""
for var in "$@"
do
    echo "$var"
done
```
执行 ./test.sh "a" "b" "c" "d"，结果：
```sh
$*=  a b c d
"$*"= a b c d
$@=  a b c d
"$@"= a b c d
print each param from $*
a
b
c
d
print each param from $@
a
b
c
d
print each param from "$*"
a b c d
print each param from "$@"
a
b
c
d
```

## 替换
>如果表达式中包含特殊字符，Shell 将会进行替换。例如，在双引号中使用变量就是一种替换，转义字符也是一种替换。

如：
```sh
#!/bin/bash

a=10
echo -e "Value of a is $a \n"
```
这里 -e 表示对转义字符进行替换。如果不使用 -e 选项，将会原样输出：
```sh
Value of a is 10\n
```
下面的转义字符都可以用在 echo 中：

- \\\\	反斜杠
- \a	警报，响铃
- \b	退格（删除键）
- \f	换页(FF)，将当前位置移到下页开头
- \n	换行
- \r	回车
- \t	水平制表符（tab键） 
- \v	垂直制表符 

可以使用 echo 命令的 -E 选项禁止转义，默认也是不转义的；使用 -n 选项可以禁止插入换行符。

#### 命令替换
>命令替换是指Shell可以先执行命令，将输出结果暂时保存，在适当的地方输出。
 
命令替换的语法： **\`command\`**<br>
栗子：
```sh
#!/bin/bash

DATE=`date`
echo "Date is $DATE"

USERS=`who | wc -l`
echo "Logged in user are $USERS"

UP=`date ; uptime`
echo "Uptime is $UP"
```

#### 变量替换
>变量替换可以根据变量的状态（是否为空、是否定义等）来改变它的值

*To be continued*
