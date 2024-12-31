# Stage 1: Build
FROM rust as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the project files into the container
COPY . .

# Build the project in release mode
RUN cargo build

# Stage 2: Runtime
FROM rust

# Install required libraries for runtime (e.g., for PostgreSQL)
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the pre-built binaries from the build stage
COPY --from=builder /usr/src/app/target/debug/batch-explorer .
COPY --from=builder /usr/src/app/target/debug/migration .
COPY static static
COPY .env* .
# Expose the application port (default is 3000 for batch-explorer)
EXPOSE 3000

# Default command for the runtime container
CMD ["./batch-explorer"]