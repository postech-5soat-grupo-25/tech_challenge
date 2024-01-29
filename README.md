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
- [x] Validação de Objetos de Valor - CPF (remover a prop numero do cpf sem remover a validação)
- [x] Postgres
- [ ] Testes unitarios
- [ ] Testes de integração
- [ ] Documentação
- [ ] Corrigir erro de compilação do projeto com o docker

#### Desejável

- [ ] Logging/Tracing
- [ ] Validação de Objetos de Valor - Endereço
- [ ] User Roles
- [ ] Renderização de templates
- [ ] Uso de HTMX
- [ ] Docker pra rodar testes unitário e testes de integração (adicionar variavel de ambiente com tipo de ambiente)
