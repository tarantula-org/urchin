# STAGE 1: Builder
FROM rust:1-slim-bookworm as builder

# 1. Install Build Dependencies (pkg-config & OpenSSL headers)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/urchin
COPY . .

# Build the release binary
RUN cargo build --release

# STAGE 2: Runtime
FROM debian:bookworm-slim

# Install Runtime Dependencies (OpenSSL runtime libs)
RUN apt-get update && apt-get install -y \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -ms /bin/bash urchin_user
USER urchin_user
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/urchin/target/release/urchin /app/urchin

# Create the database directory
RUN mkdir -p /app/urchin_db

# Command to run the bot
CMD ["./urchin"]