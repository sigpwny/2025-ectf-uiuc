# Example usage:
# docker build -t decoder:uiuc .
# docker run -t --rm -v .\decoder/:/decoder -v .\global.secrets:/global.secrets -v .\out:/out -e DECODER_ID=0xdeadbeef decoder:uiuc

FROM rust:latest

WORKDIR /decoder

RUN apt-get update && apt-get install -y curl unzip && rm -rf /var/lib/apt/lists/*

# Install cargo-make
RUN cargo install --force cargo-make

# Install cargo-binutils
# RUN cargo install cargo-binutils
ARG CARGO_BINUTILS_VERSION=0.3.6
ARG CARGO_BINUTILS_TARGET
RUN set -eux; \
    : "${CARGO_BINUTILS_TARGET:=$(rustc -vV | sed -n 's|host: ||p')}"; \
    URL="https://github.com/rust-embedded/cargo-binutils/releases/download/v${CARGO_BINUTILS_VERSION}/${CARGO_BINUTILS_TARGET}.tar.gz"; \
    curl -L "$URL" | tar -xz -C /usr/local/bin --strip-components=1; \
    chmod +x /usr/local/bin/*

# Install Rust toolchain targets and components
COPY rust-toolchain.toml rust-toolchain.toml
RUN rustup show

ENTRYPOINT ["bash", "-c", "cargo make --profile production --env DECODER_ID=${DECODER_ID} docker-flow"]
