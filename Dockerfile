FROM rust:latest as builder

WORKDIR /usr/src/app
ENV SQLX_OFFLINE=true
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > /usr/src/app/src/main.rs
RUN apt-get update && apt-get install -y musl-tools pkg-config perl make && \
    apt-get clean && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release --locked

COPY . .
RUN ls /usr/src/app
RUN cargo build --target x86_64-unknown-linux-musl --release --locked && sleep 600

FROM alpine
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/tutors /app/tutors
COPY --from=builder /usr/src/app/sql /app/sql
COPY static /app/static
COPY views /app/views

RUN ls /app

CMD ["/app/tutors"]
