# Meson build for dasm-rs (modern Dalvik assembler in Rust)

cargo = find_program('cargo', required: false)

if cargo.found()
  dasm_manifest = meson.current_source_dir() / 'Cargo.toml'

  dasm_bin = custom_target(
    'dasm-rs',
    build_by_default: true,
    input: [
      'Cargo.toml',
      'src/lib.rs',
      'src/main.rs',
      'src/opcode.rs',
      'src/parser.rs',
    ],
    output: ['dasm-rs'],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_cargo_bin.py',
      cargo,
      dasm_manifest,
      '@PRIVATE_DIR@/target',
      '@OUTPUT@',
    ],
    install: true,
    install_dir: get_option('bindir'),
    console: true,
  )

  test(
    'dasm-rs-cargo-unit-tests',
    cargo,
    args: ['test', '--manifest-path', dasm_manifest],
    workdir: meson.current_source_dir(),
  )

  test(
    'dasm-rs-parse-all-opcodes',
    dasm_bin,
    args: [meson.project_source_root() / 'tools/dasm/test/all_opcodes.d'],
  )
endif
