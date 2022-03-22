FROM ekidd/rust-musl-builder as builder

WORKDIR /home/rust/

COPY . .
RUN sudo apt update -yq
RUN sudo apt-get install -yq python3
RUN cargo build --release --bin ascella

ENTRYPOINT ["./target/x86_64-unknown-linux-musl/release/ascella"]

FROM scratch
WORKDIR /home/rust/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/ascella .
ENTRYPOINT ["./ascella"]