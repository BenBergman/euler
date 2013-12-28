package main

import (
	"fmt"
	"math"
)

func SumOfNumbersDivisibleByThreeAndFive(n int) int {
	sum := 0
	for i := 0; i < n; i++ {
		if math.Mod(float64(i), 3) == 0 || math.Mod(float64(i), 5) == 0 {
			sum += i
		}
	}
	return sum
}

func main() {
	fmt.Printf("%d\n", SumOfNumbersDivisibleByThreeAndFive(1000))
}
