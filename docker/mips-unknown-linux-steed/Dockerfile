FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev && \
    mkdir /json

COPY xargo.sh /
RUN bash /xargo.sh

COPY lld.sh /
RUN bash /lld.sh

COPY qemu.sh /
RUN bash /qemu.sh 2.10.1 mips

COPY mips-unknown-linux-steed.json /json

ENV CARGO_TARGET_MIPS_UNKNOWN_LINUX_STEED_RUNNER=qemu-mips \
    RUST_TARGET_PATH=/json \
    RUST_TEST_THREADS=1
