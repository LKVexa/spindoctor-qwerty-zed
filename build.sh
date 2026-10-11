#!/usr/bin/env sh
# SpinDoctor Qwerty 1.1.0 - Zed extension
# Copyright (C) 2026 RUSSELL PHILIP SMITHSON <russellsmithson77@outlook.com>
# SPDX-License-Identifier: GPL-3.0-or-later
# This program is free software: you can redistribute it and/or modify it under the terms of the GNU General
# Public License as published by the Free Software Foundation, either version 3 of the License, or (at your
# option) any later version. It is distributed WITHOUT ANY WARRANTY; see the LICENSE file for details.
# Build the Zed extension WebAssembly module (extension.wasm) from source.
# Needs Rust via rustup; the wasm32-wasip2 target is added automatically.  Zed's
# "Install Dev Extension" also builds the crate itself, so this script is optional.
set -eu
cd "$(dirname "$0")"
rustup target add wasm32-wasip2
cargo test --quiet
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/spindoctor_qwerty.wasm extension.wasm
echo "built extension.wasm"
