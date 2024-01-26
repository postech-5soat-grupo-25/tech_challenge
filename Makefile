.PHONY: all
all: up

.PHONY: build
build:
	docker compose up --build --remove-orphans

.PHONY: run
run:
	docker compose up --remove-orphans

.PHONY: down
down:
	docker compose down
