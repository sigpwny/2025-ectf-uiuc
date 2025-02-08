# Dockerfile for the 2025 eCTF SIGPwny design

# Get latest version of rust
FROM rust:latest

WORKDIR /decoder

# Install Requisite Packages 
# Step 1
RUN apt-get update && apt-get install -y \
    gdb \
    gcc-arm-none-eabi \
    binutils-arm-none-eabi \
    make \
    python3.10 \
    libudev-dev \
    wget \
    unzip    

# Install MSDK
# Step 2
RUN apt-get install -y unzip && \
    wget https://github.com/analogdevicesinc/msdk/archive/refs/tags/v2024_02.zip && \
    unzip v2024_02.zip && \
    rm -f v2024_02.zip

ENV MAXIM_PATH=/root/msdk-2024_02

# ENTRYPOINT ["bash", "-c", "make release DECODER_ID=${DECODER_ID} && cp build/max78000.elf build/max78000.bin /out"]
ENTRYPOINT ["cargo", "build", "--release"]

# Sample run command:
# docker run --rm -v ./build_out:/out -v ./:/decoder -v ./secrets:/secrets -e DECODER_ID=0xdeadbeef decoderFROM rust:latest
