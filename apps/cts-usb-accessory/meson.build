#
# Copyright (C) 2011 The Android Open Source Project
# Copyright (C) 2026 The AOSP and FrizkOS.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

cargo = find_program('cargo', required: false)

if cargo.found()
  custom_target(
    'cts-usb-accessory',
    build_by_default: true,
    input: [
      'Cargo.toml',
      'src/main.rs',
    ],
    output: ['cts-usb-accessory'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_bin.py',
      cargo,
      meson.current_source_dir() / 'Cargo.toml',
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    install: true,
    install_dir: get_option('bindir'),
    console: true,
  )

  test(
    'cts-usb-accessory-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', meson.current_source_dir() / 'Cargo.toml'],
    workdir: meson.current_source_dir(),
  )
else
  message('cargo not found, skipping cts-usb-accessory')
endif
