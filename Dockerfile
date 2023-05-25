FROM docker.io/clux/muslrust:1.68.2 as builder
RUN mkdir polkem-runner
WORKDIR ./polkem-runner

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release
RUN mv target/x86_64-unknown-linux-musl/release/polkem-runner /

FROM alpine:3.17.3
COPY --from=builder /polkem-runner /polkem-runner

RUN [ "./polkem-runner", "--version"]

ENTRYPOINT [ "./polkem-runner" ]
