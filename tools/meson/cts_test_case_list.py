#!/usr/bin/env python3
"""
Port of CtsTestCaseList.mk to modern Python / Meson tooling.
Defines all CTS packages, test cases, and host libraries.
"""

import json
import sys

CTS_SECURITY_APPS = [
    "CtsAppAccessData",
    "CtsAppWithData",
    "CtsExternalStorageApp",
    "CtsInstrumentationAppDiffCert",
    "CtsPermissionDeclareApp",
    "CtsPermissionDeclareAppCompat",
    "CtsReadExternalStorageApp",
    "CtsSharedUidInstall",
    "CtsSharedUidInstallDiffCert",
    "CtsSimpleAppInstall",
    "CtsSimpleAppInstallDiffCert",
    "CtsTargetInstrumentationApp",
    "CtsUsePermissionDiffCert",
    "CtsWriteExternalStorageApp",
    "CtsMultiUserStorageApp",
]

CTS_SUPPORT_PACKAGES = [
    "CtsAccelerationTestStubs",
    "CtsDeviceAdmin",
    "CtsDeviceOpenGl",
    "CtsDeviceTaskswitchingAppA",
    "CtsDeviceTaskswitchingAppB",
    "CtsDeviceTaskswitchingControl",
    "CtsDeviceUi",
    "CtsHoloDeviceApp",
    "CtsMonkeyApp",
    "CtsMonkeyApp2",
    "CtsSampleDeviceApp",
    "CtsSomeAccessibilityServices",
    "CtsTestStubs",
    "SignatureTest",
    "TestDeviceSetup",
    "CtsUiAutomatorApp",
    "CtsUsbSerialTestApp",
] + CTS_SECURITY_APPS

CTS_EXTERNAL_PACKAGES = [
    "com.replica.replicaisland",
]

CTS_TEST_CASE_LIST = CTS_SUPPORT_PACKAGES + CTS_EXTERNAL_PACKAGES

CTS_TEST_PACKAGES = [
    "CtsDeviceFilePerf",
    "CtsDeviceUi",
    "CtsDeviceDram",
    "CtsDeviceSimpleCpu",
    "CtsDeviceBrowserBench",
    "CtsDeviceVideoPerf",
    "CtsDeviceOpenGl",
    "CtsAccelerationTestCases",
    "CtsAccountManagerTestCases",
    "CtsAccessibilityServiceTestCases",
    "CtsAccessibilityTestCases",
    "CtsAdminTestCases",
    "CtsAnimationTestCases",
    "CtsAppTestCases",
    "CtsBluetoothTestCases",
    "CtsCalendarcommon2TestCases",
    "CtsContentTestCases",
    "CtsDatabaseTestCases",
    "CtsDisplayTestCases",
    "CtsDpiTestCases",
    "CtsDpiTestCases2",
    "CtsDreamsTestCases",
    "CtsDrmTestCases",
    "CtsEffectTestCases",
    "CtsGestureTestCases",
    "CtsGraphicsTestCases",
    "CtsGraphics2TestCases",
    "CtsHardwareTestCases",
    "CtsHoloTestCases",
    "CtsJniTestCases",
    "CtsKeystoreTestCases",
    "CtsLocationTestCases",
    "CtsMediaStressTestCases",
    "CtsMediaTestCases",
    "CtsNativeOpenGLTestCases",
    "CtsNdefTestCases",
    "CtsNetTestCases",
    "CtsOpenGLTestCases",
    "CtsOpenGlPerfTestCases",
    "CtsOsTestCases",
    "CtsPermissionTestCases",
    "CtsPermission2TestCases",
    "CtsPreferenceTestCases",
    "CtsPreference2TestCases",
    "CtsProviderTestCases",
    "CtsRenderscriptTestCases",
    "CtsRenderscriptGraphicsTestCases",
    "CtsRsCppTestCases",
    "CtsSampleDeviceTestCases",
    "CtsSaxTestCases",
    "CtsSecurityTestCases",
    "CtsSpeechTestCases",
    "CtsTelephonyTestCases",
    "CtsTextTestCases",
    "CtsTextureViewTestCases",
    "CtsThemeTestCases",
    "CtsUtilTestCases",
    "CtsViewTestCases",
    "CtsWebkitTestCases",
    "CtsWidgetTestCases",
]

CTS_COVERAGE_TEST_CASE_LIST = CTS_SUPPORT_PACKAGES + CTS_TEST_PACKAGES

CTS_HOST_LIBRARIES = [
    "CtsAdbTests",
    "CtsAppSecurityTests",
    "CtsHoloHostTestCases",
    "CtsHostJank",
    "CtsHostUi",
    "CtsMonkeyTestCases",
    "CtsSampleHostTestCases",
    "CtsUsbTests",
]

CTS_NATIVE_EXES = [
    "NativeMediaTest_SL",
    "NativeMediaTest_XA",
    "bionic-unit-tests-cts",
]

CTS_UI_TESTS = [
    "CtsUiAutomatorTests",
]

CTS_DEVICE_JARS = [
    "CtsDeviceJank",
]

DATA = {
    "security_apps": CTS_SECURITY_APPS,
    "support_packages": CTS_SUPPORT_PACKAGES,
    "external_packages": CTS_EXTERNAL_PACKAGES,
    "test_case_list": CTS_TEST_CASE_LIST,
    "test_packages": CTS_TEST_PACKAGES,
    "coverage_test_case_list": CTS_COVERAGE_TEST_CASE_LIST,
    "host_libraries": CTS_HOST_LIBRARIES,
    "native_exes": CTS_NATIVE_EXES,
    "ui_tests": CTS_UI_TESTS,
    "device_jars": CTS_DEVICE_JARS,
}

def main():
    if len(sys.argv) > 1:
        key = sys.argv[1]
        if key in DATA:
            for item in DATA[key]:
                print(item)
            return 0
        else:
            print(f"Unknown key: {key}", file=sys.stderr)
            return 1
    print(json.dumps(DATA, indent=2))
    return 0

if __name__ == "__main__":
    sys.exit(main())
