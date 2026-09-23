#
# Copyright (C) 2012 The Android Open Source Project
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

cpp = meson.get_compiler('cpp')

common_includes = include_directories(
  'compat/include',
  'lib/include',
  'lib/src',
  'executable/src',
)

lib_sources = run_command(
  'find',
  meson.current_source_dir() / 'lib/src',
  '-type',
  'f',
  '-name',
  '*.cpp',
  check: true,
).stdout().strip().split('\n')
lib_sources = lib_sources[0] != '' ? lib_sources : []

lib_deps = [
  dependency('threads', required: true),
  cpp.find_library('dl', required: false),
  cpp.find_library('rt', required: false),
  cpp.find_library('m', required: false),
  dependency('tinyalsa', required: false),
  dependency('tinyxml', required: false),
]

cts_audio_quality_lib = static_library(
  'cts_audio_quality',
  lib_sources,
  include_directories: common_includes,
  dependencies: lib_deps,
  install: false,
)

executable(
  'cts_audio_quality',
  'executable/src/main.cpp',
  include_directories: common_includes,
  link_with: cts_audio_quality_lib,
  dependencies: lib_deps,
  install: true,
)

if get_option('audio_quality_tests')
  test_sources = run_command(
    'find',
    meson.current_source_dir() / 'test',
    '-type',
    'f',
    '-name',
    '*.cpp',
    check: true,
  ).stdout().strip().split('\n')
  test_sources = test_sources[0] != '' ? test_sources : []

  gtest_dep = dependency('gtest_main', required: false)
  if not gtest_dep.found()
    gtest_dep = dependency('gtest', required: false)
  endif

  if gtest_dep.found() and test_sources.length() > 0
    executable(
      'cts_audio_quality_test',
      test_sources,
      include_directories: common_includes,
      link_with: cts_audio_quality_lib,
      dependencies: [lib_deps, gtest_dep],
      install: false,
    )
  endif
endif

install_subdir(
  'test_description',
  install_dir: join_paths(get_option('datadir'), 'android-cts-audio-quality'),
)
