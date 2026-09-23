custom_target(
  'cts-api-coverage-jar',
  output: 'cts-api-coverage.jar',
  command: [
    find_program('python3'),
    meson.project_source_root() / 'tools/meson/build_java_jar.py',
    '--output', '@OUTPUT@',
    '--main-class', 'com.android.cts.apicoverage.CtsApiCoverage',
    '--source-root', meson.project_source_root() / 'tools/cts-api-coverage/src',
  ],
  build_by_default: true,
  install: false,
)
