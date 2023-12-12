dev:
  cargo watch -x 'run --bin api'

dev-docker:
  docker-compose -f docker-compose.dev.yml up --build

prod-docker:
  docker-compose up --build