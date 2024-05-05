FROM rust:1.61 as builder

WORKDIR /usr/src/gitautosync

COPY ./Cargo.toml ./Cargo.lock ./

COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/gitautosync/target/release/gitautosync /usr/local/bin/gitautosync

CMD ["gitautosync"]
