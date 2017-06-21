package mylib

type IntSet struct {
	data map[int]struct{}
}

func (s *IntSet) Init() {
	s.data = make(map[int]struct{})
}
func (s *IntSet) Size() int {
	return len(s.data)
}
func (s *IntSet) Add(n int) bool {
	if _, ok := s.data[n]; ok {
		return false
	}
	s.data[n] = struct{}{}
	return true
}

func (s *IntSet) Remove(n int) bool {
	if _, ok := s.data[n]; ok {
		delete(s.data, n)
		return true
	}
	return false
}
func (s *IntSet) Has(n int) bool {
	_, ok := s.data[n]
	return ok
}
func (s *IntSet) IsEmpty() bool {
	return len(s.data) == 0
}
func (s *IntSet) GetMap() map[int]struct{} {
	return s.data
}

func (s *IntSet) ToSlice() []int {
	ints := make([]int, 0)
	for i := range s.data {
		ints = append(ints, i)
	}
	return ints
}

func (s *IntSet) Clone() *IntSet {
	m := make(map[int]struct{})
	for i := range s.data {
		m[i] = struct{}{}
	}
	return &IntSet{data: m}
}

type Int64Set struct {
	data map[int64]struct{}
}

func (s *Int64Set) Init() {
	s.data = make(map[int64]struct{})
}
func (s *Int64Set) RemoveAll() {
	s.Init()
}
func (s *Int64Set) Size() int {
	return len(s.data)
}
func (s *Int64Set) Add(n int64) bool {
	if _, ok := s.data[n]; ok {
		return false
	}
	s.data[n] = struct{}{}
	return true
}

func (s *Int64Set) Remove(n int64) bool {
	if _, ok := s.data[n]; ok {
		delete(s.data, n)
		return true
	}
	return false
}
func (s *Int64Set) Has(n int64) bool {
	_, ok := s.data[n]
	return ok
}
func (s *Int64Set) IsEmpty() bool {
	return len(s.data) == 0
}
func (s *Int64Set) GetMap() map[int64]struct{} {
	return s.data
}

func (s *Int64Set) ToSlice() []int64 {
	ints := make([]int64, 0)
	for i := range s.data {
		ints = append(ints, i)
	}
	return ints
}

func (s *Int64Set) Clone() *Int64Set {
	m := make(map[int64]struct{})
	for i := range s.data {
		m[i] = struct{}{}
	}
	return &Int64Set{data: m}
}
