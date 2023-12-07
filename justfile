dev:
  cargo watch -x 'run --bin api'

dev-docker:
  docker-compose up --build

prod-docker:
  docker-compose -f docker-compose.prod.yml up --build