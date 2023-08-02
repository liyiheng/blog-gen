---
title: "vim-go 和 coc.nvim 共享 gopls daemon"
author: "liyiheng"
date: 2023-08-02T22:34:24+08:00
subtitle: "通过共享 gopls 避免不必要的内存开销"
image: ""
tags: [golang, go, vim, coc.nvim, vim-go]
type: ""
---

TLDR：
指定环境变量即可
vimrc 中 `let $XDG_RUNTIME_DIR='/tmp'` 
或 zshrc 中 `export XDG_RUNTIME_DIR=/tmp`

<!--more-->

共享 `gopls` , `gopls` 又是什么呢？
> gopls (pronounced "Go please") is the official Go language server developed by the Go team. It provides IDE features to any LSP-compatible editor.

`gopls` 是 go 语言官方的语言服务器，可以为支持 
[LSP](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
协议的编辑器提供 IDE 功能，例如自动提示、补全、跳转、查找引用、重命名等。

[vim-go](https://github.com/fatih/vim-go) 
是一个功能丰富的 vim golang 开发插件，其部分功能基于 `gopls` 实现；
[coc.nvim](https://github.com/neoclide/coc.nvim) 
是基于 LSP 的智能感知插件，提供了优秀的补全体验，同时在其基础上可以安装各种 coc 插件。


coc.nvim 与 vim-go 功能并不重叠，难免有同时使用的情况。
这时打开 go 项目，coc 和 vim-go 会各自启动一个 gopls 实例，产生四个 gopls 进程，耗费了两份内存。
四个进程分别为 2 个 forwarder 和 2 个 daemon，预期是 2 个 forwarder 和 1 个 daemon，
关于 forwarder 和 daemon 详见 
[gopls execution modes](https://github.com/golang/tools/blob/master/gopls/doc/daemon.md#background-gopls-execution-modes)
此时 `vimrc` 中两者相关配置:
```vim
call plug#begin('~/.vim/plugged')
Plug 'fatih/vim-go'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
call plug#end()
let g:coc_user_config['languageserver']['golang'] = {
		\   'command': 'gopls',
		\   'rootPatterns': ['go.mod'],
		\   'filetypes': ['go']
		\}
```
通过相关文档得知，可以通过 `-remote=auto` 参数使 vim-go 和 coc 共享同一个 gopls daemon。
于是调整配置：
```vim
let g:go_gopls_enabled = 1
let g:go_gopls_options = ['-remote=auto']
let g:coc_user_config['languageserver']['golang'] = {
		\   'command': 'gopls',
		\   'args': ['-remote=auto'],
		\   'rootPatterns': ['go.mod'],
		\   'filetypes': ['go']
		\}
```
重新打开 go 项目，发现仍有四个 gopls 进程,索性禁用 vim-go 的 gopls 功能：
```vim
let g:go_gopls_enabled = 0
```
近期想启用 vim-go 的 gopls 特性，于是重新排查这个问题。
无意间发现两个 daemon 使用的 socket 文件名相同，路径不同：
```plaintext
vim-go:
/tmp/gopls-cd85fd-daemon.liyiheng
coc.nvim:
/tmp/nvim.liyiheng/qUMdHS/gopls-cd85fd-daemon.liyiheng
```
猜测之所以未能共享 daemon 是因为 `gopls -remote=auto` 启动时检测不到 socket 导致启动了一个新实例，
那么问题的关键就在 socket 文件路径。
查阅 [gopls 源码](https://github.com/golang/tools/blob/3d20bbe0fb2f6c3c3b340ce4d0bbcd1ad880071c/gopls/internal/lsp/lsprpc/autostart_posix.go#L38)发现其路径规则：
```go
runtimeDir := os.TempDir()
if xdg := os.Getenv("XDG_RUNTIME_DIR"); xdg != "" {
	runtimeDir = xdg
}
return "unix", filepath.Join(runtimeDir, fmt.Sprintf("%s-%s-daemon.%s%s", basename, shortHash, user, idComponent))
```
由此可见，gopls 将 socket 文件创建在 `os.TempDir()` 下。
**当时竟未注意到 `XDG_RUNTIME_DIR` 的优先级更高。**
于是查看 TempDir 的实现：
```go
func TempDir() string {
	return tempDir()
}
func tempDir() string {
	dir := Getenv("TMPDIR")
	if dir == "" {
		if runtime.GOOS == "android" {
			dir = "/data/local/tmp"
		} else {
			dir = "/tmp"
		}
	}
	return dir
}
```
socket 所处路径由环境变量控制，难道 vim 内部会操作环境变量？
即便如此，vim-go 和 coc 都是在 vim 中运行，
vim 的某些环境对两者都应有效。

最终带着疑问给 vim-go 提了 [issue](https://github.com/fatih/vim-go/issues/3566)。
通过 @bhcleek 提供的信息，
了解到问题的根源是 vim-go 和 coc 使用了不同的 `TMPDIR`，并且早在 2020 年 gopls 就通过
[优先使用 XDG\_RUNTIME\_DIR](https://github.com/golang/tools/commit/7470481624a7f4457204e63e016f779d020ba92e) 
解决了这个问题。
至于为什么 vim-go 和 coc 会使用不同的 `TMPDIR` 的疑问仍然未得到解答。
好在有了明确的解决方案：
```
let $XDG_RUNTIME_DIR='/tmp'
```
指定 `XDG_RUNTIME_DIR` 后，
vim-go 和 coc 启动的 gopls 都会从同一个目录检查是否已有 daemon，
进行实现 daemon 共享从而节约资源
