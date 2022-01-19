# base image
FROM ubuntu:20.04 as compiler-dev-base

# some arguments
ARG KOOPA_REPO_URL=https://github.com/pku-minic/koopa.git
ARG KOOPAC=koopac.tar.gz
ARG SYSYRT_REPO_URL=https://github.com/pku-minic/sysy-runtime-lib.git
ARG AUTOTEST=autotest
ARG LIB_INSTALL_DIR=/opt/lib
ARG BIN_INSTALL_DIR=/opt/bin

# setup APT mirror
RUN echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ focal main restricted universe multiverse" > /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ focal-updates main restricted universe multiverse" >> /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ focal-backports main restricted universe multiverse" >> /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ focal-security main restricted universe multiverse" >> /etc/apt/sources.list

# install LLVM's APT
RUN apt update && \
  DEBIAN_FRONTEND="noninteractive" apt install -y wget gnupg && \
  wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/llvm-apt/focal/ llvm-toolchain-focal main" >> /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/llvm-apt/focal/ llvm-toolchain-focal-12 main" >> /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/llvm-apt/focal/ llvm-toolchain-focal-13 main" >> /etc/apt/sources.list

# install necessary packages
RUN apt update && DEBIAN_FRONTEND="noninteractive" apt install -y \
  git flex bison python3 \
  make cmake \
  qemu-user-static \
  clang-13 lldb-13 lld-13

# setup LLVM toolchain
WORKDIR /root
COPY update-alternatives-clang.sh .
RUN bash update-alternatives-clang.sh 13 100 && \
  rm update-alternatives-clang.sh

# install Rust toolchain
RUN wget -O - https://sh.rustup.rs | sh -s -- -y && \
  mkdir -p ~/.cargo && \
  echo "[source.crates-io]" > ~/.cargo/config && \
  echo "replace-with = 'tuna'" >> ~/.cargo/config && \
  echo "[source.tuna]" >> ~/.cargo/config && \
  echo "registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"" >> ~/.cargo/config
ENV RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
ENV RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
ENV PATH="/root/.cargo/bin:${PATH}"

# setup GitHub proxy
RUN git config --global url."https://hub.fastgit.org/".insteadOf "https://github.com/" && \
  git config --global protocol.https.allow always

# setup install directories
RUN mkdir -p ${LIB_INSTALL_DIR} ${BIN_INSTALL_DIR}
ENV CDE_LIBRARY_PATH="${LIB_INSTALL_DIR}"
ENV PATH="${BIN_INSTALL_DIR}:${PATH}"

# install libkoopa
WORKDIR ${LIB_INSTALL_DIR}
RUN git clone --single-branch --depth 1 ${KOOPA_REPO_URL} koopa && \
  cargo build --release --manifest-path koopa/crates/libkoopa/Cargo.toml && \
  mkdir -p native && \
  cp koopa/target/release/libkoopa.a koopa/target/release/libkoopa.so native && \
  rm -rf koopa

# install koopac
WORKDIR ${BIN_INSTALL_DIR}
COPY ${KOOPAC} .
RUN mkdir kpc && tar xzf ${KOOPAC} -C kpc && \
  cargo build --release --manifest-path kpc/Cargo.toml && \
  cp kpc/target/release/koopac . && \
  rm -rf ${KOOPAC} kpc

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
ARG AUTOTEST
ARG BIN_INSTALL_DIR
ARG LINK_DIR
ARG OPEN_TEST_CASES_REPO_URL=https://github.com/pku-minic/open-test-cases.git

# install autotest and open-test-cases
WORKDIR ${BIN_INSTALL_DIR}
COPY ${AUTOTEST} .
RUN git clone --recursive --shallow-submodules --single-branch --depth 1 \
  ${SYSYRT_REPO_URL} testcases

WORKDIR /root
