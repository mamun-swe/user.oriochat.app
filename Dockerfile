# Stage 1: Builder
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Install necessary dependencies including protoc
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

# Copy the Cargo.toml and Cargo.lock to the container to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies first, so we can cache them separately
RUN cargo fetch

# Copy the entire source code into the container
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim AS runtime

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/user-oriochat-app /usr/local/bin/user-oriochat-app

# Set the default command for the container
CMD ["user-oriochat-app"]
