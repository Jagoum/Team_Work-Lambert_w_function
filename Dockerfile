# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the binary in release mode
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/lambert_w /app/lambert_w

# Set the binary as the entry point
ENTRYPOINT ["./lambert_w"]
