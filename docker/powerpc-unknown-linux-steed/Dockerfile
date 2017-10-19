FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev && \
    mkdir /json

COPY xargo.sh /
RUN bash /xargo.sh

COPY qemu.sh /
RUN apt-get install -y --no-install-recommends \
    gcc-powerpc-linux-gnu && \
    bash /qemu.sh 2.10.1 ppc

COPY powerpc-unknown-linux-steed.json /json

ENV CARGO_TARGET_POWERPC_UNKNOWN_LINUX_STEED_RUNNER=qemu-ppc \
    RUST_TARGET_PATH=/json \
    RUST_TEST_THREADS=1
