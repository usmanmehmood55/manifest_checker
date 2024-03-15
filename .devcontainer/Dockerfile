# Official Rust image
FROM rust:latest as rust-builder

# Switch to Debian Buster for compatibility with GLIBC 2.28
FROM debian:buster

RUN apt-get update &&                                                                      \
    apt-get install -y build-essential lldb gcc-arm-linux-gnueabihf git ca-certificates && \
    update-ca-certificates &&                                                              \
    rm -rf /var/lib/apt/lists/*

# Copy Rust toolchain from rust-builder
COPY --from=rust-builder /usr/local/cargo /usr/local/cargo
COPY --from=rust-builder /usr/local/rustup /usr/local/rustup

# Set environment variables for Rust
ENV PATH=/usr/local/cargo/bin:$PATH
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo

WORKDIR /usr/src/app

COPY . .

RUN rustup target add armv7-unknown-linux-gnueabihf