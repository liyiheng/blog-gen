---
date: 2017-11-13
title: "google/gops源码分析"
draft: false
categories:
  - golang
tags:
  - golang
thumbnailImagePosition: left
---



发现了一个很实用的工具：gops<br>

> gops is a command to list and diagnose Go processes currently running on your system.

觉得判断一个进程是否是go程序挺有意思，不妨看看它的大致实现。<br>

<!--more-->

### 0x0000
先看看效果
```sh
$ go get -u github.com/google/gops
$ gops
14047 14020 gops               go1.8.4 /home/liyiheng/projects/bin/gops
7596  3852  shadowsocks-local  go1.8.1 /home/liyiheng/projects/bin/shadowsocks-local
```

### 0x0001
大致结构如下：
```sh
$ tree
.
├── agent
│   ├── agent.go
│   └── agent_test.go
├── cmd.go
├── examples
│   └── hello
│       └── main.go
├── Gopkg.lock
├── Gopkg.toml
├── goprocess
│   └── gp.go
├── internal
│   └── internal.go
├── LICENSE
├── main.go
├── main_test.go
├── README.md
├── signal
│   └── signal.go
└── vendor ...
```
主要是为了看看如何判断一个进程是否是golang程序，暂时忽略诊断相关的代码。<br>

### 0x0002
顺着`main()`函数找到了`processes()`函数，继而找到`goprocess.FindAll()`函数：
```golang
// FindAll returns all the Go processes currently running on this host.
func FindAll() []P {
	var results []P

	pss, err := ps.Processes()
	if err != nil {
		return results
	}

	var wg sync.WaitGroup
	wg.Add(len(pss))

	for _, pr := range pss {
		pr := pr
		go func() {
			defer wg.Done()

			path, version, agent, ok, err := isGo(pr)
			if err != nil {
				// TODO(jbd): Return a list of errors.
			}
			if !ok {
				return
			}
			results = append(results, P{
				PID:          pr.Pid(),
				PPID:         pr.PPid(),
				Exec:         pr.Executable(),
				Path:         path,
				BuildVersion: version,
				Agent:        agent,
			})
		}()
	}
	wg.Wait()
	return results
}
```
在这个函数中看到了想看的东西：
```golang
pss, err := ps.Progresses()
// .... 
for _, pr := range pss {
	pr := pr
	go func(){
		 // ...
		 path, version, agent, ok, err := isGo(pr)
		 // ...
	}()
}
```
`Progresses()`获取所有进程信息，`isGo()`判断是否是golang程序。<br>
此外，还有意外收获:`pr := pr`, 这很巧妙,比`go func(a A){}(a)`优雅。<br>

### 0x0003
`Progresses()`中是`progresses()函数`，不同系统下的`progresses`有不同的实现，这里只看Linux的。<br>
```golang
func processes() ([]Process, error) {
	d, err := os.Open("/proc")
	if err != nil {
		return nil, err
	}
	defer d.Close()

	results := make([]Process, 0, 50)
	for {
		fis, err := d.Readdir(10)
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, err
		}

		for _, fi := range fis {
			// We only care about directories, since all pids are dirs
			if !fi.IsDir() {
				continue
			}

			// We only care if the name starts with a numeric
			name := fi.Name()
			if name[0] < '0' || name[0] > '9' {
				continue
			}

			// From this point forward, any errors we just ignore, because
			// it might simply be that the process doesn't exist anymore.
			pid, err := strconv.ParseInt(name, 10, 0)
			if err != nil {
				continue
			}

			p, err := newUnixProcess(int(pid))
			if err != nil {
				continue
			}

			results = append(results, p)
		}
	}

	return results, nil
}
```
遍历/proc下的文件获取所有进程pid，再根据pid对应目录下的文件创建含有对应进程信息的结构体
```golang
// Refresh reloads all the data associated with this process.
func (p *UnixProcess) Refresh() error {
	statPath := fmt.Sprintf("/proc/%d/stat", p.pid)
	dataBytes, err := ioutil.ReadFile(statPath)
	if err != nil {
		return err
	}

	// First, parse out the image name
	data := string(dataBytes)
	binStart := strings.IndexRune(data, '(') + 1
	binEnd := strings.IndexRune(data[binStart:], ')')
	p.binary = data[binStart : binStart+binEnd]

	// Move past the image name and start parsing the rest
	// The name here might not be the full name
	data = data[binStart+binEnd+2:]
	_, err = fmt.Sscanf(data,
		"%c %d %d %d",
		&p.state,
		&p.ppid,
		&p.pgrp,
		&p.sid)

	return err
}
```

