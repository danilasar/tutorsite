FROM rust:1.78.0 as builder

RUN apt-get update && apt-get install -y musl-tools pkg-config perl make && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY sql sql
COPY .sqlx .sqlx
COPY views views 
COPY static static

ENV SQLX_OFFLINE=true
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /app 

COPY --from=builder usr/src/app/target/x86_64-unknown-linux-musl/release/tutors /app/tutors
COPY --from=builder /usr/src/app/sql /app/sql
COPY static /app/static 
COPY views /app/views
CMD ["/app/tutors"]

