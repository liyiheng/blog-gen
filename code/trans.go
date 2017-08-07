package main

import "fmt"
import "strconv"

func main() {
	// 210 137
	for {
		i := new(string)
		n, _ := fmt.Scanln(i)
		if n == 0 {
			return
		}
		o := make([]byte, 0)
		for _, r := range *i {
			o = strconv.AppendQuoteRune(o, r)
			o = append(o, 210, 137)
		}
		fmt.Println(string(o))

	}
}
