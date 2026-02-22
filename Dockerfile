# STAGE 1: Builder
FROM rust:1-slim-bookworm as builder

# Install build-time dependencies for OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/urchin
COPY . .

# Build the release binary
RUN cargo build --release

# STAGE 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
# ca-certificates is CRITICAL for fixing the Stoat SSL "invalid peer certificate" error
RUN apt-get update && apt-get install -y \
    openssl \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -ms /bin/bash urchin_user

# Set up the application directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/urchin/target/release/urchin /app/urchin

# 1. Create the database directory as ROOT
# 2. Change ownership to the non-root user
# 3. This ensures Sled can write to the DB without permission errors
RUN mkdir -p /app/urchin_db && chown -R urchin_user:urchin_user /app

# Switch to the non-root user for security
USER urchin_user

# Ensure the database path environment variable (if used) matches the volume
ENV DATABASE_PATH=/app/urchin_db

# Run the kernel
CMD ["./urchin"]