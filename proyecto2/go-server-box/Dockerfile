FROM golang:1.21.13-bullseye AS builder
WORKDIR /build
COPY . .
RUN go mod download
RUN go build -o ./go-server-box

FROM gcr.io/distroless/base-debian12 AS runtime
WORKDIR /app
COPY --from=builder /build/go-server-box /go-server-box
EXPOSE 3003
CMD ["/go-server-box"]