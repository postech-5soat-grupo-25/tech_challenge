FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .

RUN rustup default nightly
RUN cargo install --path .

EXPOSE 8000

CMD ["rust_ddd_template"]