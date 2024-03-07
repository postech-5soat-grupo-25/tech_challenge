#!/bin/bash

# Your migration commands
psql -U ${POSTGRES_USER} -d ${POSTGRES_DB} -a -f 0001_create_table.sql
psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -a -f 0002_insert_basic.sql
