.PHONY: dev build test

dev:
	uvx maturin develop --release --uv

build:
	uv build

test:
	uv run --with pytest pytest tests/ -v
