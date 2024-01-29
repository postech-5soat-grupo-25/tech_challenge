# Precisa ter o .env com as variaveis corretas
include .env
export

.PHONY: build
build:
	docker compose up db -d
	sleep 5
	docker cp ./migrations/0001_create_table.sql tech_challenge-db-1:/0001_create_table.sql
	docker compose exec db psql -U ${POSTGRES_USER} -d ${POSTGRES_DB} -a -f 0001_create_table.sql
	sleep 2
	docker cp ./migrations/0002_insert_basic.sql tech_challenge-db-1:/0002_insert_basic.sql
	docker compose exec db psql -U ${POSTGRES_USER} -d ${POSTGRES_DB} -a -f 0002_insert_basic.sql
	sleep 2
	docker compose up app --build

.PHONY: run
run:
	docker compose up --remove-orphans

.PHONY: down
down:
	docker compose down
