FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /usr/src

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin minecraft_exporter

FROM scratch AS runtime
COPY --from=builder /usr/src/target/release/minecraft_exporter /minecraft_exporter
ENTRYPOINT ["/minecraft_exporter"]