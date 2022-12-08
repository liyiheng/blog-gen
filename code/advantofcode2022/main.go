package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	day07()
}

type node struct {
	Name     string           `json:"name,omitempty"`
	IsDir    bool             `json:"-"`
	Size     int              `json:"-"`
	Children map[string]*node `json:"children"`
	Parent   *node            `json:"-"`
}

func (n *node) GetSize(f func(*node)) int {
	if !n.IsDir {
		f(n)
		return n.Size
	}
	if n.Size > 0 {
		f(n)
		return n.Size
	}
	total := 0
	for _, c := range n.Children {
		s := c.GetSize(f)
		total += s
	}
	n.Size = total
	f(n)
	return total

}

func day07() {
	dat, err := ioutil.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	rootParent := &node{
		Name:     "rootParent",
		IsDir:    true,
		Size:     -1,
		Children: make(map[string]*node),
	}
	rootParent.Children["/"] = &node{Name: "/", IsDir: true, Children: make(map[string]*node)}
	current := rootParent
	for _, lineDat := range bytes.Split(dat, []byte("\n")) {
		if len(lineDat) == 0 {
			continue
		}
		line := string(lineDat)
		if strings.HasPrefix(line, "$ cd") {
			name := strings.ReplaceAll(line, "$ cd ", "")
			if name == ".." {
				current = current.Parent
				continue
			}
			current = current.Children[name]
			continue
		}
		if strings.HasPrefix(line, "$ ls") {
			continue
		}
		segs := strings.Split(line, " ")
		n := &node{
			Parent: current,
			Name:   segs[1],
			IsDir:  segs[0] == "dir",
		}
		if segs[0] != "dir" {
			n.Size, _ = strconv.Atoi(segs[0])
		} else {
			n.Children = make(map[string]*node)
		}
		current.Children[segs[1]] = n
	}
	total := 70000000
	needSpace := 30000000
	sizes := make([]int, 0)
	ans := 0
	used := rootParent.Children["/"].GetSize(func(n *node) {
		sizes = append(sizes, n.Size)
		if n.Size <= 100000 && n.IsDir {
			ans += n.Size
		}
	})
	fmt.Println("day07 part1:", ans)
	needToFree := needSpace - (total - used)
	sort.Sort(sort.IntSlice(sizes))
	for _, v := range sizes {
		if v >= needToFree {
			fmt.Println("day07 part2:", v)
			break
		}

	}

}
