package main

import "testing"

func TestSumOfNumbersDivisibleByThreeAndFive(t *testing.T) {
	const in, out = 1000, 233168
	if x := SumOfNumbersDivisibleByThreeAndFive(in); x != out {
		t.Errorf("SumOfNumbersDivisibleByThreeAndFive(%v) = %v, want %v", in, x, out)
	}
}
