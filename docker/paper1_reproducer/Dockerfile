# Paper 1 v2 reproducibility image.
# Usage:
#   docker build -t turingosv4-paper1 docker/paper1_reproducer
#   docker run --rm -e DEEPSEEK_API_KEY=... -v $PWD/out:/out turingosv4-paper1

FROM rust:1.83-bookworm

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
        git curl ca-certificates python3 python3-pip build-essential pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Lean 4 via elan (pins toolchain from repo's lean-toolchain file).
RUN curl -sSfL https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh \
        | bash -s -- -y --default-toolchain none
ENV PATH="/root/.elan/bin:${PATH}"

WORKDIR /work
COPY . /work/turingosv4
WORKDIR /work/turingosv4

# Respect lean-toolchain pin if present.
RUN (test -f lean-toolchain && elan toolchain install "$(cat lean-toolchain)" || true)

# Build the evaluator binary (release).
RUN cargo build --release -p minif2f_v4 --bin evaluator

# Stamp BUILD_SHA from bundled snapshot (git metadata may not be present inside image).
RUN echo "BUILD_SHA=$(git rev-parse --short HEAD 2>/dev/null || echo paper1_v2_reproducer)" > /work/build_sha.env

ENTRYPOINT ["/work/turingosv4/docker/paper1_reproducer/entrypoint.sh"]
CMD []
