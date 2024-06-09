_help:
	just -l

# Run tests using nextest
test:
	cargo nextest run

# Format using nightly
fmt:
	cargo +nightly fmt
