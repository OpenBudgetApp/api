FROM rust:1.64.0-alpine3.16 AS builder
# Install dependencies
RUN apk add --no-cache musl-dev sqlite-dev git
# Compile application
WORKDIR /api
COPY src src
COPY migrations migrations
COPY Cargo.toml Cargo.lock diesel.toml ./
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

FROM alpine:3.16.2
COPY --from=builder /api/target/release/oba_api /usr/local/bin/oba_api
# Install dependencies
RUN apk add --no-cache musl-dev sqlite-dev libgcc
# Add user oba
RUN adduser -S -D -h /oba oba
USER oba
WORKDIR /oba
ENV DATABASE_URL=/oba/db.sqlite
# Run the server
EXPOSE 8000
ENV RUST_BACKTRACE=1
ENV ROCKET_ADDRESS=0.0.0.0
ENTRYPOINT ["/usr/local/bin/oba_api"]
