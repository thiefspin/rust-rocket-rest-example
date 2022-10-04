################
##### Builder
FROM rust:1.61.0-slim as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new hello-rocket

# We want dependencies cached, so copy those first.
COPY Rocket.toml Cargo.toml Cargo.lock /usr/src/hello-rocket/

# Set the working directory
WORKDIR /usr/src/hello-rocket

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/hello-rocket/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/hello-rocket/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:3.16.0 AS runtime 

# Copy application binary from builder image
COPY --from=builder /usr/src/hello-rocket/target/x86_64-unknown-linux-musl/release/hello-rocket /usr/local/bin
COPY --from=builder /usr/src/hello-rocket/Rocket.toml Rocket.toml

EXPOSE 3030

# Run the application
CMD ["/usr/local/bin/hello-rocket"]