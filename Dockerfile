# Build stage (builder)
FROM ubuntu:24.04 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-good1.0-dev \
    libgstreamer-plugins-bad1.0-dev \
    libgstrtspserver-1.0-dev \
    pkg-config

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy source code
WORKDIR /mobius
COPY . .

# Build project in release mode
RUN cargo build --release

# Runtime stage
FROM ubuntu:24.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libgstreamer1.0-0 \
    libgstreamer-plugins-base1.0-0 \
    libgstreamer-plugins-good1.0-0 \
    libgstreamer-plugins-bad1.0-0 \
    libgstrtspserver-1.0-0 \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-x \
    gstreamer1.0-tools

# Create user for security
RUN useradd -m -s /bin/bash mobiususer

# Copy compiled binary from builder stage
COPY --from=builder /mobius/target/release/mobius-rtsp /usr/local/bin/mobius-rtsp

# Create video directory and make it writable
RUN mkdir -p /mobius/videos && chown mobiususer:mobiususer /mobius/videos

# Switch to unprivileged user
USER mobiususer

# Working directory
WORKDIR /mobius

# Start server
ENTRYPOINT ["mobius-rtsp"]