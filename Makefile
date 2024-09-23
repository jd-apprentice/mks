folder_name ?= sample

all: test lint format docker compose

start:
	./target/release/mks $(folder_name)

dev:
	cargo watch -x run

build: src/main.rs
	cargo build --release --bin mks

docker: 
	docker build -f docker/Dockerfile -t mks .

compose:
	docker compose -f docker/compose.yml up

test:
	cargo test

lint:
	cargo clippy -- -D warnings
 
format:
	cargo fmt --all --check

format-fix:
	cargo fmt --all

.PHONY: build docker test lint format compose