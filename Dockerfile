# First stage: Rust Builder
FROM rust:1.77 as builder

# Create a workspace
WORKDIR /usr/src/app

# Copy the manifests
COPY ./Cargo.toml ./
COPY ./Cargo.lock ./

COPY ./src ./src

# Build dependencies only to cache them
RUN mkdir -p ./target/release \
    && touch ./src/main.rs \
    && cargo build --release --bin tutorIA_back

# Build the actual application
COPY ./src ./src
RUN touch ./src/main.rs \
    && cargo build --release --bin tutorIA_back

# Second stage: Runtime
FROM debian:bookworm

# Install needed packages (e.g., for SSL)
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/tutorIA_back /usr/src/app/tutorIA_back

# Set permissions to execute
RUN chmod +x /usr/src/app/tutorIA_back

# Run the binary
CMD ["tutorIA_back"]
