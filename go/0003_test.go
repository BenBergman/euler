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

	in, out = 10, 5
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 12, 3
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 13, 13
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 13, 13
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 30, 5
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	in, out = 13195, 29
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}

	if testing.Short() {
		t.Skip("Skipping test in short mode.")
	}

	in, out = 600851475143, 6857
	if x := FindLargestPrimeFactor(in); x != out {
		t.Errorf("FindLargestPrimeFactor(%v) = %v, want %v", in, x, out)
	}
}

func BenchmarkFindLargestPrimeFactor(b *testing.B) {
	for i := 0; i < b.N; i++ {
		var in int64 = 600851475143
		FindLargestPrimeFactor(in)
	}
}
