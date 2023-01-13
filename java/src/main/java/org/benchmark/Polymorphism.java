package org.benchmark;

import org.benchmark.polymorphism.*;
import org.openjdk.jmh.annotations.*;
import org.openjdk.jmh.infra.Blackhole;

@BenchmarkMode(Mode.AverageTime)
@Fork(10)
@Warmup(iterations = 0)
@Measurement(iterations = 1)
public class Polymorphism {
    @Benchmark
    public void measureWith(Blackhole bh) {
        Cart c = new Cart(new Book("1984"));
        c.p = new Movie("Godfather");
        for (long i = 0; i < 10000000000L; i++) {
            bh.consume(c.total());
        }
    }

    @Benchmark
    public void measureWithout(Blackhole bh) {
        Cart1 c1 = new Cart1(new Book("1984"));
        Cart2 c2 = c1.with(new Movie("Godfather"));
        for (long i = 0; i < 10000000000L; i++) {
            bh.consume(c2.total());
        }
    }
}
