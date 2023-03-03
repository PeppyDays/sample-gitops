FROM rust:1.67-slim AS build

RUN rustup toolchain install nightly
RUN cargo +nightly install -Z sparse-registry

RUN USER=root cargo new --bin cart
WORKDIR /cart

COPY ./Cargo.* ./
RUN cargo build --release

COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim AS run

COPY --from=build /cart/target/release/cart /usr/local/bin/cart

EXPOSE 8080
CMD ["cart"]
