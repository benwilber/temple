all: build

build:
	cargo build

release:
	cargo build --release

format: addrustfmt
	cargo fmt

lint: addrustfmt addclippy
	cargo fmt -- --check
	cargo clippy

doc:
	cargo doc

test:
	cargo test

testall:
	cargo test --all-features

testcli: build
	cd tests && TEMPLE="$(shell pwd)/target/debug/temple" bats tests.bats

testclirelease: release
	TEMPLE="$(shell pwd)/target/release/temple" bats tests.bats

ready: format lint testcli
	@echo "Ready!"

addrustfmt:
	@rustup component add rustfmt 2> /dev/null

addclippy:
	@rustup component add clippy 2> /dev/null

.PHONY: all build release format lint doc test testall addrustfmt addclippy
