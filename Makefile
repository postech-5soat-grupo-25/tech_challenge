.PHONY: all
all: up

.PHONY: build
build:
	docker compose up db -d
	sleep 10
	docker ps
	docker cp ./migrations/0001_create_table.sql tech_challenge-db-1:/0001_create_table.sql
	docker compose exec db psql -U postgres -d postgres -a -f 0001_create_table.sql

.PHONY: run
run:
	docker compose up --remove-orphans

.PHONY: down
down:
	docker compose down

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
