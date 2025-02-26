# Example usage:
# docker build -t decoder:uiuc .
# docker run -t --rm -v .\decoder/:/decoder -v .\global.secrets:/global.secrets -v .\out:/out -e DECODER_ID=0xdeadbeef decoder:uiuc

FROM rust:latest

WORKDIR /decoder

# Install cargo-make
RUN cargo install --force cargo-make cargo-binutils

# Install Rust toolchain targets and components
COPY rust-toolchain.toml rust-toolchain.toml
RUN rustup show

ENTRYPOINT ["bash", "-c", "cargo make --profile production --env DECODER_ID=${DECODER_ID} docker-flow"]
