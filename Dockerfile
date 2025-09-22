# Build stage
FROM rust:1.83 AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifest files
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Pre-build deps
# Create dummy main files for both workspace members to satisfy Cargo during dependency pre-build
RUN mkdir src && echo 'fn main(){}' > src/main.rs
RUN mkdir -p scripts/src && echo 'fn main(){}' > scripts/src/main.rs
RUN echo '[package]\nname = "scripts"\nversion = "0.1.0"\nedition = "2021"' > scripts/Cargo.toml
RUN cargo build --release || true
RUN rm -rf src scripts

# Now copy real source
COPY src ./src

# Copy scripts
COPY scripts ./scripts

# Do the actual build
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

