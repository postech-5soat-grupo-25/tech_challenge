FROM rust:1.67

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["aula-2"]