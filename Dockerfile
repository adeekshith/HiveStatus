# Build Stage
FROM rust:alpine AS builder

# Install build dependencies
# - musl-dev, gcc: For compiling C dependencies (like ring)
# - ca-certificates: To copy to the final image for HTTPS support
RUN apk add --no-cache musl-dev gcc ca-certificates

WORKDIR /app

# Copy source code
COPY . .

# Build for release
# rust:alpine targets x86_64-unknown-linux-musl by default, which is statically linked.
RUN cargo build --release

# Runtime Stage - Scratch (Empty Image)
FROM scratch

WORKDIR /app

# Copy CA certificates from the builder stage so HTTPS requests to Gatus work
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the static binary
COPY --from=builder /app/target/release/hive-status .

# Copy static assets
COPY static ./static

# Expose the application port
EXPOSE 3000

# Default Environment Variable
ENV GATUS_BASE_URL="https://status.deekshith.in"

# Run the application
CMD ["./hive-status"]
