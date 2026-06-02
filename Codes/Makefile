DOCKER_EXEC = docker compose exec -u $(shell id -u):$(shell id -g) dev

.PHONY: up down build exec cpp rust

up:
	@mkdir -p workspace
	@docker compose up -d

down:
	@docker compose down

build:
	@mkdir -p workspace
	@docker compose build

exec:
	@$(DOCKER_EXEC) bash

cpp:
	@$(DOCKER_EXEC) bash -c "g++ -std=c++23 -O2 $(FILE).cpp -o $(FILE) && ./$(FILE)"

rust:
	@$(DOCKER_EXEC) bash -c '\
		export CARGO_HOME=/tmp/cargo && \
		export RUSTUP_HOME=/tmp/rustup && \
		export RUNNER_DIR=/workspace/.runner && \
		mkdir -p $$CARGO_HOME $$RUSTUP_HOME && \
		rm -rf $$RUNNER_DIR && \
		cargo new $$RUNNER_DIR --name runner 2>/dev/null && \
		cp /workspace/$(FILE).rs $$RUNNER_DIR/src/main.rs && \
		cd $$RUNNER_DIR && \
		cargo run \
	'
