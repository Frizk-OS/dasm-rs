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

import org.xml.sax.Attributes;
import org.xml.sax.SAXException;
import org.xml.sax.helpers.DefaultHandler;

import java.util.ArrayList;
import java.util.List;

/**
 * {@link DefaultHandler} that builds an empty {@link ApiCoverage} object from scanning current.xml.
 */
final class CurrentXmlHandler extends DefaultHandler {

    private String mCurrentPackageName;
    private String mCurrentClassName;
    private boolean mIgnoreCurrentClass;
    private String mCurrentMethodName;
    private String mCurrentMethodReturnType;
    private boolean mCurrentMethodIsAbstract;
    private boolean mDeprecated;

    private final List<String> mCurrentParameterTypes = new ArrayList<>();
    private final ApiCoverage mApiCoverage = new ApiCoverage();

    public ApiCoverage getApi() {
        return mApiCoverage;
    }

    @Override
    public void startElement(String uri, String localName, String name, Attributes attributes)
            throws SAXException {
        super.startElement(uri, localName, name, attributes);
        if ("package".equalsIgnoreCase(localName)) {
            mCurrentPackageName = getValue(attributes, "name");
            var apiPackage = new ApiPackage(mCurrentPackageName);
            mApiCoverage.addPackage(apiPackage);
        } else if ("class".equalsIgnoreCase(localName)) {
            if (isEnum(attributes)) {
                mIgnoreCurrentClass = true;
                return;
            }
            mIgnoreCurrentClass = false;
            mCurrentClassName = getValue(attributes, "name");
            mDeprecated = isDeprecated(attributes);
            var apiClass = new ApiClass(mCurrentClassName, mDeprecated, isAbstract(attributes));
            ApiPackage apiPackage = mApiCoverage.getPackage(mCurrentPackageName);
            if (apiPackage != null) {
                apiPackage.addClass(apiClass);
            }
        } else if ("interface".equalsIgnoreCase(localName)) {
            mIgnoreCurrentClass = true;
        } else if ("constructor".equalsIgnoreCase(localName)) {
            mDeprecated = isDeprecated(attributes);
            mCurrentParameterTypes.clear();
        } else if ("method".equalsIgnoreCase(localName)) {
            mDeprecated = isDeprecated(attributes);
            mCurrentMethodName = getValue(attributes, "name");
            mCurrentMethodReturnType = getValue(attributes, "return");
            mCurrentMethodIsAbstract = isAbstract(attributes);
            mCurrentParameterTypes.clear();
        } else if ("parameter".equalsIgnoreCase(localName)) {
            mCurrentParameterTypes.add(getValue(attributes, "type"));
        }
    }

    @Override
    public void endElement(String uri, String localName, String name) throws SAXException {
        super.endElement(uri, localName, name);
        if (mIgnoreCurrentClass) {
            return;
        }
        if ("constructor".equalsIgnoreCase(localName)) {
            if (mCurrentParameterTypes.isEmpty()) {
                return;
            }
            var apiConstructor = new ApiConstructor(mCurrentClassName,
                    mCurrentParameterTypes, mDeprecated);
            ApiPackage apiPackage = mApiCoverage.getPackage(mCurrentPackageName);
            if (apiPackage != null) {
                ApiClass apiClass = apiPackage.getClass(mCurrentClassName);
                if (apiClass != null) {
                    apiClass.addConstructor(apiConstructor);
                }
            }
        } else if ("method".equalsIgnoreCase(localName)) {
            if (mCurrentMethodIsAbstract) {
                return;
            }
            var apiMethod = new ApiMethod(mCurrentMethodName, mCurrentParameterTypes,
                    mCurrentMethodReturnType, mDeprecated);
            ApiPackage apiPackage = mApiCoverage.getPackage(mCurrentPackageName);
            if (apiPackage != null) {
                ApiClass apiClass = apiPackage.getClass(mCurrentClassName);
                if (apiClass != null) {
                    apiClass.addMethod(apiMethod);
                }
            }
        }
    }

    private static String getValue(Attributes attributes, String name) {
        int index = attributes.getIndex(name);
        return index != -1 ? attributes.getValue(index) : "";
    }

    private static boolean isDeprecated(Attributes attributes) {
        return "deprecated".equals(getValue(attributes, "deprecated"));
    }

    private static boolean isAbstract(Attributes attributes) {
        return Boolean.parseBoolean(getValue(attributes, "abstract"));
    }

    private static boolean isEnum(Attributes attributes) {
        return "java.lang.Enum".equals(getValue(attributes, "extends"));
    }
}
