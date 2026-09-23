custom_target(
  'cts-xml-generator-jar',
  output: 'cts-xml-generator.jar',
  input: [vogarexpectlib_jar, jsonlib_jar],
  command: [
    python,
    meson.project_source_root() / 'tools/meson/build_java_jar.py',
    '--output', '@OUTPUT@',
    '--main-class', 'com.android.cts.xmlgenerator.CtsXmlGenerator',
    '--source-root', meson.current_source_dir() / 'src',
    '--dependency-jar', meson.project_source_root() / '../prebuilts/devtools/tools/lib/guava-13.0.1.jar',
    '--dependency-jar', '@INPUT0@',
    '--dependency-jar', '@INPUT1@',
  ],
  build_by_default: true,
  install: false,
)
