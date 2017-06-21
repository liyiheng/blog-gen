package mylib

import (
	"fmt"
	"testing"
)

func TestArrayList_RemoveAt(t *testing.T) {
	l := &ListList{}

	l.Init()
	l.PushBack("a")
	l.PushBack("b")
	l.PushBack("c")
	l.PushBack("d")

	length := l.Len()
	e := l.List.Front()
	for i := 0; i < length; i++ {
		fmt.Println(e.Value)
		e = e.Next()
	}
	fmt.Println("-------------------")
	fmt.Println(l.RemoveAt(-1))
	fmt.Println(l.RemoveAt(100))
	fmt.Println(l.RemoveAt(2))
	fmt.Println(l.ToSlice())
}
func TestSliceList(t *testing.T) {
	l := SliceList{}
	l.Init()
	l.Add("first")
	l.Add("2nd")
	l.Add("3rd")
	l.Add("4th")
	l.Add("5th")
	l.Add("6th")
	l.Add("7th")
	l.RemoveAt(4)
	fmt.Println("data:", l.data)

	if l.Get(1) != "2nd" {
		t.Error(l.Get(1), "should be \"2nd\"")
	}
	l.RemoveAt(0)
	if l.Get(1) != "3rd" {
		t.Error(l.Get(1), "should be \"3rd\"")
	}
	fmt.Println("data:", l.data)
	l.Insert(0, "pos0")
	l.Insert(0, "pos0new")
	fmt.Println("data:", l.data,"\nsize:",l.Size())


}
