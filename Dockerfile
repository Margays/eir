FROM rust:1.90 as base

FROM base AS workspace
ARG REMOTE_USER
ARG REMOTE_UID
ARG REMOTE_GID
RUN addgroup --gid ${REMOTE_GID} ${REMOTE_USER}
RUN adduser --disabled-password --uid ${REMOTE_UID} --gid ${REMOTE_GID} ${REMOTE_USER}
RUN wget https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64 -O /usr/local/bin/yq \
    && chmod +x /usr/local/bin/yq
RUN curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 \
    && chmod 700 get_helm.sh \
    && ./get_helm.sh \
    && rm get_helm.sh
ENV HOME /home/${REMOTE_USER}
ENV LC_ALL=C.UTF-8
USER ${REMOTE_USER}

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
