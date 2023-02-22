FROM rust:1.67-slim AS build

RUN USER=root cargo new --bin application
WORKDIR /application

COPY ./Cargo.* ./
RUN cargo build --release

COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim AS run

COPY --from=build /application/target/release/cart /usr/local/bin/cart

CMD ["cart"]
