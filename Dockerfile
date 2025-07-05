FROM rust:bullseye AS build

RUN USER=root cargo new --bin server_handler
WORKDIR /server_handler
RUN mkdir src/server_handler
RUN mv src/main.rs src/server_handler/main.rs

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN cargo build --bin server_handler --release
RUN rm src/server_handler/*.rs

COPY src src
COPY .sqlx .sqlx
COPY images images
COPY release-config.toml config.toml

RUN rm target/release/deps/server_handler*
RUN cargo build --bin server_handler --release

FROM debian:bullseye-slim
EXPOSE 8001

COPY --from=build /server_handler/target/release/server_handler .
COPY --from=build /server_handler/images images
COPY --from=build /server_handler/config.toml config.toml


CMD ["./server_handler"]
