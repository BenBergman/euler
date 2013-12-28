package main

import (
	"fmt"
	"math"
)

func main() {
	sum := 0
	for i := 0; i < 1000; i++ {
		if math.Mod(float64(i), 3) == 0 || math.Mod(float64(i), 5) == 0 {
			sum += i
		}
	}
	fmt.Printf("%d\n", sum)
}
