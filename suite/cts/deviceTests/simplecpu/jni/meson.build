# Meson build for CPU test cdylib rewritten in Rust

if cargo.found()
  cpu_manifest = meson.current_source_dir() / 'Cargo.toml'
  
  ctscpu_jni_so = custom_target(
    'libctscpu_jni',
    build_by_default: true,
    input: [
      'Cargo.toml',
      'src/lib.rs',
    ],
    output: ['libctscpu_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      cpu_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'ctscpu-jni-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', cpu_manifest],
    workdir: meson.current_source_dir(),
  )
endif
