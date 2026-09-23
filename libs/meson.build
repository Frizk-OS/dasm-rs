# libs/meson.build

jsonlib_jar = custom_target(
  'jsonlib-jar',
  output: 'jsonlib.jar',
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'json/src',
  ],
  build_by_default: true,
  install: false,
)

ctscommonutil_jar = custom_target(
  'ctscommonutil-jar',
  output: 'ctscommonutil.jar',
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'commonutil/src',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

vogarexpectlib_jar = custom_target(
  'vogarexpectlib-jar',
  output: 'vogarexpectlib.jar',
  input: jsonlib_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'vogar-expect/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/devtools/tools/lib/guava-13.0.1.jar',
  ],
  build_by_default: true,
  install: false,
)

ctsdeviceutil_jar = custom_target(
  'ctsdeviceutil-jar',
  output: 'ctsdeviceutil.jar',
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'deviceutil/src',
    '--source-root', meson.current_source_dir() / 'commonutil/src',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/sdk/current/android.jar',
  ],
  build_by_default: true,
  install: false,
)

ctstestserver_jar = custom_target(
  'ctstestserver-jar',
  output: 'ctstestserver.jar',
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'testserver/src',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/sdk/current/android.jar',
  ],
  build_by_default: true,
  install: false,
)

wrappedgtest_jar = custom_target(
  'wrappedgtest-jar',
  output: 'wrappedgtest.jar',
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'wrappedgtest/src',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/sdk/current/android.jar',
  ],
  build_by_default: true,
  install: false,
)
