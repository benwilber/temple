all: build

build:
	cargo build

buildrelease:
	cargo build --release

format: addrustfmt
	cargo fmt

lint: addrustfmt addclippy
	cargo fmt -- --check
	cargo clippy

doc:
	cargo doc

test:
	@mkdir -p tests/outputs
	cargo test

ready: format lint test
	@echo "Ready!"

addrustfmt:
	@rustup component add rustfmt 2> /dev/null

addclippy:
	@rustup component add clippy 2> /dev/null

.PHONY: all build buildrelease format lint doc test ready addrustfmt addclippy
