FROM ekidd/rust-musl-builder as builder

ADD --chown=rust:rust . ./
RUN cargo build --release

FROM scratch

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/cet_service /cet_service

ENTRYPOINT ["/cet_service"]