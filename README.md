# Benchmarks

*Alena: I'll add a description later.*

### Results

Case | Java | C++ | Go 
------ | ------ | ------ | ------ 
Polimorphic | 8.648s | 32.642s | 46.001s
No polimorphism | 6.360s | 29.822s | 24.122s
Performance gain | 35.97% | 9.45% | 90.7%

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
