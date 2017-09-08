all: run

build:
	cargo build --release
	mv ./target/release/libmvrpl.dylib ./mvrpl.so

run: build
	python test.py

clean:
	cargo clean
	rm ./example.so