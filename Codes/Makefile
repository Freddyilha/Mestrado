.PHONY: up down build exec

up:
	@mkdir -p workspace
	@docker compose up -d

down:
	@docker compose down

build:
	@mkdir -p workspace
	@docker compose build

exec:
	@docker compose exec dev bash

run:
	@docker compose exec dev bash -c "$(CMD)"

cpp:
	@docker compose exec dev bash -c "g++ -std=c++23 -O2 $(FILE).cpp -o $(FILE) && ./$(FILE)"

rust:
	@docker compose exec dev bash -c "\
		if [ ! -f /tmp/runner/Cargo.toml ]; then cargo new /tmp/runner --name runner 2>/dev/null; fi && \
		cp /workspace/$(FILE).rs /tmp/runner/src/main.rs && \
		cd /tmp/runner && \
		grep -E '^(extern crate|use )' /tmp/runner/src/main.rs | \
			grep -oE '^(extern crate |use )[a-z_0-9]+' | \
			awk '{print \$$2}' | \
			grep -vE '^(std|core|alloc|runner)$$' | sort -u | \
			xargs -I{} sh -c 'grep -q \"{}\" Cargo.toml || cargo add {}' && \
		cargo run"
