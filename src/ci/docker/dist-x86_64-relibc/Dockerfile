FROM ubuntu:19.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  file \
  curl \
  ca-certificates \
  python2.7 \
  git \
  xz-utils \
  unzip \
  sudo \
  gdb \
  make \
  g++ \
  cmake \
  pkg-config \
  libssl-dev \
  clang \
  lld

WORKDIR /build/
COPY scripts/relibc.sh /build/
RUN bash relibc.sh && \
    rm -rf /build

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

ENV RUST_CONFIGURE_ARGS \
#      --enable-full-tools \
#      --enable-sanitizers \
#      --enable-profiler \
#      --enable-compiler-docs \
      --set target.x86_64-unknown-linux-relibc.linker=x86_64-unknown-linux-relibc-clang \
      --set target.x86_64-unknown-linux-relibc.ar=/relibc/bin/llvm-ar \
      --set target.x86_64-unknown-linux-relibc.ranlib=/relibc/bin/llvm-ranlib \
#      --set llvm.thin-lto=true \
      --enable-extended \
      --disable-docs

ENV TARGETS=x86_64-unknown-linux-relibc

ENV CC_x86_64_unknown_linux_relibc=x86_64-unknown-linux-relibc-clang \
    CXX_x86_64_unknown_linux_relibc=x86_64-unknown-linux-relibc-clang++

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_RELIBC_LINKER=x86_64-unknown-linux-relibc-clang

ENV SCRIPT \
      python2.7 ../x.py test --target $TARGETS && \
      python2.7 ../x.py dist --target $TARGETS
