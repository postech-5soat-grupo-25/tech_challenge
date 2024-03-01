#!/bin/bash
for i in {1..100000}; do
  curl -X POST localhost:31200/auth/login -H "Content-Type: application/json" -d '{"cpf": "000.000.000-00", "senha": "melhor_projeto"}'
done