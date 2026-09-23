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
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.io.PipedInputStream;
import java.io.PipedOutputStream;
import java.util.List;

import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerException;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.stream.StreamResult;
import javax.xml.transform.stream.StreamSource;

/**
 * Outputs an HTML report of the {@link ApiCoverage} collected by transforming the XML report.
 */
final class HtmlReport {

    public static void printHtmlReport(final List<File> testApks, final ApiCoverage apiCoverage,
            final String packageFilter, final String reportTitle, final OutputStream out)
                throws IOException, TransformerException {
        final var xmlOut = new PipedOutputStream();
        final var xmlIn = new PipedInputStream(xmlOut);

        Thread writerThread = new Thread(() -> {
            try (xmlOut) {
                XmlReport.printXmlReport(testApks, apiCoverage, packageFilter, reportTitle, xmlOut);
            } catch (IOException e) {
                System.err.println("Error streaming XML for HTML transformation: " + e.getMessage());
            }
        });
        writerThread.start();

        try (InputStream xsl = CtsApiCoverage.class.getResourceAsStream("/api-coverage.xsl")) {
            if (xsl == null) {
                throw new IOException("Resource /api-coverage.xsl not found in classpath");
            }
            var xslSource = new StreamSource(xsl);
            var factory = TransformerFactory.newInstance();
            Transformer transformer = factory.newTransformer(xslSource);

            var xmlSource = new StreamSource(xmlIn);
            var result = new StreamResult(out);
            transformer.transform(xmlSource, result);
        }
    }
}
