FROM ubuntu:20.04

# setup APT mirror
RUN echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ focal main restricted universe multiverse" > /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ focal-updates main restricted universe multiverse" >> /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ focal-backports main restricted universe multiverse" >> /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/ubuntu/ focal-security main restricted universe multiverse" >> /etc/apt/sources.list

# install LLVM's APT
RUN apt update && \
  apt install -y wget gnupg && \
  wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/llvm-apt/focal/ llvm-toolchain-focal main" >> /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/llvm-apt/focal/ llvm-toolchain-focal-12 main" >> /etc/apt/sources.list && \
  echo "deb http://mirrors.tuna.tsinghua.edu.cn/llvm-apt/focal/ llvm-toolchain-focal-13 main" >> /etc/apt/sources.list

# install necessary packages
RUN apt update && apt install -y \
  git flex bison python3 \
  make cmake \
  qemu-user-static \
  clang-13 lldb-13 lld-13

# setup LLVM toolchain
WORKDIR /root
COPY update-alternatives-clang.sh .
RUN bash update-alternatives-clang.sh 13 100 && rm update-alternatives-clang.sh

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

# install libkoopa
# TODO

# install koopac
# TODO

# install SysY runtime library
# TODO

# install autotest and open-test-cases
# TODO

WORKDIR /root
