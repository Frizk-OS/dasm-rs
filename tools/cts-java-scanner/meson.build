custom_target(
  'cts-java-scanner-jar',
  output: 'cts-java-scanner.jar',
  command: [
    find_program('python3'),
    meson.project_source_root() / 'tools/meson/build_java_jar.py',
    '--output', '@OUTPUT@',
    '--main-class', 'com.android.cts.javascanner.CtsJavaScanner',
    '--source-root', meson.project_source_root() / 'tools/cts-java-scanner/src',
  ],
  build_by_default: true,
  install: false,
)
