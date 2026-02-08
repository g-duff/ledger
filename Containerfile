ARG RUST_VERSION=1.92
ARG APP=ledger


FROM rust:${RUST_VERSION} AS builder
ARG APP
WORKDIR /usr/src/${APP}
COPY . .
RUN cargo install --path .
CMD ["${APP}"]


FROM scratch
ARG APP
COPY --from=builder /usr/local/cargo/bin/${APP} /
