package main // パッケージ宣言（必須）

import (
	"fmt"
	"math/rand"
	"slices"
)

func main() { // エントリーポイント
	fmt.Println("Hello, Go!")

	randoms := generateRandomSlice(10)
	input := generateRandomSlice(3)

	tries := []int{0, 1, 2}
	cnt := 0

	fmt.Println("randoms:", randoms)

	for len(tries) > 0 {
		cnt++
		fmt.Println("input:", input)
		for i, t := range tries {
			if slices.Contains(randoms[:], input[t]) {
				tries = append(tries[:i], tries[i+1:]...)
			} else {
				input[t] = rand.Intn(100)
			}
		}
		fmt.Println("回数:", cnt)
		fmt.Println("一致した数:", len(input)-len(tries))
	}

	fmt.Println("やった回数:", cnt)
}

func generateRandomSlice(n int) []int {
	slice := make([]int, n)
	for i := range slice {
		slice[i] = rand.Intn(100)
	}
	return slice
}
