FROM docker.io/clux/muslrust:1.68.2 as builder
RUN mkdir polkem_runner
WORKDIR ./polkem_runner

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release
RUN mv target/x86_64-unknown-linux-musl/release/polkem_runner /

FROM alpine:3.17.3
COPY --from=builder /polkem_runner /polkem_runner

RUN [ "./polkem_runner", "--version"]

ENTRYPOINT [ "./polkem_runner" ]
