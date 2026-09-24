if get_option('enable_rust_jni')
  subdir('jni')
endif

# APK build target for CTS Verifier
if get_option('enable_rust_jni') and cargo.found()
  cts_verifier_apk = custom_target(
    'CtsVerifier-apk',
    output: 'CtsVerifier.apk',
    input: [ctsverifier_jni_so],
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_apk.py',
      '--output', '@OUTPUT@',
      '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
      '--res-dir', meson.current_source_dir() / 'res',
      '--assets-dir', meson.current_source_dir() / 'assets',
      '--source-root', meson.current_source_dir() / 'src',
      '--source-root', meson.project_source_root() / 'tests/tests/hardware/src/android/hardware/cts/helpers',
      '--jni-lib', '@INPUT0@',
    ],
    build_by_default: true,
    install: false,
  )
else
  cts_verifier_apk = custom_target(
    'CtsVerifier-apk',
    output: 'CtsVerifier.apk',
    command: [
      python,
      meson.project_source_root() / 'tools/meson/build_apk.py',
      '--output', '@OUTPUT@',
      '--manifest', meson.current_source_dir() / 'AndroidManifest.xml',
      '--res-dir', meson.current_source_dir() / 'res',
      '--assets-dir', meson.current_source_dir() / 'assets',
      '--source-root', meson.current_source_dir() / 'src',
      '--source-root', meson.project_source_root() / 'tests/tests/hardware/src/android/hardware/cts/helpers',
    ],
    build_by_default: true,
    install: false,
  )
endif
