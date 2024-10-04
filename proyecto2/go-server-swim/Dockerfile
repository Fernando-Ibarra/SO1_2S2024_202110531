FROM golang:1.22-alpine AS builder
WORKDIR /build
COPY . .
RUN go mod download
RUN go build -o ./go-server-swim

FROM gcr.io/distroless/base-debian12 AS runtime
WORKDIR /app
COPY --from=builder /build/go-server-swim /go-server-swim
EXPOSE 3000
CMD ["/go-server-swim"]