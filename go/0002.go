package main

import "math"

func FibonacciGenerator(max int) []int {
	fib := []int{0}
	a, b := 0, 1

	for b < max {
		fib = append(fib, b)
		x := a + b
		a = b
		b = x
	}

	return fib
}

func SumOfEvenNumbers(slice []int) int {
	sum := 0

	for _, x := range slice {
		if math.Mod(float64(x), 2) == 0 {
			sum += x
		}
	}

	return sum
}
