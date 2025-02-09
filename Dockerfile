FROM rust:1.72-bullseye AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY api_tarea1.db api_tarea1.db
COPY service-account.json service-account.json
COPY service-account.enc service-account.enc

RUN rustc --version && cargo --version

RUN rm -f Cargo.lock && cargo generate-lockfile

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates libsqlite3-0 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/api_tarea1 .

COPY --from=builder /usr/src/app/api_tarea1.db api_tarea1.db

COPY dll/ /usr/local/lib/

COPY service-account.json service-account.json
COPY service-account.enc service-account.enc

EXPOSE 8080

CMD ["./api_tarea1"]