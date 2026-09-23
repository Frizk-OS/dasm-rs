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

import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.Collection;
import java.util.HashMap;
import java.util.Map;
import java.util.Scanner;

/**
 * Parser of test lists that are in the format of:
 *
 * suite:android.holo.cts
 * case:HoloTest
 * test:testHolo
 * test:testHoloDialog[:timeout_value]
 */
final class TestListParser {

    public Collection<TestSuite> parse(InputStream input) {
        Map<String, TestSuite> suiteMap = new HashMap<>();
        TestSuite currentSuite = null;
        TestCase currentCase = null;

        try (var scanner = new Scanner(input, StandardCharsets.UTF_8)) {
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine().strip();
                if (line.isEmpty()) {
                    continue;
                }
                String[] tokens = line.split(":");
                if (tokens.length < 2) {
                    continue;
                }

                String key = tokens[0];
                String value = tokens[1];
                switch (key) {
                    case "suite" -> currentSuite = handleSuite(suiteMap, value);
                    case "case" -> currentCase = handleCase(currentSuite, value);
                    case "test" -> {
                        int timeout = -1;
                        if (tokens.length == 3) {
                            try {
                                timeout = Integer.parseInt(tokens[2]);
                            } catch (NumberFormatException ignored) {
                            }
                        }
                        handleTest(currentCase, value, timeout);
                    }
                    default -> {
                    }
                }
            }
        }
        return suiteMap.values();
    }

    private TestSuite handleSuite(Map<String, TestSuite> suiteMap, String fullSuite) {
        String[] suites = fullSuite.split("\\.");
        TestSuite lastSuite = null;

        for (String name : suites) {
            if (lastSuite != null) {
                if (lastSuite.hasSuite(name)) {
                    lastSuite = lastSuite.getSuite(name);
                } else {
                    var newSuite = new TestSuite(name);
                    lastSuite.addSuite(newSuite);
                    lastSuite = newSuite;
                }
            } else {
                lastSuite = suiteMap.computeIfAbsent(name, TestSuite::new);
            }
        }

        return lastSuite;
    }

    private TestCase handleCase(TestSuite suite, String caseName) {
        if (suite == null) {
            throw new IllegalStateException("Test case '" + caseName + "' declared without an active test suite");
        }
        var testCase = new TestCase(caseName);
        suite.addCase(testCase);
        return testCase;
    }

    private void handleTest(TestCase testCase, String test, int timeout) {
        if (testCase != null) {
            testCase.addTest(test, timeout);
        }
    }
}
