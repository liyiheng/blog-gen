package mylib

import "container/list"

// A Java-like list struct
// just a demo, can't ensure performance,
// using slice instead of list.List might be better
// TODO : 1. A slice implementation of ListList. 2. Benchmark
type ListList struct {
	list.List
}

func (a *ListList) Add(v interface{}) {
	a.List.PushBack(v)
}
func (a *ListList) RemoveAt(index int) interface{} {
	l := a.List.Len()
	if index < 0 || index >= l {
		return nil
	}
	if index < l/2 {
		e := a.List.Front()
		for i := 0; i < index; i++ {
			e = e.Next()
		}
		return a.List.Remove(e)

	} else {
		e := a.List.Back()
		t := l - index - 1
		for i := 0; i < t; i++ {
			e = e.Prev()
		}
		return a.List.Remove(e)
	}
}
func (a *ListList) Get(index int) interface{} {
	l := a.List.Len()
	if index < 0 || index >= l {
		return nil
	}
	if index < l/2 {
		e := a.List.Front()
		for i := 0; i < index; i++ {
			e = e.Next()
		}
		return e.Value

	} else {
		e := a.List.Back()
		t := l - index - 1
		for i := 0; i < t; i++ {
			e = e.Prev()
		}
		return e.Value
	}
}
func (a *ListList) IndexOf(e *list.Element) int {
	l := a.Len()
	if l == 0 {
		return -1
	}
	tmp := a.Front()
	for i := 0; i < l; i++ {
		if tmp == e {
			return i
		}
		tmp = tmp.Next()

	}
	return -1
}
func (a *ListList) LastIndexOf(e *list.Element) int {
	l := a.Len()
	if l == 0 {
		return -1
	}
	tmp := a.Back()
	for i := l - 1; i >= 0; i-- {
		if tmp == e {
			return i
		}
		tmp = tmp.Prev()
	}
	return -1
}
func (a *ListList) ToSlice() []interface{} {
	l := a.Len()
	if l == 0 {
		return make([]interface{}, 0)
	}
	slc := make([]interface{}, l)
	tmp := a.Front()
	for i := 0; i < l; i++ {
		slc[i] = tmp.Value
		tmp = tmp.Next()
	}
	return slc
}

//type IList interface {
//	Add(v interface{})
//	Remove(v interface{})
//	RemoveAt(index int) interface{}
//	Get(index int) interface{}
//}

// A slice implementation of Java-ListList
// Extremely early implementation
// TODO Slice gotchas
type SliceList struct {
	data []interface{}
}

func (l *SliceList) Init() {
	l.data = make([]interface{}, 0)
}

func (l *SliceList) Add(v interface{}) {
	l.data = append(l.data, v)
}

func (l *SliceList) IndexOf(e interface{}) int {
	for i, v := range l.data {
		if v == e {
			return i
		}
	}
	return -1
}

func (l *SliceList) Remove(v interface{}) {
	i := l.IndexOf(v)
	if i != -1 {
		l.RemoveAt(i)
	}
}

func (l *SliceList) RemoveAt(i int) {
	size := len(l.data)
	part1 := l.data[0:i]
	part2 := l.data[i+1 : size]
	l.data = append(part1, part2...)
}

func (l *SliceList) Get(index int) interface{} {
	return l.data[index]
}

func (l *SliceList) Insert(index int, v interface{}) {
	// part1 := l.data[0:index]
	part1 := l.data[0:index:index]
	part2 := l.data[index:]
	part1 = append(part1, v)
	l.data = append(part1, part2...)
}

func (l *SliceList) Size() int {
	return len(l.data)
}