### 0x0004
重点来了
```golang
// isGo looks up the runtime.buildVersion symbol
// in the process' binary and determines if the process
// if a Go process or not. If the process is a Go process,
// it reports PID, binary name and full path of the binary.
func isGo(pr ps.Process) (path, version string, agent, ok bool, err error) {
	if pr.Pid() == 0 {
		// ignore system process
		return
	}
	path, err = pr.Path()
	if err != nil {
		return
	}
	var versionInfo goversion.Version
	versionInfo, err = goversion.ReadExe(path)
	if err != nil {
		return
	}
	ok = true
	version = versionInfo.Release
	pidfile, err := internal.PIDFile(pr.Pid())
	if err == nil {
		_, err := os.Stat(pidfile)
		agent = err == nil
	}
	return path, version, agent, ok, nil
}
```
显然，核心是`goversion.ReadExe(path)`，跳进去，
```golang
// ReadExe reports information about the Go version used to build
// the program executable named by file.
func ReadExe(file string) (Version, error) {
	var v Version
	f, err := openExe(file)
	if err != nil {
		return v, err
	}
	defer f.Close()
	isGo := false
	for _, name := range f.SectionNames() {
		if name == ".note.go.buildid" {
			isGo = true
		}
	}
	syms, symsErr := f.Symbols()
	isGccgo := false
	for _, sym := range syms {
		name := sym.Name
		if name == "runtime.main" || name == "main.main" {
			isGo = true
		}
		if strings.HasPrefix(name, "runtime.") && strings.HasSuffix(name, "$descriptor") {
			isGccgo = true
		}
		if name == "runtime.buildVersion" {
			isGo = true
			release, err := readBuildVersion(f, sym.Addr, sym.Size)
			if err != nil {
				return v, err
			}
			v.Release = release

		}
		if strings.Contains(name, "_Cfunc__goboringcrypto_") {
			v.BoringCrypto = true
		}
		for _, s := range standardCryptoNames {
			if strings.Contains(name, s) {
				v.StandardCrypto = true
			}
		}
	}

	if *debugMatch {
		v.Release = ""
	}
	if v.Release == "" {
		g, release := readBuildVersionX86Asm(f)
		if g {
			isGo = true
			v.Release = release
		}
	}
	if isGccgo && v.Release == "" {
		isGo = true
		v.Release = "gccgo (version unknown)"
	}
	if !isGo && symsErr != nil {
		return v, symsErr
	}

	if !isGo {
		return v, errors.New("not a Go executable")
	}
	if v.Release == "" {
		v.Release = "unknown Go version"
	}
	return v, nil
}
```
整个函数都是围绕着`f, err := openExe(file)`中的`f`，先看看`openExe`函数。<br>
`func openExe(file string) (exe,error) {}`<br>
可见， `f`的类型是`exe`。
```golang
type exe interface {
	AddrSize() int // bytes
	ReadData(addr, size uint64) ([]byte, error)
	Symbols() ([]sym, error)
	SectionNames() []string
	Close() error
	ByteOrder() binary.ByteOrder
	Entry() uint64
}
```
再看`openExe`的函数体：
```golang
// 打开传入的文件目录对应的文件
f, err := os.Open(file)
if err != nil {
	return nil, err
}
// 读取前16个字节
data := make([]byte, 16)
if _, err := io.ReadFull(f, data); err != nil {
	return nil, err
}
// 将文件指针移回开头(细节见APUE，APUE是本神书，强烈安利)
f.Seek(0, 0)
// 判断是否是ELF文件
// Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
// ELF文件的魔数第一个字节是7F，接下来三个分别是ELF
if bytes.HasPrefix(data, []byte("\x7FELF")) {
	// 创建一个elfExe结构体并返回其指针
	e, err := elf.NewFile(f)
	if err != nil {
		f.Close()
		return nil, err
	}
	return &elfExe{f, e}, nil
}
if bytes.HasPrefix(data, []byte("MZ")) {
	e, err := pe.NewFile(f)
	if err != nil {
		f.Close()
		return nil, err
	}
	return &peExe{f, e}, nil
}
if bytes.HasPrefix(data, []byte("\xFE\xED\xFA")) || bytes.HasPrefix(data[1:], []byte("\xFA\xED\xFE")) {
	e, err := macho.NewFile(f)
	if err != nil {
		f.Close()
		return nil, err
	}
	return &machoExe{f, e}, nil
}
return nil, fmt.Errorf("unrecognized executable format")
```
因此，正常情况下`openExe`返回的`f`是`*elfExe, *peExe, *machoExe`中的一种。<br>

