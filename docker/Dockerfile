# base image
FROM ubuntu:24.04 as compiler-dev-base

# some arguments
ARG KOOPA_REPO_URL=https://github.com/pku-minic/koopa.git
ARG KOOPAC=koopac.tar.gz
ARG SYSYRT_REPO_URL=https://github.com/pku-minic/sysy-runtime-lib.git
ARG AUTOTEST=autotest
ARG INSTALL_DIR=/opt
ARG LIB_INSTALL_DIR=${INSTALL_DIR}/lib
ARG INC_INSTALL_DIR=${INSTALL_DIR}/include
ARG BIN_INSTALL_DIR=${INSTALL_DIR}/bin

# setup APT mirror for all architectures
RUN sed -i "s@http://.*archive.ubuntu.com@http://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list.d/ubuntu.sources && \
  sed -i "s@http://.*security.ubuntu.com@http://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list.d/ubuntu.sources && \
  sed -i "s@http://.*ports.ubuntu.com@http://mirrors.tuna.tsinghua.edu.cn@g" /etc/apt/sources.list.d/ubuntu.sources

# install LLVM's APT repository
RUN apt update && \
  DEBIAN_FRONTEND="noninteractive" apt install -y wget && \
  wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key > /etc/apt/trusted.gpg.d/llvm.asc && \
  cat > /etc/apt/sources.list.d/llvm.sources << EOF
Types: deb
Architectures: amd64 arm64
Signed-By: /etc/apt/trusted.gpg.d/llvm.asc
URIs: http://apt.llvm.org/noble/
Suites: llvm-toolchain-noble-21
Components: main
EOF

# install necessary packages
RUN apt update && DEBIAN_FRONTEND="noninteractive" apt install -y \
  git flex bison python3 \
  make cmake \
  qemu-user-static \
  clang-21 lldb-21 lld-21

# setup LLVM toolchain
WORKDIR /root
COPY update-alternatives-clang.sh .
RUN bash update-alternatives-clang.sh 21 100 && \
  rm update-alternatives-clang.sh

# install Rust toolchain
RUN wget -O - https://sh.rustup.rs | sh -s -- -y && \
  mkdir -p ~/.cargo && \
  cat > ~/.cargo/config.toml << EOF
[source.crates-io]
replace-with = 'mirror'
[source.mirror]
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
[registries.mirror]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
EOF
ENV RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
ENV RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
ENV PATH="/root/.cargo/bin:${PATH}"

# setup install directories
RUN mkdir -p ${LIB_INSTALL_DIR} ${BIN_INSTALL_DIR}
ENV CDE_LIBRARY_PATH="${LIB_INSTALL_DIR}"
ENV CDE_INCLUDE_PATH="${INC_INSTALL_DIR}"
ENV PATH="${BIN_INSTALL_DIR}:${PATH}"

# install libkoopa
WORKDIR ${LIB_INSTALL_DIR}
RUN git clone --single-branch --depth 1 ${KOOPA_REPO_URL} koopa && \
  CARGO_PROFILE_RELEASE_PANIC=abort cargo build -r -p libkoopa --manifest-path koopa/Cargo.toml && \
  mkdir -p native && \
  cp koopa/target/release/libkoopa.a native && \
  mkdir -p ${INC_INSTALL_DIR} && \
  cp -R koopa/libkoopa/include/* ${INC_INSTALL_DIR}/. && \
  rm -rf koopa /root/.cargo/registry

# install koopac
WORKDIR ${INSTALL_DIR}
COPY ${KOOPAC} .
RUN mkdir kpc && tar xzf ${KOOPAC} -C kpc && \
  cargo install --path kpc --root ${INSTALL_DIR} --no-track && \
  rm -rf ${KOOPAC} kpc /root/.cargo/registry

# install SysY runtime library
WORKDIR ${LIB_INSTALL_DIR}
RUN git clone --single-branch --depth 1 ${SYSYRT_REPO_URL} sysyrt && \
  make -C sysyrt libsysy && \
  mkdir -p native && cp sysyrt/build/libsysy.a native && \
  make NO_LIBC=1 \
       ADD_CFLAGS="-target riscv32-unknown-linux-elf -march=rv32im -mabi=ilp32" \
       -C sysyrt clean libsysy && \
  mkdir -p riscv32 && cp sysyrt/build/libsysy.a riscv32 && \
  rm -rf sysyrt


# compiler development environment image
FROM compiler-dev-base as compiler-dev

# some arguments
ARG TEST_CASES_REPO_URL=https://github.com/pku-minic/compiler-dev-test-cases.git

# install autotest and test cases
WORKDIR ${BIN_INSTALL_DIR}
COPY ${AUTOTEST} .
RUN git clone --single-branch --depth 1 ${TEST_CASES_REPO_URL} cd-test-cases && \
  make LIB_DIR=${LIB_INSTALL_DIR}/native INSTALL_DIR=${BIN_INSTALL_DIR}/testcases \
       -C cd-test-cases install -j`nproc` && \
  rm -rf cd-test-cases

WORKDIR /root
