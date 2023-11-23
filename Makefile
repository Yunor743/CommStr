# Makefile for Rust project using Cargo

# Compiler
RUSTC = rustc

# Cargo
CARGO = cargo

# Binary name
OUTPUT = commstr

# Build directory
TARGET = x86_64-unknown-linux-musl

# Release build configuration
RELEASE = --release

BUILD_DIR = target/x86_64-unknown-linux-musl
# Build target
OUTPUT_DIR = $(BUILD_DIR)/release/

.PHONY: all build clean run

all: build

build:
	$(CARGO) build $(RELEASE) --target=$(TARGET)
	cp $(OUTPUT_DIR)/$(OUTPUT) .

clean:
	$(CARGO) clean
	rm -f $(OUTPUT)

run: build
	./$(OUTPUT)
