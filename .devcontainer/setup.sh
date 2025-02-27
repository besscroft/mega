
## Install LLVM
wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -

## update and install some things we should probably have
apt-get update
apt-get -y upgrade
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  build-essential \
  openssl \
  libssl-dev \
  fuse3 \
  libfuse3-dev \
  pkg-config \
  postgresql \
  cmake \
  nodejs \
  npm \
  wget \
  file \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  ca-certificates \
  zstd \
  clang-18 \
  lldb-18 \
  lld-18 \
  libllvm-18-ocaml-dev \
  libllvm18 \
  llvm-18 \
  llvm-18-dev \
  llvm-18-doc \
  llvm-18-examples \
  llvm-18-runtime \
  libgtk4-dev \
  libadwaita-1-0 \
  libadwaita-1-dev

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup install default
rustup component add rustfmt
rustup component add clippy
source $HOME/.cargo/env
cargo install cargo-expand
cargo install cargo-edit

## Install Buck2 and Reindeer
wget https://github.com/facebook/buck2/releases/download/2025-02-01/buck2-x86_64-unknown-linux-musl.zst
zstd -d /home/buck2-x86_64-unknown-linux-musl.zst
mv /home/buck2-x86_64-unknown-linux-musl /home/buck2
chmod +x /home/buck2
mv /home/buck2 /usr/local/bin/buck2
cargo install --locked --git https://github.com/facebookincubator/reindeer reindeer