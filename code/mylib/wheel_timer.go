package mylib

import (
	"errors"
	"fmt"
	"runtime/debug"
	"sync"
	"time"
)

// Inspired by HashedWheelTimer in Netty
type WheelTimer struct {
	step     time.Duration // check duration, eg: 1s
	timeout  time.Duration // timeout duration, eg: 1min
	doAction func(int64)   // action when timeout/expired, eg: logout
	id2slot  map[int64]int // id --> where it is
	sync.Mutex
	circle *cycleList
	stop   chan struct{}
}

type cycleList struct {
	currentSlot int
	body        []*Int64Set
}

func (cl *cycleList) getPreviousSlot() int {
	l := len(cl.body)
	if l == 0 {
		return 0
	}
	if cl.currentSlot == 0 {
		return l - 1
	}
	return cl.currentSlot - 1
}
func (cl *cycleList) getNextSlot() int {
	l := len(cl.body)
	if l == 0 {
		return 0
	}
	if l-1 == cl.currentSlot {
		return 0
	}
	return cl.currentSlot + 1
}
func (cl *cycleList) takeStep() {
	cl.currentSlot = cl.getNextSlot()
}

// Initialize the timer
func (wt *WheelTimer) Init(step, timeout time.Duration, doWhenExpired func(int64)) error {
	n := timeout % step
	if n != 0 {
		// timeout would not work
		// actually (timeout/step)*step works.
		// eg.:	Init(3s,10s,logout)
		//	there will be 3 slots,
		//	3s * 3 == 9s
		return errors.New("timeout%step must be zero")
	}

	wt.step = step
	wt.timeout = timeout
	wt.doAction = doWhenExpired
	wt.stop = make(chan struct{}, 0)
	wt.id2slot = make(map[int64]int)
	cl := &cycleList{}
	cl.body = make([]*Int64Set, timeout/step)

	for i := range cl.body {
		s := &Int64Set{}
		s.Init()
		cl.body[i] = s
	}

	wt.circle = cl
	return nil
}

// Update or register an id in the timer
//
// If an id is never updated since last time in
// expire duration which set in Init() function,
// means it's expired, and doWhenExpired(id)
// will be called
func (wt *WheelTimer) Update(id int64) {
	wt.Lock()
	defer wt.Unlock()
	if v, ok := wt.id2slot[id]; ok {
		// remove id from the old set
		oldSet := wt.circle.body[v]
		oldSet.Remove(id)
	}
	// get the set of current slot
	i := wt.circle.currentSlot
	slot := wt.circle.body[i]
	// add id in current set
	slot.Add(id)
	// record down where it is
	wt.id2slot[id] = i
}

// Stop the timer
func (wt *WheelTimer) Stop() {
	wt.stop <- struct{}{}
}

// Start the timer
func (wt *WheelTimer) Start() {
	go func() {
		for {
			select {
			case <-wt.stop:
				return
			case <-time.Tick(wt.step):
				wt.circle.takeStep()
				wt.process()
			}
		}
	}()
}

// execute all expired IDs
func (wt *WheelTimer) process() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println(debug.Stack())
		}
	}()
	// get all ids in current slot
	expiredIDs := wt.circle.body[wt.circle.currentSlot]
	go func(ids []int64) {
		for _, id := range ids {
			wt.doAction(id)
		}
	}(expiredIDs.ToSlice())
	expiredIDs.RemoveAll()
}
