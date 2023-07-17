clippy:
	cargo clippy --workspace --tests

test:
	LD_LIBRARY_PATH=$(shell conda info --base)/lib RUST_BACKTRACE=1 \
	cargo test -- --nocapture
