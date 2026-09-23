dex_tools_jar = custom_target(
  'dex-tools-jar',
  output: 'dex-tools.jar',
  command: [
    python,
    files('../meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'src',
  ],
  build_by_default: true,
  install: false,
)
