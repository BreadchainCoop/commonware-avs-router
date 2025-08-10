# Build stage
FROM rust:1.83 AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Copy source code
COPY src ./src
COPY eigenlayer-bls-local ./eigenlayer-bls-local
COPY commonware-avs-node ./commonware-avs-node

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 -s /bin/bash appuser

# Copy the binary from builder
COPY --from=builder /app/target/release/commonware-avs-router /usr/local/bin/commonware-avs-router

# Copy configuration files
COPY config /app/config

# Set ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Set working directory
WORKDIR /app

# Expose port 3000 for the application
EXPOSE 3000

# Run the binary
ENTRYPOINT ["commonware-avs-router"]