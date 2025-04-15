.PHONY: all
all: build_all


build_all:
	cargo build

build_release:
	cargo build --release

build_lib:
	cargo build --release --lib

build_p12_bin:
	cargo build --release --bin p12_bin
