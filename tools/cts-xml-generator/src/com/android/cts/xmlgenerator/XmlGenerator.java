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

package com.android.cts.xmlgenerator;

import vogar.Expectation;
import vogar.ExpectationStore;

import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Collection;
import java.util.List;

/**
 * Generator of TestPackage XML files for native tests.
 */
final class XmlGenerator {

    private final String mAppNamespace;
    private final String mAppPackageName;
    private final String mName;
    private final String mRunner;
    private final String mTargetBinaryName;
    private final String mTargetNameSpace;
    private final String mJarPath;
    private final String mTestType;
    private final String mOutputPath;
    private final ExpectationStore mExpectations;

    XmlGenerator(ExpectationStore expectations, String appNameSpace, String appPackageName,
            String name, String runner, String targetBinaryName, String targetNameSpace,
            String jarPath, String testType, String outputPath) {
        mAppNamespace = appNameSpace;
        mAppPackageName = appPackageName;
        mName = name;
        mRunner = runner;
        mTargetBinaryName = targetBinaryName;
        mTargetNameSpace = targetNameSpace;
        mJarPath = jarPath;
        mTestType = testType;
        mOutputPath = outputPath;
        mExpectations = expectations;
    }

    public void writePackageXml() throws IOException {
        if (mOutputPath != null) {
            Path outPath = Path.of(mOutputPath);
            if (outPath.getParent() != null) {
                Files.createDirectories(outPath.getParent());
            }
            try (var writer = new PrintWriter(Files.newBufferedWriter(outPath, StandardCharsets.UTF_8))) {
                writer.println("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
                writeTestPackage(writer);
            }
        } else {
            var writer = new PrintWriter(System.out, true, StandardCharsets.UTF_8);
            writer.println("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
            writeTestPackage(writer);
            writer.flush();
        }
    }

    private void writeTestPackage(PrintWriter writer) {
        writer.append("<TestPackage");
        if (mAppNamespace != null) {
            writer.append(" appNameSpace=\"").append(mAppNamespace).append("\"");
        }

        writer.append(" appPackageName=\"").append(mAppPackageName).append("\"");
        writer.append(" name=\"").append(mName).append("\"");

        if (mRunner != null) {
            writer.append(" runner=\"").append(mRunner).append("\"");
        }

        if (mAppNamespace != null && mTargetNameSpace != null
                && !mAppNamespace.equals(mTargetNameSpace)) {
            writer.append(" targetBinaryName=\"").append(mTargetBinaryName).append("\"");
            writer.append(" targetNameSpace=\"").append(mTargetNameSpace).append("\"");
        }

        if (mTestType != null && !mTestType.isEmpty()) {
            writer.append(" testType=\"").append(mTestType).append("\"");
        }

        if (mJarPath != null) {
            writer.append(" jarPath=\"").append(mJarPath).append("\"");
        }

        writer.println(" version=\"1.0\">");

        var parser = new TestListParser();
        Collection<TestSuite> suites = parser.parse(System.in);
        var nameCollector = new StringBuilder();
        writeTestSuites(writer, suites, nameCollector);
        writer.println("</TestPackage>");
    }

    private void writeTestSuites(PrintWriter writer, Collection<TestSuite> suites,
            StringBuilder nameCollector) {
        List<TestSuite> sorted = suites.stream().sorted().toList();
        for (TestSuite suite : sorted) {
            writer.append("<TestSuite name=\"").append(suite.getName()).println("\">");

            String namePart = suite.getName();
            if (nameCollector.length() > 0) {
                namePart = "." + namePart;
            }
            nameCollector.append(namePart);

            writeTestSuites(writer, suite.getSuites(), nameCollector);
            writeTestCases(writer, suite.getCases(), nameCollector);

            nameCollector.delete(nameCollector.length() - namePart.length(),
                    nameCollector.length());
            writer.println("</TestSuite>");
        }
    }

    private void writeTestCases(PrintWriter writer, Collection<TestCase> cases,
            StringBuilder nameCollector) {
        List<TestCase> sorted = cases.stream().sorted().toList();
        for (TestCase testCase : sorted) {
            String name = testCase.getName();
            writer.append("<TestCase name=\"").append(name).println("\">");
            nameCollector.append('.').append(name);

            writeTests(writer, testCase.getTests(), nameCollector);

            nameCollector.delete(nameCollector.length() - name.length() - 1,
                    nameCollector.length());
            writer.println("</TestCase>");
        }
    }

    private void writeTests(PrintWriter writer, Collection<Test> tests,
            StringBuilder nameCollector) {
        List<Test> sorted = tests.stream().sorted().toList();
        for (Test test : sorted) {
            nameCollector.append('#').append(test.getName());
            writer.append("<Test name=\"").append(test.getName()).append("\"");
            if (isKnownFailure(mExpectations, nameCollector.toString())) {
                writer.append(" expectation=\"failure\"");
            }
            if (test.getTimeout() >= 0) {
                writer.append(" timeout=\"").append(String.valueOf(test.getTimeout())).append("\"");
            }
            writer.println(" />");

            nameCollector.delete(nameCollector.length() - test.getName().length() - 1,
                    nameCollector.length());
        }
    }

    public static boolean isKnownFailure(ExpectationStore expectationStore, String testName) {
        return expectationStore != null && expectationStore.get(testName) != Expectation.SUCCESS;
    }
}
