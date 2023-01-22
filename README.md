# Benchmarks

*Alena: I'll add a description later.*

### Results

Case | Polimorphic | No polimorphism | Performance gain
------ | ------ | ------ | ------ 
Java | 8.648s | 6.360s | +35.97%
C++ | 32.642s | 29.822s | +9.45%
Go |  46.001s | 24.122s | +90.7%
Python |  2306.132s | 2867.444s | -19.6%

Machine: Windows 10 21H2 and Intel(R) Core(TM) i5-8265U at 1.60GHz

### Usage

To build Java project:
```bash
$ make build
```

To run Java and Go benchmarks locally:
```bash
$ make run
```
