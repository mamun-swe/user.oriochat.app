# Step 1: Use the official Rust image as the base image
FROM rust:latest AS builder

# Step 2: Set the working directory inside the container
WORKDIR /usr/src/app

# Step 3: Install the protoc compiler and protobuf libraries
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

# Step 4: Copy the Cargo.toml and Cargo.lock files to leverage Docker caching
COPY Cargo.toml Cargo.lock ./

# Step 6: Copy the .sqlx directory with the metadata
COPY .sqlx .sqlx
COPY proto ./proto

# Step 7: Copy the rest of the application code
COPY . .

# Step 9: Build the project in release mode
RUN cargo build --release

# Step 10: Create the runtime image (alpine-based)
FROM debian:bookworm-slim

# Install dependencies, including the MySQL client
RUN apt-get update && \
    apt-get install -y libssl-dev libssl3 default-mysql-client && \
    apt-get clean

# Step 11: Copy the compiled Rust binary from the builder stage
COPY --from=builder /usr/src/app/target/release/user-oriochat-app /usr/local/bin/user-oriochat-app

# Expose both the HTTP and gRPC ports
EXPOSE 5000
EXPOSE 50051

# Set environment variables
ENV DATABASE_URL=mysql://root:rootx@172.18.0.1:3306/oriochat_user_db
ENV JWT_SECRET=12ZCDSGFERT4523

# Step 13: Set the command to run the application
CMD ["user-oriochat-app"]
