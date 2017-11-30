package cmap

import (
	"sync"
	"testing"
)

func BenchmarkStdMap_Store(b *testing.B) {
	var std = &sync.Map{}
	b.StartTimer()
	var wg sync.WaitGroup
	for i := b.N; i >= 0; i-- {
		wg.Add(1)
		go func(v int) {
			std.Store(v, v)
			wg.Done()
		}(i)
	}
	wg.Wait()
}

func BenchmarkCMap_Set(b *testing.B) {
	var wg sync.WaitGroup
	m := &CMap{}
	m.Init(10)
	b.StartTimer()
	for i := int64(b.N); i >= 0; i-- {
		wg.Add(1)
		go func(v int64) {
			m.Set(v, v)
			wg.Done()
		}(i)
	}
	wg.Wait()

}

func BenchmarkStdMap_Load(b *testing.B) {
	b.StopTimer()
	var std = &sync.Map{}
	for i := b.N; i >= 0; i-- {
		std.Store(i, i)
	}
	var wg sync.WaitGroup
	b.StartTimer()
	for i := b.N; i >= 0; i-- {
		wg.Add(1)
		go func(v int) {
			std.Load(v)
			wg.Done()
		}(i)
	}
	wg.Wait()
}
func BenchmarkCMap_Get(b *testing.B) {
	b.StopTimer()
	var wg sync.WaitGroup
	m := &CMap{}
	m.Init(10)
	for i := int64(b.N); i >= 0; i-- {
		m.Set(i, i)
	}
	b.StartTimer()
	for i := int64(b.N); i >= 0; i-- {
		wg.Add(1)
		go func(v int64) {
			m.Get(v)
			wg.Done()
		}(i)
	}
	wg.Wait()

}
