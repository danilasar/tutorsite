FROM rust:1.78.0

WORKDIR /usr/src/backend
COPY . .

RUN cargo install --path .

CMD ["backend"]
