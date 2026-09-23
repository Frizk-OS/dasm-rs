/*
 * Copyright (C) 2010 The Android Open Source Project
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

package com.android.cts.apicoverage;

import java.io.OutputStream;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.util.List;

/**
 * Class that outputs a text report of {@link ApiCoverage}.
 */
final class TextReport {

    public static void printTextReport(ApiCoverage api, String packageFilter, OutputStream outputStream) {
        var out = new PrintStream(outputStream, true, StandardCharsets.UTF_8);
        var comparator = new CoverageComparator();

        List<ApiPackage> packages = api.getPackages().stream()
                .sorted(comparator)
                .toList();

        for (ApiPackage apiPackage : packages) {
            if (apiPackage.getName().startsWith(packageFilter) && apiPackage.getTotalMethods() > 0) {
                printPackage(apiPackage, out);
            }
        }

        out.println();
        out.println();

        for (ApiPackage apiPackage : packages) {
            if (apiPackage.getName().startsWith(packageFilter)) {
                printPackage(apiPackage, out);

                List<ApiClass> classes = apiPackage.getClasses().stream()
                        .sorted(comparator)
                        .toList();

                for (ApiClass apiClass : classes) {
                    if (apiClass.getTotalMethods() > 0) {
                        printClass(apiClass, out);

                        List<ApiConstructor> constructors = apiClass.getConstructors().stream()
                                .sorted()
                                .toList();
                        for (ApiConstructor constructor : constructors) {
                            printConstructor(constructor, out);
                        }

                        List<ApiMethod> methods = apiClass.getMethods().stream()
                                .sorted()
                                .toList();
                        for (ApiMethod method : methods) {
                            printMethod(method, out);
                        }
                    }
                }
            }
        }
    }

    private static void printPackage(ApiPackage apiPackage, PrintStream out) {
        out.printf("%s %d%% (%d/%d)%n",
                apiPackage.getName(),
                Math.round(apiPackage.getCoveragePercentage()),
                apiPackage.getNumCoveredMethods(),
                apiPackage.getTotalMethods());
    }

    private static void printClass(ApiClass apiClass, PrintStream out) {
        out.printf("  %s %d%% (%d/%d)%n",
                apiClass.getName(),
                Math.round(apiClass.getCoveragePercentage()),
                apiClass.getNumCoveredMethods(),
                apiClass.getTotalMethods());
    }

    private static void printConstructor(ApiConstructor constructor, PrintStream out) {
        String params = String.join(", ", constructor.getParameterTypes());
        out.printf("    [%s] %s(%s)%n",
                constructor.isCovered() ? "X" : " ",
                constructor.getName(),
                params);
    }

    private static void printMethod(ApiMethod method, PrintStream out) {
        String params = String.join(", ", method.getParameterTypes());
        out.printf("    [%s] %s %s(%s)%n",
                method.isCovered() ? "X" : " ",
                method.getReturnType(),
                method.getName(),
                params);
    }
}
