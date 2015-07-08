default: clean test bench

clean:
	cargo clean

test:
	cargo test --verbose -- --nocapture

build:
	cargo build --verbose
	cargo doc --verbose

release: test
	cargo build --verbose --release

bench: build
	cargo bench

update:
	cargo update

ci:
	@echo "CI Status available at: https://travis-ci.org/stuarthicks/cidrr"
