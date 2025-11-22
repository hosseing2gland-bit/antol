# Build stage
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY migrations ./migrations

# Build release
RUN cargo build --release

# Runtime stage
FROM alpine:latest

RUN apk add --no-cache libgcc openssl ca-certificates

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/backend /app/backend
COPY --from=builder /app/migrations /app/migrations

# Set environment
ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/backend"]
