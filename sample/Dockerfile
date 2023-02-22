FROM rust:1.67-slim AS build

RUN USER=root cargo new --bin sample
WORKDIR /sample

COPY ./Cargo.* ./
RUN cargo build --release

COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim AS run

COPY --from=build /sample/target/release/sample /usr/local/bin/sample

CMD ["sample"]
