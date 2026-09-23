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

import java.util.Collections;
import java.util.List;
import java.util.Objects;

/** Representation of a constructor in the API with parameters (arguments). */
final class ApiConstructor implements Comparable<ApiConstructor> {

    private final String mName;
    private final List<String> mParameterTypes;
    private final boolean mDeprecated;
    private boolean mIsCovered;

    ApiConstructor(String name, List<String> parameterTypes, boolean deprecated) {
        mName = Objects.requireNonNull(name);
        mParameterTypes = List.copyOf(parameterTypes);
        mDeprecated = deprecated;
    }

    @Override
    public int compareTo(ApiConstructor another) {
        return Integer.compare(mParameterTypes.size(), another.mParameterTypes.size());
    }

    public String getName() {
        return mName;
    }

    public List<String> getParameterTypes() {
        return Collections.unmodifiableList(mParameterTypes);
    }

    public boolean isDeprecated() {
        return mDeprecated;
    }

    public boolean isCovered() {
        return mIsCovered;
    }

    public void setCovered(boolean covered) {
        mIsCovered = covered;
    }
}
