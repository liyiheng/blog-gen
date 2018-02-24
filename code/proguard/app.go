package main

import (
	"bufio"
	"log"
	"os"
	"time"

	"gopkg.in/cheggaaa/pb.v1"
)

const (
	myMagic  = 66
	magicCnt = 6
)

func main() {
	if len(os.Args) < 2 {
		return
	}
	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Panicln(err.Error())
	}
	s, err := f.Stat()
	if err != nil {
		log.Panicln(err.Error())
	}
	defer f.Close()

	bar := pb.StartNew(int(s.Size()))
	bar.SetRefreshRate(time.Millisecond * 500)
	bar.SetUnits(pb.U_BYTES)

	mixed := obfuscated(f)

	dstName := os.Args[1] + ".recovered"
	mask := byte(myMagic)
	realSize := s.Size() - magicCnt

	if !mixed {
		mask = 256 - myMagic
		dstName = os.Args[1] + ".mixed"
		realSize += magicCnt
	}
	dstFile, err := os.Create(dstName)
	if err != nil {
		log.Panicln(err.Error())
	}

	r := bufio.NewReader(f)
	w := bufio.NewWriter(dstFile)

	buf := make([]byte, 1024*1024*10)
	counter := int64(0)

	for {
		cnt, _ := r.Read(buf[:])
		if cnt == 0 {
			break
		}
		for i, v := range buf[:cnt] {
			buf[i] = byte(int(v+mask) % 256)
		}
		counter += int64(cnt)
		bar.Add64(int64(cnt))
		if counter >= realSize {
			cnt -= int(counter - realSize)
			if cnt > 0 {
				w.Write(buf[:cnt])
			}
			break
		} else {
			w.Write(buf[:cnt])
		}

	}
	if !mixed {
		mag := make([]byte, magicCnt)
		for i := range mag {
			mag[i] = myMagic
		}
		w.Write(mag)
	}
	w.Flush()
}

func obfuscated(f *os.File) bool {
	offset, err := f.Seek(-magicCnt, 2)
	if err != nil {
		log.Panicln(err.Error())
	}
	println(offset)
	magic := make([]byte, magicCnt)
	_, err = f.ReadAt(magic, offset)
	if err != nil {
		log.Panicln(err.Error())
	}
	f.Seek(0, 0)

	for _, v := range magic {
		if v != myMagic {
			return false
		}
	}
	return true
}
