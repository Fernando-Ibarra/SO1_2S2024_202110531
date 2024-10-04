FROM golang:1.22-alpine AS builder
WORKDIR /build
COPY . .
RUN go mod download
RUN go build -o ./go-server-run

FROM gcr.io/distroless/base-debian12 AS runtime
WORKDIR /app
COPY --from=builder /build/go-server-run /go-server-run
EXPOSE 3001
CMD ["/go-server-run"]