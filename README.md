# Benchmarks

### Results

Execution time per 10 billion operations

Case | Polimorphic | No polimorphism | Performance gain
------ | ------ | ------ | ------ 
Java | 8.648s | 6.360s | +35.97%
C++ | 32.642s | 29.822s | +9.45%
Go |  46.001s | 24.122s | +90.7%
Python |  2920.821s | 2890.683s | +1%
JavaScript | 13.970s | 13.514s | +3.4%
Rust | 164.812s | 157.509s | +11.7%

Machine: Windows 10 21H2 and Intel(R) Core(TM) i5-8265U at 1.60GHz

### Usage

To build Java project:
```bash
$ make build
```

To run Java, Go, Python, Rust and JS benchmarks locally:
```bash
$ make run
```
