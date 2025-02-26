# Decoder Build System

> [!NOTE]  
> To build the decoder using Docker via the standard eCTF 2025 build system, see the documentation in the [top-level README](../README.md).

First, install the prerequisite tools:
```sh
cargo install cargo-make cargo-binutils
```

Then, build the decoder:
```sh
cargo make --profile production --env DECODER_ID=0xdeadbeef
```

This will first build the decoder firmware in `max78000/`, then build the `firmware-builder` tool in `firmware-builder/`, and finally run the `firmware-builder` tool to inject the deployment secrets into the firmware.