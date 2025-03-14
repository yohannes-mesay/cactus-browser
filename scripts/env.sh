#!/usr/bin/env bash
# Local GTK/WebKit build env using .deps/prefix (dev headers) + system runtime libs.
# Usage: source scripts/env.sh
#        source scripts/env.sh --check

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PREFIX="${ROOT}/.deps/prefix"
SYS_LIB="/usr/lib/x86_64-linux-gnu"
PREFIX_LIB="${PREFIX}/usr/lib/x86_64-linux-gnu"

if [ -f "${HOME}/.cargo/env" ]; then
  # shellcheck source=/dev/null
  source "${HOME}/.cargo/env"
fi

export PKG_CONFIG_PATH="${PREFIX_LIB}/pkgconfig:${PREFIX}/usr/share/pkgconfig"
# gcc/collect2 search path for -l (unversioned .so symlinks live in PREFIX_LIB)
export LIBRARY_PATH="${PREFIX_LIB}:${SYS_LIB}${LIBRARY_PATH:+:${LIBRARY_PATH}}"
export LD_LIBRARY_PATH="${SYS_LIB}:${PREFIX_LIB}${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}"
export PATH="${HOME}/.cargo/bin:${PATH}"
# rust-lld does not reliably honor LIBRARY_PATH; pass native search dirs explicitly
export RUSTFLAGS="-Lnative=${PREFIX_LIB} -Lnative=${SYS_LIB}${RUSTFLAGS:+ ${RUSTFLAGS}}"

if [ "${1:-}" = "--check" ]; then
  pkg-config --exists --print-errors gtk4 libadwaita-1 webkitgtk-6.0 sqlite3 && echo ALL_OK
  pkg-config --modversion gtk4 libadwaita-1 webkitgtk-6.0 sqlite3
fi
