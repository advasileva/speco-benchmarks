# Benchmarks

**Task**: Call a polymorphic method many times.

**Question**: How does Late Binding affect performance?

### Results

Case | Java | C++ | Go 
------ | ------ | ------ | ------ 
Polimorphic | 8.648s | 32.642s | 17.574s 
No polimorphism | 6.360s | 29.822s | 2.907s
Performance gain | 35.97% | 9.45% | 504,54%

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
