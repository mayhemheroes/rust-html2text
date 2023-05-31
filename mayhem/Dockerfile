FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /rust-html2text
WORKDIR /rust-html2text/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /rust-html2text/fuzz/target/x86_64-unknown-linux-gnu/release/html2text-fuzz /