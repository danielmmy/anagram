.PHONY: test bench bench-large

test:
	cargo test --lib

bench:
	cargo bench

bench-large:
	cargo bench "test large"