# Modelo de Projeto Rust utilizando Boas práticas e DDD

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust Analyzer](https://rust-analyzer.github.io/manual.html#installation)

## Objetivo

- Criar um modelo de projeto rust utilizando boas práticas e DDD

## Uso básico

Versão utilizada do Rust: v1.75.0-nightly

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

## Usando docker (ainda não funciona)

- Crie a imagem

```bash
docker build -t rust_ddd_template .
```

- Execute o container

```bash
docker run -it --rm -p 3000:8000 --name rust_ddd_template-container rust_ddd_template
```
