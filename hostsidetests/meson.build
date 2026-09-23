# hostsidetests/meson.build

cts_usb_tests_jar = custom_target(
  'cts-usb-tests-jar',
  output: 'CtsUsbTests.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'usb/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_monkey_test_cases_jar = custom_target(
  'cts-monkey-test-cases-jar',
  output: 'CtsMonkeyTestCases.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'monkey/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_aadb_test_cases_jar = custom_target(
  'cts-aadb-test-cases-jar',
  output: 'CtsAadbTestCases.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'aadb/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_sample_host_test_cases_jar = custom_target(
  'cts-sample-host-test-cases-jar',
  output: 'CtsSampleHostTestCases.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'sample/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_app_security_tests_jar = custom_target(
  'cts-app-security-tests-jar',
  output: 'CtsAppSecurityTests.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'appsecurity/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)

cts_holo_test_cases_jar = custom_target(
  'cts-holo-test-cases-jar',
  output: 'CtsHoloTestCases.jar',
  input: cts_tradefed_jar,
  command: [
    python,
    files('../tools/meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-root', meson.current_source_dir() / 'holo/src',
    '--dependency-jar', '@INPUT@',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/tradefed/tradefed-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/misc/common/ddmlib/ddmlib-prebuilt.jar',
    '--dependency-jar', meson.project_source_root() / 'tools/vm-tests-tf/lib/junit.jar',
  ],
  build_by_default: true,
  install: false,
)
