FROM rust:1.27
RUN mkdir /src
WORKDIR /src

COPY Cargo.* /src/
RUN mkdir /src/src
COPY src/ /src/src

RUN cargo build --release && \
    cp /src/target/release/shrug-rs / && \
    cargo clean

EXPOSE 3000
CMD /shrug-rs
