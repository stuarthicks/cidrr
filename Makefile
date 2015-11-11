default: build test

clean:
	cargo clean

build:
	cargo build --verbose
	cargo doc --verbose

test:
	RUST_BACKTRACE=1 cargo test --verbose -- --nocapture

release: test
	cargo build --verbose --release

bench: build
	cargo bench

update:
	cargo update

ci:
	@echo "CI Status available at: https://travis-ci.org/stuarthicks/cidrr"
