FROM rust:latest

RUN apt-get update && apt-get install -y binutils-arm-none-eabi

WORKDIR /decoder

# ENTRYPOINT ["bash", "-c", "make release DECODER_ID=${DECODER_ID} && cp build/max78000.elf build/max78000.bin /out"]
ENTRYPOINT ["cargo", "build", "--release"]

# Sample run command:
# docker run --rm -v ./build_out:/out -v ./:/decoder -v ./secrets:/secrets -e DECODER_ID=0xdeadbeef decoder