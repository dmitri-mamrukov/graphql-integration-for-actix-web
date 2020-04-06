# Use the latest Rust
FROM rust

WORKDIR /usr/src/app

# Bundle the app source.
COPY . .

# Build the app.
RUN cargo install --path .

# Start the app.
CMD ["server"]
