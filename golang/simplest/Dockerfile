FROM golang:alpine AS builder
WORKDIR /build
COPY . .
RUN go build -o server .

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /app/
COPY --from=builder /build/server .
EXPOSE 8888
ENTRYPOINT /app/server

