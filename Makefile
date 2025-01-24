TARGET := release
BIN_DIR := ./src/bin

all: build 

build:
	cargo build --bin user_main --$(TARGET) --target-dir ./src/bin

run:
	cargo run

clean: 
	cargo clean

check:
	cargo check