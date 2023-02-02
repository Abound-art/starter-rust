FROM rust:1.67-slim AS build

WORKDIR /build
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new abound
WORKDIR /build/abound
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /build/abound/target/x86_64-unknown-linux-musl/release/starter-rust /app

ENTRYPOINT ["/app"]
