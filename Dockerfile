FROM rust:1.71.0-bookworm AS chef

ENV CARGO_REGISTRIES_CARTESI_INDEX=https://github.com/cartesi/crates-index
RUN rustup component add rustfmt
RUN cargo install cargo-chef

FROM chef AS planner
WORKDIR /app
COPY . /app/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --test cartesi_machine_client_tests

FROM --platform=linux/amd64 marcelopolitzer/output-unification:latest

COPY --from=cartesi/linux-kernel:0.17.0 /opt/riscv/kernel/artifacts/linux-5.15.63-ctsi-2-v0.17.0.bin /opt/cartesi/share/images/linux.bin
COPY --from=cartesi/rootfs:0.18.0 /opt/riscv/rootfs/artifacts/rootfs.ext2 /opt/cartesi/share/images/

USER root
RUN apt-get update && \
    apt-get install -y procps wget

RUN \
    wget -O /opt/cartesi/share/images/rom.bin https://github.com/cartesi/machine-emulator-rom/releases/download/v0.17.0/rom-v0.17.0.bin

COPY --from=builder /app/target/release/ /root/target/release/
COPY ./ /root
WORKDIR /root

ENTRYPOINT [ "/root/entrypoint.sh" ]
