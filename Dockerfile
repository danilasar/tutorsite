FROM rust:1.88.0-alpine3.22 as builder

RUN apk add --no-cache musl-dev perl make pkgconfig

WORKDIR /usr/src/app
ENV SQLX_OFFLINE=true
COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked

COPY . .
RUN touch src/main.rs
RUN cargo build --release --locked 

FROM scratch
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/tutors /app/tutors
COPY --from=builder /usr/src/app/sql /app/sql
COPY static /app/static
COPY views /app/views

CMD ["/app/tutors"]
