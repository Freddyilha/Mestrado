PROJECT ?= Teste
PROJECT_DIR ?= /usr/src/$(PROJECT)

up:
	docker compose up -d

down:
	docker compose down

restart:
	docker compose down
	docker compose up -d --force-recreate

logs:
	docker compose logs -f

scanner-shell:
	docker compose exec scanner sh

sonar:
	docker compose exec scanner sh -c "\
		cd $(PROJECT_DIR) && \
		sonar-scanner \
			-Dsonar.projectKey=$(PROJECT) \
			-Dsonar.projectName=$(PROJECT) \
			-Dsonar.sources=."

lizard:
	docker compose exec lizard sh -c "\
		cd $(PROJECT_DIR) && \
		lizard ."

cppcheck:
	docker compose exec cppcheck sh -c "\
		cd $(PROJECT_DIR) && \
		cppcheck --enable=all ."

clang-tidy:
	docker compose exec clang-tidy sh -c "\
		cd $(PROJECT_DIR) && \
		find . -name '*.cpp' | xargs clang-tidy"

permissions:
	sudo chmod -R 777 projects

clean:
	docker compose down -v
