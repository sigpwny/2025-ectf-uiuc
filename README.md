# 2025-ectf-uiuc

This repository contains the implementation of UIUC's secure satellite TV system for the MITRE eCTF 2025 competition.

## Design Document

We provide our design document in the `docs/` directory in both Markdown and [PDF](docs/uiuc-design-doc.pdf) formats.

## Repository Structure

- `decoder/`: Decoder implementation and shared crates
  - `common/`: Crate for shared code, such as structs and constants
  - `firmware-builder/`: Post-build tool to inject secrets into the firmware after the Decoder firmware is built
  - `max78000/`: The actual Decoder firmware implementation
  - `Dockerfile`: Sets up the build environment for the Decoder
  - `Makefile.toml`: Describes the flow for building the Decoder
- `design/`: Contains the wrapper Python module to install the `ectf25_design` package
  - `ectf25_design/`: Rust source code for the Encoder and generator functions (see [README](design/ectf25_design/README.md) for more info)
  - `wheels/`: Pre-compiled Rust wheels for the `ectf25_design` package
- `docs/`: Design document
- `tools/`: Organizer-provided host tools (see [docs](https://rules.ectf.mitre.org/2025/system/host_tools.html))

## Build System

See the [eCTF docs](https://rules.ectf.mitre.org/2025/system/host_tools.html) for more info on how to build the decoder using the standard eCTF 2025 build interfaces.

```
docker build -t decoder:uiuc .\decoder
docker run --rm -v .\decoder/:/decoder -v .\global.secrets:/global.secrets -v .\out:/out -e DECODER_ID=0xdeadbeef decoder:uiuc
```

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE.txt) file for details.