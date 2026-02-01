# ---------- Stage 1: Build ----------
FROM rust:1.91-slim AS builder

# Set working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock first (cache deps)
COPY . ./


# Install system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libfontconfig1-dev \
        fonts-ipafont \
        ca-certificates \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {println!(\"Hello\");}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy main.rs
RUN rm -rf src

# Build the actual application
RUN cargo build --release

# ---------- Stage 2: Minimal runtime ----------
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libfontconfig1 \
        fonts-ipafont \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/stock ./stock

# Copy CSV files or expect them to be mounted at runtime
# (optional: if CSVs already on host, mount them as volume)

# Set environment variable for fontconfig
ENV FONTCONFIG_PATH=/etc/fonts

# Default command
CMD ["./stock"]
