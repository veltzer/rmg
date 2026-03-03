.PHONY: all
all:
	@cargo build
	@cargo build --release

.PHONY: test
test:
	cargo nextest run --release
	cargo nextest run

.PHONY: clean
clean:
	@cargo clean
