FROM golang:1.22-alpine AS builder
WORKDIR /build
COPY . .
RUN go mod download
RUN go build -o ./go-server-box

FROM gcr.io/distroless/base-debian12 AS runtime
WORKDIR /app
COPY --from=builder /build/go-server-box /go-server-box
EXPOSE 3002
CMD ["/go-server-box"]