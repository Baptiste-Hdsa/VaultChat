# Using the latest stable version of Rust
FROM rust:1.78 AS builder

# Set the working directory
WORKDIR /app

# Install necessary dependencies
RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev

# Copy the dependency definitions
COPY Cargo.toml Cargo.lock ./

# Create a dummy project to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the actual source code
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Second stage: create a smaller image
FROM debian:bullseye-slim

# Install OpenSSL, which is a runtime dependency
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/backend /usr/local/bin/

# Expose the port the app runs on
EXPOSE 3000

# Set the startup command to run the binary
CMD ["backend"]