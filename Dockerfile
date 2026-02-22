# STAGE 1: Builder
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /usr/src/urchin
COPY . .

RUN cargo build --release

# STAGE 2: Runtime
FROM alpine:latest

# ca-certificates and openssl are required for Stoat's native-tls handshake
RUN apk add --no-cache openssl ca-certificates && \
    addgroup -S urchin && \
    adduser -S urchin_user -G urchin

WORKDIR /app

# Copy binary and configuration from builder
COPY --from=builder /usr/src/urchin/target/release/urchin /app/urchin
COPY --from=builder /usr/src/urchin/config.toml /app/config.toml

# Set up persistence layer permissions
RUN mkdir -p /app/urchin_db && chown -R urchin_user:urchin /app

USER urchin_user

CMD ["./urchin"]