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

test_device_setup_apk = custom_target(
  'TestDeviceSetup-apk',
  output: 'TestDeviceSetup.apk',
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_apk.py',
    '--output', '@OUTPUT@',
    '--manifest', meson.current_source_dir() / 'TestDeviceSetup/AndroidManifest.xml',
    '--source-root', meson.current_source_dir() / 'TestDeviceSetup/src',
  ],
  build_by_default: true,
  install: false,
)
