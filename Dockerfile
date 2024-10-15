FROM rust:1.81 as builder
RUN mkdir /app \
    && cd /app \
    && USER=root cargo new project
WORKDIR /app/project
COPY Cargo.toml /app/project/Cargo.toml
COPY Cargo.lock /app/project/Cargo.lock
RUN cargo build --release

COPY src /app/project/src
RUN cargo build --release

FROM alpine:3.20
COPY --from=builder /target/release/eir /usr/local/bin/eir
CMD ["eir"]
