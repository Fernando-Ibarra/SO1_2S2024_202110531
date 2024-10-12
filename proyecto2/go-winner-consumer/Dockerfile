FROM golang:1.20-buster AS builder
WORKDIR /build
COPY . .
RUN go mod download
RUN go build -o ./go-winner-consumer

FROM gcr.io/distroless/base-debian12 AS runtime
WORKDIR /app
COPY --from=builder /build/go-winner-consumer /go-winner-consumer
EXPOSE 3004
CMD ["/go-winner-consumer"]