# Usando imagem alpine para diminuir o tamanho do container
FROM rust:1.75-alpine3.19 as builder

# Feature apenas em nightly
RUN rustup update nightly 

RUN rustup default nightly

# Definindo a pasta dentro do container onde
WORKDIR /usr/src/tech-challenge

RUN apk add --no-cache musl-dev

COPY . .

RUN cargo build --release

# Criando um novo stage com imagem scratch e passando apenas os 
# arquivos necessarios
FROM scratch as production
COPY --from=builder /usr/src/tech-challenge/target/release /
ENTRYPOINT [ "/api" ]
EXPOSE 3000