/*
 * Copyright (C) 2011 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package com.android.cts.javascanner;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Stream;

final class DocletRunner {

    private final File mSourceDir;
    private final File mDocletPath;

    DocletRunner(File sourceDir, File docletPath) {
        mSourceDir = sourceDir;
        mDocletPath = docletPath;
    }

    int runJavaDoc() throws IOException, InterruptedException {
        var args = new ArrayList<String>();
        args.add("javadoc");
        args.add("-doclet");
        args.add("com.android.cts.javascannerdoclet.CtsJavaScannerDoclet");
        args.add("-docletpath");
        args.add(mDocletPath.toString());
        args.add("-sourcepath");
        args.add(getSourcePath(mSourceDir));
        args.add("-classpath");
        args.add(getClassPath());
        args.addAll(getSourceFiles(mSourceDir));

        var process = new ProcessBuilder(args).start();
        try (var scanner = new Scanner(process.getInputStream())) {
            while (scanner.hasNextLine()) {
                System.out.println(scanner.nextLine());
            }
        }

        return process.waitFor();
    }

    private String getSourcePath(File sourceDir) {
        var basePaths = List.of(
            "./frameworks/base/core/java",
            "./frameworks/base/test-runner/src",
            "./external/junit/src",
            "./development/tools/hosttestlib/src",
            "./libcore/dalvik/src/main/java",
            "./cts/tests/src",
            "./cts/libs/commonutil/src",
            "./cts/libs/deviceutil/src",
            "./frameworks/testing/uiautomator/library/testrunner-src",
            "./frameworks/testing/uiautomator_test_libraries/src",
            sourceDir.toString()
        );
        return String.join(":", basePaths);
    }

    private String getClassPath() {
        return "./prebuilts/misc/common/tradefed/tradefed-prebuilt.jar";
    }

    private List<String> getSourceFiles(File sourceDir) throws IOException {
        if (!sourceDir.exists()) {
            return List.of();
        }
        try (Stream<Path> walk = Files.walk(sourceDir.toPath())) {
            return walk
                .filter(Files::isRegularFile)
                .map(Path::toString)
                .filter(s -> s.endsWith(".java"))
                .toList();
        }
    }
}
