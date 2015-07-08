default: clean test bench ci

clean:
	cargo clean

test:
	cargo test --verbose

build:
	cargo build --verbose
	cargo doc

bench:
	cargo bench

update:
	cargo update

ci:
	@echo "CI Status available at: https://travis-ci.org/stuarthicks/cidrr"
