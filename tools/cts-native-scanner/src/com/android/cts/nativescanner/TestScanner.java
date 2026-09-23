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

package com.android.cts.nativescanner;

import java.io.BufferedReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Read from the BufferedReader a list of test case names and test cases.
 *
 * The expected format of the incoming test list:
 *   TEST_CASE_NAME.
 *     TEST_NAME1
 *     TEST_NAME2
 *
 * The output:
 *   suite:TestSuite
 *   case:TEST_CASE_NAME
 *   test:TEST_NAME1
 *   test:TEST_NAME2
 */
final class TestScanner {

    private final String mTestSuite;
    private final BufferedReader mReader;

    TestScanner(BufferedReader reader, String testSuite) {
        mTestSuite = Objects.requireNonNull(testSuite);
        mReader = Objects.requireNonNull(reader);
    }

    public List<String> getTestNames() throws IOException {
        var testNames = new ArrayList<String>();
        String testCaseName = null;
        String line;

        while ((line = mReader.readLine()) != null) {
            if (!line.isEmpty()) {
                if (line.startsWith(" ")) {
                    if (testCaseName != null) {
                        testNames.add("test:" + line.strip());
                    } else {
                        throw new IOException("TEST_CASE_NAME not defined before first test.");
                    }
                } else {
                    testCaseName = line.strip();
                    if (testCaseName.endsWith(".")) {
                        testCaseName = testCaseName.substring(0, testCaseName.length() - 1);
                    }
                    testNames.add("suite:" + mTestSuite);
                    testNames.add("case:" + testCaseName);
                }
            }
        }
        return List.copyOf(testNames);
    }
}
