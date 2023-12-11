# Use the official Rust image as the build environment
FROM rust AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# precache deps?

# Copy the source code to the working directory
COPY src ./src

# Build the application
RUN cargo build --release --bin ssrf

EXPOSE 8000

WORKDIR /app/target/release

RUN mkdir -p ./content\
    && chmod +rx ./ssrf

CMD ["sh", "-c", "pwd; echo; ls; echo; ./ssrf"]

# Use a minimal Alpine image as the runtime environment
# FROM alpine

# # Set the working directory
# WORKDIR /app
# EXPOSE 8000

# # Copy the binary from the builder stage to the runtime environment
# COPY --from=builder /app/target/release/ssrf .

# RUN mkdir -p ./content\
#     && chmod +rx ./ssrf

# # Run the binary
# CMD ["./ssrf"]
