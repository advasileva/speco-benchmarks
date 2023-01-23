build.java:
	cd java && mvn clean install

build.eo:
	cd eo && make build

build:
	make build.java
	make build.eo

run.java:
	cd java && java -jar target/benchmarks.jar

run.go:
	cd go && go test -gcflags=-N -bench MeasureWithPoly
	cd go && go test -gcflags=-N -bench MeasureWithoutPoly

run.python:
	cd python && python -m main

run.eo:
	cd eo && make run

run.rust:
	cd rust && cargo bench

run.js:
	cd js && node main.js

run:
	make run.java
	make run.go
	make run.python
	make run.eo
	make run.rust
	make run.js