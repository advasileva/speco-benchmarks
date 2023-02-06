build.java:
	cd java && mvn clean install

build:
	make build.java

run.java:
	cd java && java -jar target/benchmarks.jar

run.go:
	cd go && go test -gcflags=-N -bench MeasureWithPoly
	cd go && go test -gcflags=-N -bench MeasureWithoutPoly

run.python:
	cd python && python -m main

run.rust:
	cd rust && cargo bench

run.js:
	cd js && node main.js

run:
	make run.java
	make run.go
	make run.python
	make run.rust
	make run.js