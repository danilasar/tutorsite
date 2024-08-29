FROM rust:1.78.0 as builder

WORKDIR /usr/src/app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY sql sql
COPY views views 
COPY static static
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/tutors /usr/local/bin/tutors

CMD ["tutors"]
