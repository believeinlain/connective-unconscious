# Use the official Rust image as the base image
FROM rust:bookworm AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Build dependencies to cache them
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/server.rs && \
    cargo build --release && \
    rm -rf src

# Copy the rest of your source code
COPY . .

# Build your Rust application
RUN cargo build --release

# Create a new lightweight image without the build dependencies
FROM debian:bookworm-slim

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/server ./

# Copy static files
COPY --from=builder /usr/src/app/static ./static

# Expose any necessary ports
EXPOSE 3000

# Command to run your application
ENTRYPOINT ["./server"]