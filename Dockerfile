# syntax=docker/dockerfile:experimental
FROM rust:latest as builder

WORKDIR /app
# Copy the source code into the container.
COPY . .

# Build the project in release mode.
RUN cargo build --release

# Use a minimal base image to reduce final image size. bookworm has the libgcc dependencies which is required for building the image
FROM debian:bookworm-slim

WORKDIR /app
# Copy the binary from the builder stage.
COPY --from=builder /app/target/release/lambert_w .

# Set the startup command.
CMD ["./lambert_w"]
