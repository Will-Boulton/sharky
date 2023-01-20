FROM rust:1.62 as build
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release


FROM debian:buster-slim
COPY --from=build ./target/release/shark ./
COPY ./www ./www
EXPOSE 80
CMD ["./shark"]