# Use the official Rust image
FROM rust:1.82

# Set working directory for the application
WORKDIR /app

# Copy the dependency files separately to leverage caching
COPY Cargo.toml Cargo.lock ./

# Install dependencies for your Rust project
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the rest of the application files
COPY . .

# Download dependencies (this will cache them if unchanged)
RUN cargo fetch

# Copy the wait-for-it.sh script
COPY wait-for-it.sh /usr/local/bin/wait-for-it.sh
RUN chmod +x /usr/local/bin/wait-for-it.sh

# Ensure MySQL is ready before building the application
RUN /usr/local/bin/wait-for-it.sh rust_mysql_db:3306 -- echo "MySQL is up, now starting build."


# Build your program for release
RUN cargo build --release

# Expose the port for your application
EXPOSE 5000

# Command to wait for MySQL to be ready, then run the app
CMD ["sh", "-c", "/usr/local/bin/wait-for-it.sh rust_mysql_db:3306 -- ./target/release/user-oriochat-app"]
