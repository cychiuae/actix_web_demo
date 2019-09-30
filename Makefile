.PHONY: check-format format lint lint typecheck

check-format:
	@cargo fmt -- --check

format:
	@cargo fmt

lint:
	@cargo clippy

typecheck:
	@cargo check
