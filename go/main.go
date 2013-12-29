package main

import "fmt"

func main() {
	fmt.Printf("0001: %v\n", SumOfNumbersDivisibleByThreeAndFive(1000))
	fmt.Printf("0002: %v\n", SumOfEvenNumbers(FibonacciGenerator(4000000)))
	fmt.Printf("0003: %v\n", FindLargestPrimeFactor(600851475143))
}
