clippy:
	cargo clippy --workspace --tests

test:
	LD_LIBRARY_PATH=/home/brent/omsf/clone/openmm/build \
	RUST_BACKTRACE=1 cargo test -- --nocapture $(ARGS)
