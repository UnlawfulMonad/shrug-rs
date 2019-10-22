FROM rust:1.38.0 AS build
RUN mkdir /build
WORKDIR /build

COPY . /build

RUN cargo build --release

FROM debian:10
COPY --from=build /build/target/release/shrug-rs /shrug-rs
EXPOSE 3000
CMD /shrug-rs
