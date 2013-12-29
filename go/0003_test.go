package main

import "testing"

func TestFindLargestPrimeFactor(t *testing.T) {
	var in, out int64 = 1, -1
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 2, 2
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 3, 3
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 4, 2
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 5, 5
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}
}
