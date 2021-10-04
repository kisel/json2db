FROM rust:1.55 AS build
WORKDIR /ws
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install libssl-dev pkg-config musl-tools

COPY src ./src
COPY Cargo* ./
RUN cargo build --release --target  x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/json2db

FROM alpine
WORKDIR /app
COPY --from=build /ws/target/x86_64-unknown-linux-musl/release/json2db /app/json2db
CMD /app/json2db

