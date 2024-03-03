run:
	@cargo run

lint:
	@cargo +nightly fmt
	@cargo fix --allow-dirty --allow-staged
	@cargo clippy --fix --allow-dirty --allow-staged

test:
	@cargo test

fix:
	@make lint
	@make test

