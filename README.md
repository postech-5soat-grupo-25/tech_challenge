# Modelo de Projeto Rust utilizando Boas práticas e DDD

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust Analyzer](https://rust-analyzer.github.io/manual.html#installation)

## Uso Básico

- Clone o repositório

```bash
git clone git@github.com:postech-5soat-grupo-25/tech_challenge.git
```

- Entre na pasta do projeto

```bash
cd tech_challenge
```

- Execute o projeto

Se for a primeira vez que você executa a aplicação, execute o seguinte comando para rodar os contêiners e executar as migrações (criar as tabelas e inserir os dados básicos):

```bash
make build
```

Se você já tiver tudo configurado, poderá executar os contêiners com o seguinte:

```bash
make run
```

Se você quiser parar todos os contêineres relacionados, use o seguinte:

```bash
make down
```

## Usando docker

- Produção

```bash
docker-compose up
```

ou

```bash
make run
```

- Desenvolvimento

```bash
docker-compose -f docker-compose.dev.yml up
```

ou

```bash
just dev-docker
```
