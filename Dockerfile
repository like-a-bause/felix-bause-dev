FROM node:18-alpine AS fe_installer

WORKDIR /app

COPY frontend/package.json ./
COPY frontend/yarn.lock ./

RUN yarn

FROM ghcr.io/getzola/zola:v0.17.1 AS fe_builder

WORKDIR /app

COPY --from=fe_installer /app/node_modules ./
COPY frontend/ ./
RUN ["/bin/zola", "build"]

FROM rust:slim AS be_builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=be_builder /app/target/release/felix-bause-dev /opt/
COPY --from=fe_builder /app/public /opt/
ENTRYPOINT [ "/opt/felix-bause-dev" ]