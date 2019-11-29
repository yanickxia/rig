FROM rust:1.39.0
WORKDIR /build
ADD . .
ADD cargo_config /root/.cargo/config
RUN cargo build --release

FROM debian:buster
WORKDIR /app
ENV RUST_LOG=info
ADD config/ /app/config/
COPY --from=0 /build/target/release/rig-demo .
CMD rig-demo