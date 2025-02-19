# Decoder

TODO: Add folder structure here

## Decoder Build System

> [!NOTE]  
> To build the decoder using Docker via the standard eCTF 2025 build system, see the documentation in the [top-level README](../README.md).

First, install the prerequisite tools:
```sh
cargo install cargo-make cargo-binutils
```

Then, build the decoder:
```sh
cargo make --env DECODER_ID=0xdeadbeef
```

TODO