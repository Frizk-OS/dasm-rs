# Meson build for DRAM test cdylib rewritten in Rust

if cargo.found()
  dram_manifest = meson.current_source_dir() / 'Cargo.toml'
  
  ctsdram_jni_so = custom_target(
    'libctsdram_jni',
    build_by_default: true,
    input: [
      'Cargo.toml',
      'src/lib.rs',
    ],
    output: ['libctsdram_jni.so'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_cdylib.py',
      cargo,
      dram_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    console: true,
  )

  test(
    'ctsdram-jni-cargo-tests',
    cargo,
    args: ['test', '--manifest-path', dram_manifest],
    workdir: meson.current_source_dir(),
  )
endif
