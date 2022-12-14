java_run: lib
	javac Console.java && java -Djava.library.path=target/release/ Console

lib:
	cargo build -r
