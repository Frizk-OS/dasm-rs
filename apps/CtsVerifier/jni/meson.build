# Meson build for CTS Verifier JNI library rewritten in Rust

cargo = find_program('cargo', required: false)

if cargo.found()
  verifier_jni_manifest = meson.current_source_dir() / 'verifier_rs' / 'Cargo.toml'

  custom_target(
    'libctsverifier_jni',
    build_by_default: true,
    input: [
      'verifier_rs/Cargo.toml',
      'verifier_rs/src/lib.rs',
      'verifier_rs/src/jni_sys.rs',
    ],
    output: ['libctsverifier_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      verifier_jni_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'cts-verifier-jni-rust-tests',
    cargo,
    args: ['test', '--manifest-path', verifier_jni_manifest],
    workdir: meson.current_source_dir(),
  )
endif
