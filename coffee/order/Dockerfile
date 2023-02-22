FROM rust:1.67-slim AS build

RUN USER=root cargo new --bin order
WORKDIR /order

COPY ./Cargo.* ./
RUN cargo build --release

COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim AS run

COPY --from=build /order/target/release/order /usr/local/bin/order

EXPOSE 8080
CMD ["order"]
