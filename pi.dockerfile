FROM rust:1.62 as build
RUN rustup target add armv7-unknown-linux-musleabihf
RUN apt-get update && apt-get -y install binutils-arm-linux-gnueabihf
WORKDIR /shark
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY .cargo/pi_config ./.cargo/config
RUN cargo build --release  --target armv7-unknown-linux-musleabihf


FROM --platform=linux/arm debian:buster-slim
WORKDIR /shark
COPY --from=build /shark/target/armv7-unknown-linux-musleabihf/release/shark ./
COPY ./www ./www
EXPOSE 80
CMD ["./shark"]