package main

import "testing"

func TestProblemTwo(t *testing.T) {
	const in, out = 4000000, 4613732
	if x := SumOfEvenNumbers(FibonacciGenerator(in)); x != out {
		t.Errorf("SumOfEvenNumbers(FibonacciGenerator(%v)) = %v, want %v", in, x, out)
	}
}
