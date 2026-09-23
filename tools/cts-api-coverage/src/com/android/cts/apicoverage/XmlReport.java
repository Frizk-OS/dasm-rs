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

import java.io.File;
import java.io.OutputStream;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.util.List;
import java.util.Locale;

/**
 * Class that outputs an XML report of the {@link ApiCoverage} collected.
 */
final class XmlReport {

    private static final DateTimeFormatter DATE_FORMAT =
            DateTimeFormatter.ofPattern("EEE, MMM d, yyyy h:mm a z", Locale.US);

    public static void printXmlReport(List<File> testApks, ApiCoverage apiCoverage,
            String packageFilter, String reportTitle, OutputStream outputStream) {
        var out = new PrintStream(outputStream, true, StandardCharsets.UTF_8);
        out.println("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        out.println("<?xml-stylesheet type=\"text/xsl\"  href=\"api-coverage.xsl\"?>");

        String date = DATE_FORMAT.format(ZonedDateTime.now());
        out.printf("<api-coverage generatedTime=\"%s\" title=\"%s\">%n", date, reportTitle);

        out.println("<debug>");
        out.println("<sources>");
        for (File testApk : testApks) {
            out.printf("<apk path=\"%s\" />%n", testApk.getPath());
        }
        out.println("</sources>");
        out.println("</debug>");

        out.println("<api>");

        var comparator = new CoverageComparator();
        List<ApiPackage> packages = apiCoverage.getPackages().stream()
                .sorted(comparator)
                .toList();

        int totalMethods = 0;
        int totalCoveredMethods = 0;

        for (ApiPackage pkg : packages) {
            if (pkg.getName().startsWith(packageFilter) && pkg.getTotalMethods() > 0) {
                int pkgTotal = pkg.getTotalMethods();
                totalMethods += pkgTotal;
                int pkgTotalCovered = pkg.getNumCoveredMethods();
                totalCoveredMethods += pkgTotalCovered;
                out.printf("<package name=\"%s\" numCovered=\"%d\" numTotal=\"%d\" coveragePercentage=\"%d\">%n",
                        pkg.getName(), pkgTotalCovered, pkgTotal, Math.round(pkg.getCoveragePercentage()));

                List<ApiClass> classes = pkg.getClasses().stream()
                        .sorted(comparator)
                        .toList();

                for (ApiClass apiClass : classes) {
                    if (apiClass.getTotalMethods() > 0) {
                        out.printf("<class name=\"%s\" numCovered=\"%d\" numTotal=\"%d\" deprecated=\"%b\" coveragePercentage=\"%d\">%n",
                                apiClass.getName(), apiClass.getNumCoveredMethods(), apiClass.getTotalMethods(),
                                apiClass.isDeprecated(), Math.round(apiClass.getCoveragePercentage()));

                        for (ApiConstructor constructor : apiClass.getConstructors()) {
                            out.printf("<constructor name=\"%s\" deprecated=\"%b\" covered=\"%b\">%n",
                                    constructor.getName(), constructor.isDeprecated(), constructor.isCovered());
                            if (constructor.isDeprecated()) {
                                if (constructor.isCovered()) {
                                    totalCoveredMethods -= 1;
                                }
                                totalMethods -= 1;
                            }
                            for (String parameterType : constructor.getParameterTypes()) {
                                out.printf("<parameter type=\"%s\" />%n", parameterType);
                            }
                            out.println("</constructor>");
                        }

                        for (ApiMethod method : apiClass.getMethods()) {
                            out.printf("<method name=\"%s\" returnType=\"%s\" deprecated=\"%b\" covered=\"%b\">%n",
                                    method.getName(), method.getReturnType(), method.isDeprecated(), method.isCovered());
                            if (method.isDeprecated()) {
                                if (method.isCovered()) {
                                    totalCoveredMethods -= 1;
                                }
                                totalMethods -= 1;
                            }
                            for (String parameterType : method.getParameterTypes()) {
                                out.printf("<parameter type=\"%s\" />%n", parameterType);
                            }
                            out.println("</method>");
                        }
                        out.println("</class>");
                    }
                }
                out.println("</package>");
            }
        }

        out.println("</api>");
        int overallPct = totalMethods == 0 ? 0 : Math.round((float) totalCoveredMethods / totalMethods * 100.0f);
        out.printf("<total numCovered=\"%d\" numTotal=\"%d\" coveragePercentage=\"%d\" />%n",
                totalCoveredMethods, totalMethods, overallPct);
        out.println("</api-coverage>");
    }
}
