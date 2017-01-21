FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev && \
    mkdir /json

RUN apt-get install -y --no-install-recommends \
    gcc-multilib

COPY i686-unknown-linux-steed.json /json

RUN apt-get install -y --no-install-recommends \
    curl && \
    curl -LSfs http://japaric.github.io/trust/install.sh | \
    sh -s -- --git japaric/xargo --tag v0.3.5 --target x86_64-unknown-linux-gnu --to /usr/bin && \
    apt-get purge --auto-remove -y curl

ENV RUST_TARGET_PATH=/json
