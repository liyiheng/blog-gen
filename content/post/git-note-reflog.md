---
title: git笔记(reflog)
date: 2017-08-10
draft: false
categories:
  - git
tags:
  - git
---


>不熟悉git，差点翻车

<!--more-->
一如既往，在项目中添加某特性后愉快地提交
```sh
git commit -m "Feature add: blablabla" -- a.go b.go c.go
```
`git push`前习惯性地`git status`<br>
于是发现了漏网的：
```sh
On branch develop
Your branch is up-to-date with 'origin/develop'.
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git checkout -- <file>..." to discard changes in working directory)

        modified:   没打算提交.go
        modified:   忘了提交.go

no changes added to commit (use "git add" and/or "git commit -a")
```
漏了个文件而已，老司机肯定会顺手补上：
```sh
git commit --amend 忘了提交.go
```
然而渣渣哪会这种操作，那就reset再重新提交吧
```sh
git reset --keep 上一次的commitID
```
这时候我想要的git status结果是:
```sh
n branch develop
Your branch is up-to-date with 'origin/develop'.
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git checkout -- <file>..." to discard changes in working directory)

        modified:   没打算提交.go
        modified:   忘了提交.go
	modified:   a.go
	modified:   b.go
	modified:   c.go

no changes added to commit (use "git add" and/or "git commit -a")
```
其实是用错了reset的参数，如果用了mixed就是上面的结果
详情请参阅`git help reset`
由于用错了参数，看到的结果仍然是：
```sh
On branch develop
Your branch is up-to-date with 'origin/develop'.
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git checkout -- <file>..." to discard changes in working directory)

        modified:   没打算提交.go
        modified:   忘了提交.go

no changes added to commit (use "git add" and/or "git commit -a")
```
而且，刚刚提交的文件也变回了上次提交后的状态。也就是说上次提交的更改都丢了，反而对`忘了提交.go`的更改没有丢失<br>
呵呵哒，吓得我键盘都飞出去了<br>
于是仔(大)细(概)研(了)究(解)了一下git的机制。<br>
果然，还是没找到办法<br>
方了，彻底方了<br>
。。。<br>
。。。<br>
google诚不我欺，无意间搜到了`reflog`<br>

-----

先`git reflog`：
```sh
cc976e6 HEAD@{0}: reset: moving to cc976e634a9c7bc81ebfa27668fe24bc6804c80d
c24d9b5 HEAD@{1}: commit: Feature add: blablabla
```
再`git reset --keep c24d9b5`<br>
最后`git commit --amend 忘了提交.go`
