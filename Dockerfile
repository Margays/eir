FROM rust:1.88 as base

FROM base AS workspace

FROM base as builder
RUN mkdir /app && cd /app && USER=root cargo new project
WORKDIR /app/project
COPY Cargo.toml /app/project/Cargo.toml
COPY Cargo.lock /app/project/Cargo.lock
RUN cargo check && cargo build --release
RUN rm -rf /app/project/target/release/deps/eir*
COPY src /app/project/src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY LICENSE /LICENSE
COPY --from=builder /app/project/target/release/eir /eir
EXPOSE 3000
CMD ["/eir"]