### 0x0005
继续看`ReadExe`。<br>
先遍历`f.SectionNames()`的返回值，如果其中含有“.note.go.buildid”，则为golang程序。<br>
elfExe的SectionNames()实现为，返回`File`的`Section`字段中每一个元素的`Name`。<br>
elfExe的Symbols()实现为：直接调用`*File`的`Symbols()`方法，从返回的数据中取出需要的并返回。<br>
接下来遍历`f.Symbols`返回的数据。<br>
```golang
for _, sym := range syms {
	name := sym.Name
	if name == "runtime.main" || name == "main.main" {
		isGo = true
	}
	if strings.HasPrefix(name, "runtime.") && strings.HasSuffix(name, "$descriptor") {
		isGccgo = true
	}
	if name == "runtime.buildVersion" {
		isGo = true
		release, err := readBuildVersion(f, sym.Addr, sym.Size)
		if err != nil {
			return v, err
		}
		v.Release = release
	}
	if strings.Contains(name, "_Cfunc__goboringcrypto_") {
		v.BoringCrypto = true
	}
	for _, s := range standardCryptoNames {
		if strings.Contains(name, s) {
			v.StandardCrypto = true
		}
	}
}
```

### 0x0006
大致上，gops是根据进程对应的可执行文件的`Section`和`Symbol`来判断该文件是否是golang程序。<br>


### 0x0007
关于elf、macho、pe，简介如下:

- [Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) In computing, the Executable and Linkable Format (ELF, formerly named Extensible Linking Format), is a common standard file format for executable files, object code, shared libraries, and core dumps. First published in the specification for the application binary interface (ABI) of the Unix operating system version named System V Release 4 (SVR4), and later in the Tool Interface Standard, it was quickly accepted among different vendors of Unix systems. In 1999, it was chosen as the standard binary file format for Unix and Unix-like systems on x86 processors by the 86open project. By design, ELF is flexible, extensible, and cross-platform, not bound to any given central processing unit (CPU) or instruction set architecture. This has allowed it to be adopted by many different operating systems on many different hardware platforms.
- [Mach-O](https://en.wikipedia.org/wiki/Mach-O) Mach-O, short for Mach object file format, is a file format for executables, object code, shared libraries, dynamically-loaded code, and core dumps. A replacement for the a.out format, Mach-O offers more extensibility and faster access to information in the symbol table. Mach-O is used by most systems based on the Mach kernel. NeXTSTEP, macOS, and iOS are examples of systems that have used this format for native executables, libraries and object code.
- [Portable Executable](https://en.wikipedia.org/wiki/Portable_Executable) The Portable Executable (PE) format is a file format for executables, object code, DLLs, FON Font files, and others used in 32-bit and 64-bit versions of Windows operating systems. The PE format is a data structure that encapsulates the information necessary for the Windows OS loader to manage the wrapped executable code. This includes dynamic library references for linking, API export and import tables, resource management data and thread-local storage (TLS) data. On NT operating systems, the PE format is used for EXE, DLL, SYS (device driver), and other file types. The Extensible Firmware Interface (EFI) specification states that PE is the standard executable format in EFI environments.
