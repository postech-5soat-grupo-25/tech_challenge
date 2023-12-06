# Modelo de Projeto Rust utilizando Boas práticas e DDD

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust Analyzer](https://rust-analyzer.github.io/manual.html#installation)

## Objetivo

- Criar um modelo de projeto rust utilizando boas práticas e DDD

## Uso básico

**Versão utilizada do Rust: v1.75.0-nightly**

- Clone o repositório

```bash
git clone git@github.com:albert-dm/rust_ddd_template.git
```

- Entre na pasta do projeto

```bash
cd rust_ddd_template
```

- Execute o projeto

```bash
cargo run
```

## Usando docker

- Produção

```bash
docker-compose up
```

- Desenvolvimento

```bash
docker-compose -f docker-compose.dev.yml up
```

## Roadmap

### Fase 1

- [x] Rotas
- [x] Injeção de dependências
- [x] Async
- [x] Serializaçao e Deserializaçao
- [x] Error Handling
- [x] Swagger
- [x] User CRUD
- [x] Autenticação e Autorização (+ Midlewares and Guards)
- [x] Variaveis de ambiente
- [x] Docker e Docker Compose
- [x] Usar pasta `bin` para executaveis (/bin/server  ---  cargo run api)
- [ ] Postgres
- [ ] Validação de Objetos de Valor
- [ ] Testes unitarios
- [ ] Testes de integração
- [ ] Documentação

### Desejável

- [ ] Logging/Tracing
- [ ] User Roles
