exec:
	make fix
	make test
	make run

build:
	cargo build

fix:
	cargo fmt

test:
	cargo clippy
	cargo test

run:
	cargo run
