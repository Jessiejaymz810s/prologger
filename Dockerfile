# ─────────────────────────────────────────────────────────────
# prologger — Production-grade Rust logging library
# Multi-stage Docker image for CI/CD pipelines
# ─────────────────────────────────────────────────────────────

# Stage 1: Build & validate
FROM rust:1.82-slim AS builder

WORKDIR /usr/src/prologger

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for layer caching
COPY Cargo.toml Cargo.toml

# Copy source tree
COPY src/ src/
COPY benches/ benches/
COPY examples/ examples/
COPY tests/ tests/
COPY README.md README.md
COPY LICENSE LICENSE

# Build with all features to validate everything compiles
RUN cargo build --release --all-features

# Run tests to ensure the image ships a validated crate
RUN cargo test --release --all-features

# Stage 2: Minimal runtime image with pre-built crate
FROM rust:1.82-slim AS runtime

LABEL org.opencontainers.image.title="prologger"
LABEL org.opencontainers.image.description="Production-grade, ergonomic Rust logging library with colored output, file rotation, and structured formatting."
LABEL org.opencontainers.image.source="https://github.com/Jessiejaymz810s/prologger"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.vendor="Jessiejaymz810s"

WORKDIR /usr/src/prologger

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the full source and pre-built artifacts
COPY --from=builder /usr/src/prologger/ /usr/src/prologger/
COPY --from=builder /usr/local/cargo/registry/ /usr/local/cargo/registry/

# Pre-warm the cargo cache so downstream consumers get fast builds
RUN cargo build --release --all-features 2>/dev/null || true

# Default command: run the basic example to verify the image works
CMD ["cargo", "run", "--example", "basic"]
