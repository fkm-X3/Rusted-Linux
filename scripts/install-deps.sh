#!/bin/bash
set -euo pipefail

KERNEL_DIR="$(cd "$(dirname "$0")/../kernel-src" && pwd)"

command_exists() { command -v "$1" &>/dev/null; }

detect_pkg_manager() {
  if command_exists apt-get; then
    echo "apt"
  elif command_exists dnf; then
    echo "dnf"
  elif command_exists yum; then
    echo "yum"
  elif command_exists pacman; then
    echo "pacman"
  elif command_exists zypper; then
    echo "zypper"
  else
    echo "unknown"
  fi
}

get_kernel_deps_apt() {
  cat <<EOF
build-essential
flex
bison
libelf-dev
libssl-dev
bc
cpio
rsync
gzip
xz-utils
zstd
lz4
lzop
bzip2
pahole
kmod
libncurses-dev
python3
python3-dev
python3-setuptools
python3-pip
git
EOF
}

get_kernel_deps_dnf() {
  cat <<EOF
gcc
make
flex
bison
elfutils-libelf-devel
openssl-devel
bc
cpio
rsync
gzip
xz
zstd
lz4
lzo
bzip2
dwarves
kmod
ncurses-devel
python3
python3-devel
python3-setuptools
python3-pip
git
EOF
}

install_apt() {
  echo "Detected apt-based system (Debian/Ubuntu)"
  sudo apt-get update
  get_kernel_deps_apt | xargs sudo apt-get install -y
}

install_dnf() {
  echo "Detected dnf-based system (Fedora)"
  get_kernel_deps_dnf | xargs sudo dnf install -y
}

install_yum() {
  echo "Detected yum-based system (RHEL/CentOS)"
  get_kernel_deps_dnf | xargs sudo yum install -y
}

install_pacman() {
  echo "Detected pacman-based system (Arch)"
  sudo pacman -S --needed base-devel flex bison elfutils openssl bc cpio rsync \
    gzip xz zstd lz4 lzo bzip2 kmod ncurses python python-pip git
}

install_zypper() {
  echo "Detected zypper-based system (openSUSE)"
  get_kernel_deps_dnf | xargs sudo zypper install -y
}

install_rust_deps() {
  if command_exists rustup; then
    rustup component add rust-src
  fi
  if command_exists cargo; then
    cargo install bindgen-cli
  fi
}

if [ "$(uname -s)" != "Linux" ]; then
  echo "This script must run inside a WSL/Linux environment, not on Windows."
  exit 1
fi

PKG_MANAGER=$(detect_pkg_manager)
echo "Package manager: $PKG_MANAGER"

case "$PKG_MANAGER" in
  apt)    install_apt ;;
  dnf)    install_dnf ;;
  yum)    install_yum ;;
  pacman) install_pacman ;;
  zypper) install_zypper ;;
  *)
    echo "Unsupported package manager. Install dependencies manually:"
    echo "  build-essential flex bison libelf-dev libssl-dev bc cpio rsync"
    echo "  gzip xz-utils zstd lz4 lzop bzip2 pahole kmod libncurses-dev python3 git"
    exit 1
    ;;
esac

if [ -f "$KERNEL_DIR/.config" ] && grep -q 'CONFIG_RUST=y' "$KERNEL_DIR/.config" 2>/dev/null; then
  install_rust_deps
fi

echo "All kernel build dependencies installed."
