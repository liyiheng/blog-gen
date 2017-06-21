package mylib

import (
	"fmt"
	"testing"
	"time"
)

func TestWheelTimer(t *testing.T) {
	w := WheelTimer{}
	w.Init(time.Second, time.Second*30, func(id int64) {
		fmt.Println(id, "expired")
	})
	w.Start()
	fmt.Println("Timer started")
	for i := 0; i < 5; i++ {
		w.Update(int64(i))
	}
	time.Sleep(time.Second * 10)
	w.Update(int64(3))
	fmt.Println("3 updated")
	for i := 5; i < 10; i++ {
		w.Update(int64(i))
	}
	fmt.Println("5,6,7,8,9 updated")
	time.Sleep(time.Second * 10)
	time.Sleep(time.Second * 20)
	w.Stop()
}
