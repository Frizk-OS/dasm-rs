# Meson build for core CTS JNI library rewritten in Rust

cargo = find_program('cargo', required: false)

if cargo.found()
  cts_jni_manifest = meson.current_source_dir() / 'libcts_jni_rs' / 'Cargo.toml'

  custom_target(
    'libcts_jni',
    build_by_default: true,
    input: [
      'libcts_jni_rs/Cargo.toml',
      'libcts_jni_rs/src/lib.rs',
      'libcts_jni_rs/src/jni_sys.rs',
    ],
    output: ['libcts_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      cts_jni_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'cts-jni-rust-tests',
    cargo,
    args: ['test', '--manifest-path', cts_jni_manifest],
    workdir: meson.current_source_dir(),
  )
endif
