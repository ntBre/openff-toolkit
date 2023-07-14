clippy:
	cargo clippy --workspace --tests

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture
