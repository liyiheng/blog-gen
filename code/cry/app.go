package main

import (
	"bufio"
	"crypto/aes"
	"crypto/md5"
	"crypto/rand"
	"encoding/base64"
	"flag"
	"io"
	"log"
	"os"
	"time"
	"unsafe"

	"gopkg.in/cheggaaa/pb.v1"
)

const (
	modeEncrypt  = 0
	modeDecrypt  = 1
	minKeyLength = 6
)

type params struct {
	src  string
	dst  string
	mode int
	key  string
}

func parseArgs() *params {
	src := flag.String("I", "", "Input file")
	k := flag.String("K", "", "Key, use a random string as key if not provided")
	dst := flag.String("O", "result.dat", "File to output")
	mode := flag.Int("m", 0, "0. Encrypt; 1. Decrypt")
	flag.Parse()
	if *k == "" {
		tmp := make([]byte, 30)
		rand.Read(tmp)
		*k = base64.RawStdEncoding.EncodeToString(tmp)
	}
	return &params{
		src:  *src,
		dst:  *dst,
		mode: *mode,
		key:  *k,
	}
}
func checkArgs(p *params) bool {
	if !fileExist(p.src) {
		log.Println(p.src, "not found")
		flag.Usage()
		return false
	}
	if fileExist(p.dst) {
		log.Println(p.dst, "already exists")
		flag.Usage()
		return false
	}
	if len(*((*[]byte)(unsafe.Pointer(&p.key)))) < minKeyLength {
		log.Println("Key can't be shorted than", minKeyLength)
		flag.Usage()
		return false
	}
	return true
}

func main() {
	p := parseArgs()
	if ok := checkArgs(p); !ok {
		os.Exit(1)
	}
	switch p.mode {
	case modeEncrypt:
		encrypt(p, true)
	case modeDecrypt:
		encrypt(p, false)
	default:
		log.Printf("Unsupported mode %d", p.mode)
		flag.Usage()
	}
}

func encrypt(p *params, encrypt bool) {
	key := md5.Sum([]byte(p.key))
	c, e := aes.NewCipher(key[:])
	checkErr(e)
	src, e := os.Open(p.src)
	checkErr(e)
	defer src.Close()
	dst, e := os.Create(p.dst)
	checkErr(e)
	defer dst.Close()
	info, e := src.Stat()
	checkErr(e)

	reader := bufio.NewReader(src)
	writer := bufio.NewWriter(dst)
	bs := c.BlockSize()
	totalSize := info.Size()
	if !encrypt && int(totalSize)%bs != 0 {
		log.Println("Wraning: Incorrect data")
	}
	current := 0
	bar := pb.New(int(totalSize))
	bar.SetUnits(pb.U_BYTES)
	bar.SetRefreshRate(time.Millisecond * 300)
	bar.Start()
	buf := make([]byte, bs, bs)
	for {
		n, e := reader.Read(buf[:])
		if n == 0 || e != nil {
			if e != io.EOF {
				log.Println(e)
			}
			break
		}
		if n < bs {
			buf = pad(buf[:n])
		}
		if encrypt {
			c.Encrypt(buf, buf)
		} else {
			c.Decrypt(buf, buf)
			if n < bs {
				buf = unpad(buf[:n])
			}
		}
		writer.Write(buf)
		current += n
		bar.Set(current)
	}
	writer.Flush()
	log.Printf("Done. Key is:%s", p.key)
}

func fileExist(name string) bool {
	_, e := os.Stat(name)
	return !os.IsNotExist(e)
}

func checkErr(e error) {
	if e != nil {
		log.Println(e)
		os.Exit(1)
	}
}

func pad(src []byte) []byte {
	//padding := aes.BlockSize - len(src)%aes.BlockSize
	//b := bytes.Repeat([]byte{byte(padding)}, padding)
	//return append(src, b...)
	return src
}

func unpad(src []byte) []byte {
	//length := len(src)
	//b := int(src[length-1])
	//return src[:(length - b)]
	return src
}
