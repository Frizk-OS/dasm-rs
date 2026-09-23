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

import org.xml.sax.InputSource;
import org.xml.sax.SAXException;
import org.xml.sax.XMLReader;

import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

import javax.xml.parsers.ParserConfigurationException;
import javax.xml.parsers.SAXParserFactory;
import javax.xml.transform.TransformerException;

/**
 * Tool that generates a report of what Android framework methods are being called from a given
 * set of APKS. See the {@link #printUsage()} method for more details.
 */
public final class CtsApiCoverage {

    private static final int FORMAT_TXT = 0;
    private static final int FORMAT_XML = 1;
    private static final int FORMAT_HTML = 2;

    private static void printUsage() {
        System.out.println("""
            Usage: cts-api-coverage [OPTION]... [APK]...

            Generates a report about what Android framework methods are called\s
            from the given APKs.

            Use the Meson/Ninja target cts-test-coverage to generate the report\s
            rather than executing this directly.

            Options:
              -o FILE                output file or standard out if not given
              -f [txt|xml|html]      format of output
              -d PATH                path to dexdeps or expected to be in $PATH
              -a PATH                path to the API XML file
              -p PACKAGENAMEPREFIX   report coverage only for package that start with
              -t TITLE               report title
            """);
        System.exit(1);
    }

    public static void main(String[] args) throws Exception {
        if (args.length == 0) {
            printUsage();
        }

        var testApks = new ArrayList<File>();
        File outputFile = null;
        int format = FORMAT_TXT;
        String dexDeps = "dexDeps";
        String apiXmlPath = "";
        String packageFilter = "android";
        String reportTitle = "CTS API Coverage";

        for (int i = 0; i < args.length; i++) {
            if (args[i].startsWith("-")) {
                switch (args[i]) {
                    case "-o" -> outputFile = new File(getExpectedArg(args, ++i));
                    case "-f" -> {
                        String formatSpec = getExpectedArg(args, ++i);
                        format = switch (formatSpec.toLowerCase()) {
                            case "xml" -> FORMAT_XML;
                            case "txt" -> FORMAT_TXT;
                            case "html" -> FORMAT_HTML;
                            default -> {
                                printUsage();
                                yield FORMAT_TXT;
                            }
                        };
                    }
                    case "-d" -> dexDeps = getExpectedArg(args, ++i);
                    case "-a" -> apiXmlPath = getExpectedArg(args, ++i);
                    case "-p" -> packageFilter = getExpectedArg(args, ++i);
                    case "-t" -> reportTitle = getExpectedArg(args, ++i);
                    default -> printUsage();
                }
            } else {
                testApks.add(new File(args[i]));
            }
        }

        ApiCoverage apiCoverage = getEmptyApiCoverage(apiXmlPath);
        apiCoverage.removeEmptyAbstractClasses();
        for (File testApk : testApks) {
            addApiCoverage(apiCoverage, testApk, dexDeps);
        }
        outputCoverageReport(apiCoverage, testApks, outputFile, format, packageFilter, reportTitle);
    }

    private static String getExpectedArg(String[] args, int index) {
        if (index < args.length) {
            return args[index];
        } else {
            printUsage();
            return null;
        }
    }

    private static ApiCoverage getEmptyApiCoverage(String apiXmlPath)
            throws SAXException, IOException, ParserConfigurationException {
        SAXParserFactory factory = SAXParserFactory.newInstance();
        XMLReader xmlReader = factory.newSAXParser().getXMLReader();
        var currentXmlHandler = new CurrentXmlHandler();
        xmlReader.setContentHandler(currentXmlHandler);

        if (apiXmlPath != null && !apiXmlPath.isBlank()) {
            Path currentXml = Path.of(apiXmlPath);
            if (Files.isRegularFile(currentXml)) {
                try (var reader = Files.newBufferedReader(currentXml, StandardCharsets.UTF_8)) {
                    xmlReader.parse(new InputSource(reader));
                }
            }
        }

        return currentXmlHandler.getApi();
    }

    private static void addApiCoverage(ApiCoverage apiCoverage, File testApk, String dexdeps)
            throws SAXException, IOException, ParserConfigurationException {
        SAXParserFactory factory = SAXParserFactory.newInstance();
        XMLReader xmlReader = factory.newSAXParser().getXMLReader();
        var dexDepsXmlHandler = new DexDepsXmlHandler(apiCoverage);
        xmlReader.setContentHandler(dexDepsXmlHandler);

        Process process = new ProcessBuilder(dexdeps, "--format=xml", testApk.getPath()).start();
        xmlReader.parse(new InputSource(process.getInputStream()));
    }

    private static void outputCoverageReport(ApiCoverage apiCoverage, List<File> testApks,
            File outputFile, int format, String packageFilter, String reportTitle)
                throws IOException, TransformerException, InterruptedException {

        if (outputFile != null) {
            Path outPath = outputFile.toPath();
            if (outPath.getParent() != null) {
                Files.createDirectories(outPath.getParent());
            }
            try (OutputStream out = Files.newOutputStream(outPath)) {
                writeReport(apiCoverage, testApks, format, packageFilter, reportTitle, out);
            }
        } else {
            writeReport(apiCoverage, testApks, format, packageFilter, reportTitle, System.out);
        }
    }

    private static void writeReport(ApiCoverage apiCoverage, List<File> testApks,
            int format, String packageFilter, String reportTitle, OutputStream out)
                throws IOException, TransformerException {
        switch (format) {
            case FORMAT_TXT -> TextReport.printTextReport(apiCoverage, packageFilter, out);
            case FORMAT_XML -> XmlReport.printXmlReport(testApks, apiCoverage, packageFilter, reportTitle, out);
            case FORMAT_HTML -> HtmlReport.printHtmlReport(testApks, apiCoverage, packageFilter, reportTitle, out);
        }
    }
}
