package main

import "math"
import "math/big"

func FindLargestPrimeFactor(n int64) int64 {
	for i := int64(1); i <= n/2; i++ {
		if math.Mod(float64(n), float64(i)) == 0 {
			quotient := big.NewInt(n / i)
			if quotient.ProbablyPrime(40) {
				return quotient.Int64()
			}
		}
	}

	return -1
}
