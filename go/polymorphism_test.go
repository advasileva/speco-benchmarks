package benchmarks

import "testing"

func BenchmarkMeasureWithPoly(b *testing.B) {
	c := Cart{Book{"1984"}, 10}
	c.p = Movie{"Godfather"}
	for i := 0; i < 10000000000; i++ {
		c.total()
	}
}

func BenchmarkMeasureWithoutPoly(b *testing.B) {
	c1 := Cart1{Book{"1984"}, 10}
	c2 := c1.with(Movie{"Godfather"})
	for i := 0; i < 10000000000; i++ {
		c2.total()
	}
}
