clippy:
	cargo clippy

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture
