#!/usr/bin/env python3
"""Compile Java source roots into a self-contained executable JAR."""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import tempfile
from pathlib import Path


def find_java_tool(tool_name: str) -> str | None:
    # 1. Environment variable JAVA_HOME
    java_home = Path(os.environ['JAVA_HOME']) if os.environ.get('JAVA_HOME') else None
    if java_home and (java_home / f'bin/{tool_name}').is_file():
        return str(java_home / f'bin/{tool_name}')

    # 2. PATH
    which_path = shutil.which(tool_name)
    if which_path:
        return which_path

    # 3. Known system and IDE JDK installations
    candidates = [
        Path('/usr/lib/jvm/default/bin') / tool_name,
    ]
    for candidate in candidates:
        if candidate.is_file():
            return str(candidate)

    return None


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--main-class', required=True)
    parser.add_argument('--source-root', type=Path, action='append', required=True)
    parser.add_argument('--dependency-jar', type=Path, action='append', default=[])
    args = parser.parse_args()

    javac = find_java_tool('javac')
    jar = find_java_tool('jar')
    if not javac or not jar:
        raise SystemExit('javac and jar must be available in PATH or JAVA_HOME')

    sources = [str(path) for root in args.source_root for path in sorted(root.rglob('*.java'))]
    if not sources:
        raise SystemExit('No Java sources found')

    args.output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='meson-java-') as temporary:
        classes = Path(temporary) / 'classes'
        classes.mkdir()
        classpath = os.pathsep.join(str(path) for path in args.dependency_jar)
        compile_command = [javac, '--release', '21', '-d', str(classes)]
        if classpath:
            compile_command.extend(['-cp', classpath])
        compile_command.extend(sources)
        subprocess.run(compile_command, check=True)

        for dependency in args.dependency_jar:
            subprocess.run([jar, 'xf', str(dependency)], cwd=classes, check=True)
        subprocess.run([
            jar, '--create', '--file', str(args.output), '--main-class', args.main_class,
            '-C', str(classes), '.',
        ], check=True)
    print(f'JAR: {args.output}')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
