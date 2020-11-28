package main

// ListNode ..
type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeInBetween(list1 *ListNode, a int, b int, list2 *ListNode) *ListNode {
	aParent := &ListNode{Next: list1}
	for i := 0; i < a; i++ {
		aParent = aParent.Next
	}
	bNode := aParent.Next
	aParent.Next = list2
	for i := a; i < b; i++ {
		bNode = bNode.Next
	}
	list2Tail := list2
	for list2Tail.Next != nil {
		list2Tail = list2Tail.Next
	}
	list2Tail.Next = bNode.Next
	return list1
}
