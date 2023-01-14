build.java:
	cd java && mvn clean install

build:
	make build.java

run.java:
	cd java && java -jar target/benchmarks.jar

run.go:
	cd go && go test -bench MeasureWithPoly
	cd go && go test -bench MeasureWithoutPoly

run.rust:
	cd rust && cargo bench

run:
	make run.java
	make run.go