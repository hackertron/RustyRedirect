.DEFAULT_GOAL := build

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	RUST_LOG=rusty_redirect=debug cargo run