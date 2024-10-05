# Fase de compilación
FROM rust:1.81.0

# Instalar dependencias necesarias para compilar Protobufs
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .

# Compilar el proyecto
RUN cargo build --release

# Ejecutar el binario
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
EXPOSE 8080
CMD ["./target/release/rust-server"]