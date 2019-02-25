---
date: 2017-02-03
title: "JS学习笔记--ROT13转换"
draft: true
categories:
  - JavaScript
tags:
  - JavaScript
  - ROT13
thumbnailImagePosition: left
---



JS字符串的练习，仅对大写做了处理

 - String.fromCharCode()
 - String.prototype.charCodeAt()
 
<!--more-->

##### 关于ROT13：</br>
ROT13 ("rotate by 13 places", sometimes hyphenated ROT-13) is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. ROT13 is a special case of the Caesar cipher, developed in ancient Rome.</br>
详细介绍见[wiki](https://en.wikipedia.org/wiki/ROT13 "ROT13")


代码如下：
```javascript
  function rot13(str) {
    var arr = [];
    for(var i=0;i<str.length;i++){
      var code = str.charCodeAt(i);
      if(code>=65 && code <= 90){
        code += 13;
        if(code>90){
          code -= 26;
        }
      }
      arr.push(String.fromCharCode(code));
    }
    return arr.join('');
  }
```
