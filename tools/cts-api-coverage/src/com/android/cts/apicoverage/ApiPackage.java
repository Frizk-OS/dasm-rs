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

import java.util.Collection;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;

/** Representation of a package in the API containing classes. */
final class ApiPackage implements HasCoverage {

    private final String mName;
    private final Map<String, ApiClass> mApiClassMap = new HashMap<>();

    ApiPackage(String name) {
        mName = Objects.requireNonNull(name);
    }

    @Override
    public String getName() {
        return mName;
    }

    public void addClass(ApiClass apiClass) {
        mApiClassMap.put(apiClass.getName(), apiClass);
    }

    public ApiClass getClass(String name) {
        return mApiClassMap.get(name);
    }

    public Collection<ApiClass> getClasses() {
        return Collections.unmodifiableCollection(mApiClassMap.values());
    }

    public int getNumCoveredMethods() {
        return mApiClassMap.values().stream()
                .mapToInt(ApiClass::getNumCoveredMethods)
                .sum();
    }

    public int getTotalMethods() {
        return mApiClassMap.values().stream()
                .mapToInt(ApiClass::getTotalMethods)
                .sum();
    }

    @Override
    public float getCoveragePercentage() {
        int total = getTotalMethods();
        return total == 0 ? 100.0f : ((float) getNumCoveredMethods() / total) * 100.0f;
    }

    public void removeEmptyAbstractClasses() {
        mApiClassMap.values().removeIf(cls -> cls.isAbstract() && (cls.getTotalMethods() == 0));
    }
}
