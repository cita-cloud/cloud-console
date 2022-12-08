java_run: lib
	javac Console.java && java -Djava.library.path=target/release/ Console

.PHONY: lib

lib:
	cargo build -r
