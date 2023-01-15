build.java:
	cd java && mvn clean install

build:
	make build.java

run.java:
	cd java && java -jar target/benchmarks.jar

run.go:
	cd go && go test -gcflags=-N -bench MeasureWithPoly
	cd go && go test -gcflags=-N -bench MeasureWithoutPoly

run:
	make run.java
	make run.go