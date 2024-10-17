FROM golang:1.20-buster AS builder
WORKDIR /build
COPY . .
RUN go mod download
RUN go build -o ./go-loser-consumer

FROM gcr.io/distroless/base-debian12 AS runtime
WORKDIR /app
COPY --from=builder /build/go-loser-consumer /go-loser-consumer
EXPOSE 3006
CMD ["/go-loser-consumer"]