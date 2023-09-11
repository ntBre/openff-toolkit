clippy:
	cargo clippy --workspace --tests

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture $(ARGS)

all-clippy:
	cargo clippy --workspace --tests --all-features

all-test:
	LD_LIBRARY_PATH=/home/brent/omsf/clone/openmm/build \
	RUST_BACKTRACE=1 cargo test --all-features -- --nocapture $(ARGS)
