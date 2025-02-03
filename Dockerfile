# Etapa 1: Compilación
FROM rust:1.72-bullseye as builder

WORKDIR /usr/src/app

# Copiar los archivos del proyecto, incluyendo Cargo.toml y Cargo.lock
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY api_tarea1.db api_tarea1.db
COPY service-account.json service-account.json

RUN rustc --version && cargo --version

# Eliminar el Cargo.lock y generar uno nuevo compatible con la versión de Cargo de la imagen
RUN rm -f Cargo.lock && cargo generate-lockfile

# Compilar la aplicación en modo release
RUN cargo build --release

# Etapa 2: Imagen de producción
FROM debian:bullseye-slim

# Instalar certificados y librerías requeridas (por ejemplo, SQLite)
RUN apt-get update && apt-get install -y ca-certificates libsqlite3-0 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copiar el binario compilado desde la etapa de compilación
COPY --from=builder /usr/src/app/target/release/api_tarea1 .

# Copiar la base de datos (si se requiere)
COPY --from=builder /usr/src/app/api_tarea1.db api_tarea1.db

# (Opcional) Copiar los archivos DLL u otros si los necesitas
COPY dll/ /usr/local/lib/

COPY service-account.json service-account.json

EXPOSE 8080

CMD ["./api_tarea1"]
