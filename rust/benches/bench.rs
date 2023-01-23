use bencher::{benchmark_group, benchmark_main, Bencher};
use speco_benchmarks::{run_with, run_without};

fn bench_with(bench: &mut Bencher) {
  bench.iter(|| {
    run_with();
  })
}

fn bench_without(bench: &mut Bencher) {
  bench.iter(|| {
    run_without();
  });
}

benchmark_group!(benches, bench_with, bench_without);
benchmark_main!(benches);
