package cmap

import "sync"

// CMap is a concurrency-safe data structure.
// The builtin 'map' in golang doesn't support
// concurrent reads and writes.
// In golang1.9, there is 'sync.Map' in stdlib,
// but it's not fast enough.(https://github.com/golang/go/issues/21035)
// CMap is a little trick that change "lock" to "locks",
// and it supports int64 keys only.
//
// Inspired by https://github.com/orcaman/concurrent-map
// concurrent-map use strings as keys, and use fnv32 to
// divide keys into different buckets
type CMap struct {
	slotCnt int64
	buckets []map[int64]interface{}
	locks   []*sync.RWMutex
}

// Init create all buckets and locks
// cnt is count of buckets, can't be zero
func (m *CMap) Init(cnt uint8) {
	if cnt == 0 {
		cnt = 1
	}
	m.slotCnt = int64(cnt)
	m.buckets = make([]map[int64]interface{}, cnt)
	m.locks = make([]*sync.RWMutex, cnt)
	for i := uint8(0); i < cnt; i++ {
		m.buckets[i] = make(map[int64]interface{})
		m.locks[i] = &sync.RWMutex{}
	}
}

func (m *CMap) Set(key int64, v interface{}) {
	if m.slotCnt == 0 {
		m.Init(1)
	}
	slot := key % m.slotCnt
	l := m.locks[slot]
	l.Lock()
	m.buckets[slot][key] = v
	l.Unlock()
}

func (m *CMap) Get(key int64) (v interface{}, ok bool) {
	if m.slotCnt == 0 {
		m.Init(1)
	}
	slot := key % m.slotCnt
	l := m.locks[slot]
	l.RLock()
	v, ok = m.buckets[slot][key]
	l.RUnlock()
	return
}

func (m *CMap) Remove(key int64) {
	if m.slotCnt == 0 {
		m.Init(1)
	}
	slot := key % m.slotCnt
	l := m.locks[slot]
	l.Lock()
	delete(m.buckets[slot], key)
	l.Unlock()
}
