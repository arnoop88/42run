CARGO = cargo
TARGET = target/debug/42run

all: build

build:
	$(CARGO) build

run: build
	$(TARGET)

clean:
	$(CARGO) clean

release:
	$(CARGO) build --release
	cp target/release/42run .

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy

.PHONY: all build run clean release test fmt clippy