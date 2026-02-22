FROM rust:alpine AS builder

RUN apk add --no-cache gcc musl-dev pkgconfig openssl-dev openssl-libs-static

WORKDIR /usr/src/urchin
COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache openssl ca-certificates && \
    addgroup -S urchin && \
    adduser -S urchin_user -G urchin

WORKDIR /app

COPY --from=builder /usr/src/urchin/target/release/urchin /app/urchin
COPY --from=builder /usr/src/urchin/config.toml /app/config.toml

RUN mkdir -p /app/urchin_db && chown -R urchin_user:urchin /app

USER urchin_user

CMD ["./urchin"]