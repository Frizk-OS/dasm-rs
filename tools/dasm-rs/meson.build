#
# Copyright (C) 2026 The AOSP and FrizkOS.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

project(
  'dasm-rs',
  'rust',
  version: '0.1.0',
  meson_version: '>=1.4.0',
  default_options: [
    'rust_std=2024',
    'warning_level=2',
    'buildtype=debugoptimized',
  ],
)

dasm_rs_lib = static_library(
  'dasm_rs',
  'src/lib.rs',
  rust_abi: 'rust',
)

dasm_bin = executable(
  'dasm-rs',
  'src/main.rs',
  link_with: dasm_rs_lib,
  install: true,
  install_dir: get_option('bindir'),
)

cargo = find_program('cargo', required: false)
if cargo.found()
  test(
    'dasm-rs-cargo-unit-tests',
    cargo,
    args: ['test', '--manifest-path', meson.current_source_dir() / 'Cargo.toml'],
    workdir: meson.current_source_dir(),
  )
endif

all_opcodes_test = meson.current_source_dir() / '../dasm/test/all_opcodes.d'
test(
  'dasm-rs-parse-all-opcodes',
  dasm_bin,
  args: [all_opcodes_test],
)
