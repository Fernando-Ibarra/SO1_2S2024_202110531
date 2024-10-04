# Fase de compilación
FROM rust:1.81.0
WORKDIR /app
COPY . .
RUN cargo build --release

# Ejecutar el binario
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
EXPOSE 8080
CMD ["./target/release/rust-server"]