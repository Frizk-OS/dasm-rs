ctsdeviceinfolib_jar = custom_target(
  'ctsdeviceinfolib-jar',
  output: 'ctsdeviceinfolib.jar',
  command: [
    python,
    files('../meson/build_java_jar.py'),
    '--output', '@OUTPUT@',
    '--source-file', meson.current_source_dir() / 'TestDeviceSetup/src/android/tests/getinfo/DeviceInfoConstants.java',
  ],
  build_by_default: true,
  install: false,
)
