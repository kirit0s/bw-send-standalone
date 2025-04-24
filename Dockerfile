FROM hub.intechs.by/rust:1.85 AS build

WORKDIR /code
RUN USER=root cargo init
COPY Cargo.toml Cargo.lock ./
# RUN cargo fetch

COPY src src
RUN cargo install wasm-pack
# RUN cargo test --release
RUN wasm-pack build --target web --out-dir ./src/web-ui/pkg


FROM joseluisq/static-web-server
WORKDIR /app

COPY --from=build /code/src/web-ui ./
USER 1001

CMD ["-p", "80", "-d", "./"]

