FROM postgres:latest

ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_DB=postgres

COPY migration-script.sh /docker-entrypoint-initdb.d/
COPY 0001_create_table.sql .
COPY 0002_insert_basic.sql .
RUN chmod +x /docker-entrypoint-initdb.d/migration-script.sh