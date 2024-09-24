folder_name ?= sample
folder_type ?= --hacking

all: test lint format docker compose

start:
	./target/release/mks $(folder_name) $(folder_type)

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
	cargo clippy --all-targets --all-features

lint-fix:
	cargo clippy --all-targets --all-features --fix --allow-dirty
 
format:
	cargo fmt --all --check

format-fix:
	cargo fmt --all

.PHONY: build docker test lint format compose