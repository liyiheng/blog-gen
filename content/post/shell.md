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
- [运算符](#运算符)

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


可以使用的变量替换形式：
- `${var}`  变量本来的值
- `${var:-word}`    如果变量 var 为空或已被删除(unset)，那么返回 word，但不改变 var 的值。
- `${var:=word}`	如果变量 var 为空或已被删除(unset)，那么返回 word，并将 var 的值设置为 word。
- `${var:?message}` 如果变量 var 为空或已被删除(unset)，那么将消息 message 送到标准错误输出，可以用来检测变量 var 是否可以被正常赋值。
若此替换出现在Shell脚本中，那么脚本将停止运行。
- `${var:+word}`    如果变量 var 被定义，那么返回 word，但不改变 var 的值。

栗子：
```sh
#!/bin/bash

echo ${var:-"Variable is not set"}
echo "1 - Value of var is ${var}"

echo ${var:="Variable is not set"}
echo "2 - Value of var is ${var}"

unset var
echo ${var:+"This is default value"}
echo "3 - Value of var is $var"

var="Prefix"
echo ${var:+"This is default value"}
echo "4 - Value of var is $var"

echo ${var:?"Print this message"}
echo "5 - Value of var is ${var}"
```
结果：
```sh
Variable is not set
1 - Value of var is
Variable is not set
2 - Value of var is Variable is not set

3 - Value of var is
This is default value
4 - Value of var is Prefix
Prefix
5 - Value of var is Prefix
```

## 运算符 
>Bash 支持很多运算符，包括算数运算符、关系运算符、布尔运算符、字符串运算符和文件测试运算符。原生bash不支持简单的数学运算，但是可以通过其他命令来实现，例如 awk 和 expr，expr 最常用。

expr是一款表达式计算工具，使用它能完成表达式的求值操作。例如，两个数相加：
```sh
#!/bin/bash

val=`expr 2 + 2`
echo "Total value : $val"
```
输出`Total value : 4`

两点注意：
- 表达式和运算符之间要有空格，例如 2+2 是不对的，必须写成 2 + 2。
- 完整的表达式要被 \`\` 包含。

#### 算术运算符


运算符	说明	举例
- \+	加法\`expr $a + $b\` 结果为 30。
- \-	减法	\`expr $a - $b\` 结果为 10。
- \*	乘法	\`expr $a \\* $b\` 结果为  200。
- /	除法	\`expr $b / $a\` 结果为 2。
- %	取余	\`expr $b % $a\` 结果为 0。
- =	赋值	a=$b 将把变量 b 的值赋给 a。
- ==	相等。用于比较两个数字，相同则返回 true。	[ $a == $b ] 返回 false。
- !=	不相等。用于比较两个数字，不相同则返回 true。	[ $a != $b ] 返回 true。

注意：条件表达式要放在方括号之间，并且要有空格，例如 [$a==$b] 是错误的，必须写成 [ $a == $b ]。<br>
栗子：
```sh
#!/bin/sh

a=10
b=20
val=`expr $a + $b`
echo "a + b : $val"

val=`expr $a - $b`
echo "a - b : $val"

val=`expr $a \* $b`
echo "a * b : $val"

val=`expr $b / $a`
echo "b / a : $val"

val=`expr $b % $a`
echo "b % a : $val"

if [ $a == $b ]
then
   echo "a is equal to b"
fi

if [ $a != $b ]
then
   echo "a is not equal to b"
fi
```

#### 关系运算符

- -eq	检测两个数是否相等，相等返回 true。	[ $a -eq $b ] 返回 true。
- -ne	检测两个数是否相等，不相等返回 true。	[ $a -ne $b ] 返回 true。
- -gt	检测左边的数是否大于右边的，如果是，则返回 true。	[ $a -gt $b ] 返回 false。
- -lt	检测左边的数是否小于右边的，如果是，则返回 true。	[ $a -lt $b ] 返回 true。
- -ge	检测左边的数是否大等于右边的，如果是，则返回 true。	[ $a -ge $b ] 返回 false。
- -le	检测左边的数是否小于等于右边的，如果是，则返回 true。	[ $a -le $b ] 返回 true。

栗子：
```sh
#!/bin/sh
a=10
b=20
if [ $a -eq $b ]
then
   echo "$a -eq $b : a is equal to b"
else
   echo "$a -eq $b: a is not equal to b"
fi
if [ $a -ne $b ]
then
   echo "$a -ne $b: a is not equal to b"
else
   echo "$a -ne $b : a is equal to b"
fi
if [ $a -gt $b ]
then
   echo "$a -gt $b: a is greater than b"
else
   echo "$a -gt $b: a is not greater than b"
fi
if [ $a -lt $b ]
then
   echo "$a -lt $b: a is less than b"
else
   echo "$a -lt $b: a is not less than b"
fi
if [ $a -ge $b ]
then
   echo "$a -ge $b: a is greater or  equal to b"
else
   echo "$a -ge $b: a is not greater or equal to b"
fi
if [ $a -le $b ]
then
   echo "$a -le $b: a is less or  equal to b"
else
   echo "$a -le $b: a is not less or equal to b"
fi
```

#### 关系运算符
关系运算符只支持数字，不支持字符串，除非字符串的值是数字。
- -eq	检测两个数是否相等，相等返回 true。	[ $a -eq $b ] 返回 true。
- -ne	检测两个数是否相等，不相等返回 true。	[ $a -ne $b ] 返回 true。
- -gt	检测左边的数是否大于右边的，如果是，则返回 true。	[ $a -gt $b ] 返回 false。
- -lt	检测左边的数是否小于右边的，如果是，则返回 true。	[ $a -lt $b ] 返回 true。
- -ge	检测左边的数是否大等于右边的，如果是，则返回 true。	[ $a -ge $b ] 返回 false。
- -le	检测左边的数是否小于等于右边的，如果是，则返回 true。	[ $a -le $b ] 返回 true。

栗子：
```sh
#!/bin/sh
a=10
b=20
if [ $a -eq $b ]
then
   echo "$a -eq $b : a is equal to b"
else
   echo "$a -eq $b: a is not equal to b"
fi
if [ $a -ne $b ]
then
   echo "$a -ne $b: a is not equal to b"
else
   echo "$a -ne $b : a is equal to b"
fi
if [ $a -gt $b ]
then
   echo "$a -gt $b: a is greater than b"
else
   echo "$a -gt $b: a is not greater than b"
fi
if [ $a -lt $b ]
then
   echo "$a -lt $b: a is less than b"
else
   echo "$a -lt $b: a is not less than b"
fi
if [ $a -ge $b ]
then
   echo "$a -ge $b: a is greater or  equal to b"
else
   echo "$a -ge $b: a is not greater or equal to b"
fi
if [ $a -le $b ]
then
   echo "$a -le $b: a is less or  equal to b"
else
   echo "$a -le $b: a is not less or equal to b"
fi
```

#### 布尔运算符

-   !	非运算，表达式为 true 则返回 false，否则返回 true。	[ ! false ] 返回 true。
-   -o	或运算，有一个表达式为 true 则返回 true。	[ $a -lt 20 -o $b -gt 100 ] 返回 true。
-   -a	与运算，两个表达式都为 true 才返回 true。	[ $a -lt 20 -a $b -gt 100 ] 返回 false

栗子：
```sh
#!/bin/sh
a=10
b=20
if [ $a != $b ]
then
   echo "$a != $b : a is not equal to b"
else
   echo "$a != $b: a is equal to b"
fi
if [ $a -lt 100 -a $b -gt 15 ]
then
   echo "$a -lt 100 -a $b -gt 15 : returns true"
else
   echo "$a -lt 100 -a $b -gt 15 : returns false"
fi
if [ $a -lt 100 -o $b -gt 100 ]
then
   echo "$a -lt 100 -o $b -gt 100 : returns true"
else
   echo "$a -lt 100 -o $b -gt 100 : returns false"
fi
if [ $a -lt 5 -o $b -gt 100 ]
then
   echo "$a -lt 100 -o $b -gt 100 : returns true"
else
   echo "$a -lt 100 -o $b -gt 100 : returns false"
fi
```

#### 字符串运算符
- =	检测两个字符串是否相等，相等返回 true。	[ $a = $b ] 返回 false。
- !=	检测两个字符串是否相等，不相等返回 true。	[ $a != $b ] 返回 true。
- -z	检测字符串长度是否为0，为0返回 true。	[ -z $a ] 返回 false。
- n	检测字符串长度是否为0，不为0返回 true。	[ -z $a ] 返回 true。
- str	检测字符串是否为空，不为空返回 true。	[ $a ] 返回 true。

栗子：
```sh
#!/bin/sh
a="abc"
b="efg"
if [ $a = $b ]
then
   echo "$a = $b : a is equal to b"
else
   echo "$a = $b: a is not equal to b"
fi
if [ $a != $b ]
then
   echo "$a != $b : a is not equal to b"
else
   echo "$a != $b: a is equal to b"
fi
if [ -z $a ]
then
   echo "-z $a : string length is zero"
else
   echo "-z $a : string length is not zero"
fi
if [ -n $a ]
then
   echo "-n $a : string length is not zero"
else
   echo "-n $a : string length is zero"
fi
if [ $a ]
then
   echo "$a : string is not empty"
else
   echo "$a : string is empty"
fi
```
#### 文件测试运算符
-   -b file	检测文件是否是块设备文件，如果是，则返回 true。	[ -b $file ] 返回 false。
-   -c file	检测文件是否是字符设备文件，如果是，则返回 true。	[ -b $file ] 返回 false。
-   -d file	检测文件是否是目录，如果是，则返回 true。	[ -d $file ] 返回 false。
-   -f file	检测文件是否是普通文件（既不是目录，也不是设备文件），如果是，则返回 true。	[ -f $file ] 返回 true。
-   -g file	检测文件是否设置了 SGID 位，如果是，则返回 true。	[ -g $file ] 返回 false。
-   -k file	检测文件是否设置了粘着位(Sticky Bit)，如果是，则返回 true。	[ -k $file ] 返回 false。
-   -p file	检测文件是否是具名管道，如果是，则返回 true。	[ -p $file ] 返回 false。
-   -u file	检测文件是否设置了 SUID 位，如果是，则返回 true。	[ -u $file ] 返回 false。
-   -r file	检测文件是否可读，如果是，则返回 true。	[ -r $file ] 返回 true。
-   -w file	检测文件是否可写，如果是，则返回 true。	[ -w $file ] 返回 true。
-   -x file	检测文件是否可执行，如果是，则返回 true。	[ -x $file ] 返回 true。
-   -s file	检测文件是否为空（文件大小是否大于0），不为空返回 true。	[ -s $file ] 返回 true。
-   -e file	检测文件（包括目录）是否存在，如果是，则返回 true。	[ -e $file ] 返回 true。

栗子：
```sh
#!/bin/sh
file="/var/www/tutorialspoint/unix/test.sh"
if [ -r $file ]
then
   echo "File has read access"
else
   echo "File does not have read access"
fi
if [ -w $file ]
then
   echo "File has write permission"
else
   echo "File does not have write permission"
fi
if [ -x $file ]
then
   echo "File has execute permission"
else
   echo "File does not have execute permission"
fi
if [ -f $file ]
then
   echo "File is an ordinary file"
else
   echo "This is sepcial file"
fi
if [ -d $file ]
then
   echo "File is a directory"
else
   echo "This is not a directory"
fi
if [ -s $file ]
then
   echo "File size is zero"
else
   echo "File size is not zero"
fi
if [ -e $file ]
then
   echo "File exists"
else
   echo "File does not exist"
fi
```
*To be continued*
