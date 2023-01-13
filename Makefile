build.java:
	cd java && mvn clean install

build:
	make build.java

run.java:
	cd java && java -jar target/benchmarks.jar

run:
	make run.java