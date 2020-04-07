# Use the latest Rust
FROM rust

WORKDIR /usr/src/app

# Bundle the app source.
COPY . .

# Build the app.
RUN cargo install --path .

EXPOSE 8000

# Start the app.
CMD ["crud_server_sample"]
