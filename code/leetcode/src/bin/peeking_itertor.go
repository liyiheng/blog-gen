package main

// Iterator -
type Iterator struct {
}

func (*Iterator) hasNext() bool {
	return false
}
func (*Iterator) next() int {
	return 0
}

// ------------------------------------------

type PeekingIterator struct {
	origin  *Iterator
	nextVal *int
}

func Constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{origin: iter, nextVal: nil}
}

func (iter *PeekingIterator) hasNext() bool {
	return iter.nextVal != nil || iter.origin.hasNext()
}

func (iter *PeekingIterator) next() int {
	if iter.nextVal != nil {
		v := *iter.nextVal
		iter.nextVal = nil
		return v
	}
	return iter.origin.next()
}

func (iter *PeekingIterator) peek() int {
	if iter.nextVal == nil {
		v := iter.origin.next()
		iter.nextVal = &v
	}
	return *iter.nextVal
}
