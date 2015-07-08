default: clean test build ci

clean:
	cargo clean

test:
	cargo test

build:
	cargo build

ci:
	@echo "CI Status available at: https://travis-ci.org/stuarthicks/cidrr"
