UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
	TARGET = libmvrpl.so
endif
ifeq ($(UNAME_S),Darwin)
	TARGET = libmvrpl.dylib
endif

all: run

build:
	cargo build --release
	mv ./target/release/$(TARGET) ./mvrpl.so

run: 
	build
	python test.py

clean:
	cargo clean
	rm ./mvrpl.so