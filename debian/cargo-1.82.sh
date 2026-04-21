#!/bin/bash -e
RUSTC="${RUSTC:-/usr/lib/rust-1.82/bin/rustc}" exec /usr/lib/rust-1.82/bin/cargo "$@"
