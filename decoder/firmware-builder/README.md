# firmware-builder

Post-build tool to inject secrets into the firmware after the Decoder firmware is built.

```sh
$ cargo run --release -- --help
Usage: firmware-builder [OPTIONS] --firmware <FILE> --secrets <FILE> --decoder-id <DECODER_ID>

Options:
  -f, --firmware <FILE>          Path to the firmware binary
  -s, --secrets <FILE>           Path to the global secrets file
  -d, --decoder-id <DECODER_ID>  Decoder ID
  -o, --output <FILE>            Path to the output firmware binary [default: ./out/max78000.bin]
  -h, --help                     Print help
```