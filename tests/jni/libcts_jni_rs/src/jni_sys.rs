/*
 * Copyright (C) 2009 The Android Open Source Project
 * Copyright (C) 2026 The AOSP and FrizkOS.
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

//! Modern JNI 1.4 / 1.6 FFI bindings and type-safe wrappers in idiomatic Rust 2024.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_variables, dead_code)]

use std::ffi::{c_char, c_void, CStr};
use std::fmt;
use std::ops::Deref;
use std::ptr;

//
// Primitive types
//

pub type jint = i32;
pub type jlong = i64;
pub type jbyte = i8;
pub type jboolean = u8;
pub type jchar = u16;
pub type jshort = i16;
pub type jfloat = f32;
pub type jdouble = f64;
pub type jsize = jint;

//
// Reference types
//

pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jobjectArray = jobject;
pub type jbooleanArray = jobject;
pub type jbyteArray = jobject;
pub type jcharArray = jobject;
pub type jshortArray = jobject;
pub type jintArray = jobject;
pub type jlongArray = jobject;
pub type jfloatArray = jobject;
pub type jdoubleArray = jobject;
pub type jthrowable = jobject;
pub type jweak = jobject;

pub type jfieldID = *mut c_void;
pub type jmethodID = *mut c_void;

//
// Constants
//

pub const JNI_FALSE: jboolean = 0;
pub const JNI_TRUE: jboolean = 1;

pub const JNI_OK: jint = 0;
pub const JNI_ERR: jint = -1;
pub const JNI_EDETACHED: jint = -2;
pub const JNI_EVERSION: jint = -3;

pub const JNI_COMMIT: jint = 1;
pub const JNI_ABORT: jint = 2;

pub const JNI_VERSION_1_1: jint = 0x00010001;
pub const JNI_VERSION_1_2: jint = 0x00010002;
pub const JNI_VERSION_1_4: jint = 0x00010004;
pub const JNI_VERSION_1_6: jint = 0x00010006;

#[repr(C)]
#[derive(Clone, Copy)]
pub union jvalue {
    pub z: jboolean,
    pub b: jbyte,
    pub c: jchar,
    pub s: jshort,
    pub i: jint,
    pub j: jlong,
    pub f: jfloat,
    pub d: jdouble,
    pub l: jobject,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JNINativeMethod {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub fn_ptr: *mut c_void,
}

unsafe impl Send for JNINativeMethod {}
unsafe impl Sync for JNINativeMethod {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JNINativeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub reserved3: *mut c_void,

    pub GetVersion: unsafe extern "C" fn(*mut JNIEnv) -> jint,
    pub DefineClass: unsafe extern "C" fn(*mut JNIEnv, *const c_char, jobject, *const jbyte, jsize) -> jclass,
    pub FindClass: unsafe extern "C" fn(*mut JNIEnv, *const c_char) -> jclass,
    pub FromReflectedMethod: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jmethodID,
    pub FromReflectedField: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jfieldID,
    pub ToReflectedMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, jboolean) -> jobject,
    pub GetSuperclass: unsafe extern "C" fn(*mut JNIEnv, jclass) -> jclass,
    pub IsAssignableFrom: unsafe extern "C" fn(*mut JNIEnv, jclass, jclass) -> jboolean,
    pub ToReflectedField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jboolean) -> jobject,

    pub Throw: unsafe extern "C" fn(*mut JNIEnv, jthrowable) -> jint,
    pub ThrowNew: unsafe extern "C" fn(*mut JNIEnv, jclass, *const c_char) -> jint,
    pub ExceptionOccurred: unsafe extern "C" fn(*mut JNIEnv) -> jthrowable,
    pub ExceptionDescribe: unsafe extern "C" fn(*mut JNIEnv),
    pub ExceptionClear: unsafe extern "C" fn(*mut JNIEnv),
    pub FatalError: unsafe extern "C" fn(*mut JNIEnv, *const c_char),

    pub PushLocalFrame: unsafe extern "C" fn(*mut JNIEnv, jint) -> jint,
    pub PopLocalFrame: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jobject,

    pub NewGlobalRef: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jobject,
    pub DeleteGlobalRef: unsafe extern "C" fn(*mut JNIEnv, jobject),
    pub DeleteLocalRef: unsafe extern "C" fn(*mut JNIEnv, jobject),
    pub IsSameObject: unsafe extern "C" fn(*mut JNIEnv, jobject, jobject) -> jboolean,
    pub NewLocalRef: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jobject,
    pub EnsureLocalCapacity: unsafe extern "C" fn(*mut JNIEnv, jint) -> jint,

    pub AllocObject: unsafe extern "C" fn(*mut JNIEnv, jclass) -> jobject,
    pub NewObject: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jobject,
    pub NewObjectV: *mut c_void,
    pub NewObjectA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jobject,

    pub GetObjectClass: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jclass,
    pub IsInstanceOf: unsafe extern "C" fn(*mut JNIEnv, jobject, jclass) -> jboolean,
    pub GetMethodID: unsafe extern "C" fn(*mut JNIEnv, jclass, *const c_char, *const c_char) -> jmethodID,

    pub CallObjectMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jobject,
    pub CallObjectMethodV: *mut c_void,
    pub CallObjectMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jobject,
    pub CallBooleanMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jboolean,
    pub CallBooleanMethodV: *mut c_void,
    pub CallBooleanMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jboolean,
    pub CallByteMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jbyte,
    pub CallByteMethodV: *mut c_void,
    pub CallByteMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jbyte,
    pub CallCharMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jchar,
    pub CallCharMethodV: *mut c_void,
    pub CallCharMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jchar,
    pub CallShortMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jshort,
    pub CallShortMethodV: *mut c_void,
    pub CallShortMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jshort,
    pub CallIntMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jint,
    pub CallIntMethodV: *mut c_void,
    pub CallIntMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jint,
    pub CallLongMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jlong,
    pub CallLongMethodV: *mut c_void,
    pub CallLongMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jlong,
    pub CallFloatMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jfloat,
    pub CallFloatMethodV: *mut c_void,
    pub CallFloatMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jfloat,
    pub CallDoubleMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jdouble,
    pub CallDoubleMethodV: *mut c_void,
    pub CallDoubleMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jdouble,
    pub CallVoidMethod: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...),
    pub CallVoidMethodV: *mut c_void,
    pub CallVoidMethodA: unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue),

    pub CallNonvirtualObjectMethod: *mut c_void,
    pub CallNonvirtualObjectMethodV: *mut c_void,
    pub CallNonvirtualObjectMethodA: *mut c_void,
    pub CallNonvirtualBooleanMethod: *mut c_void,
    pub CallNonvirtualBooleanMethodV: *mut c_void,
    pub CallNonvirtualBooleanMethodA: *mut c_void,
    pub CallNonvirtualByteMethod: *mut c_void,
    pub CallNonvirtualByteMethodV: *mut c_void,
    pub CallNonvirtualByteMethodA: *mut c_void,
    pub CallNonvirtualCharMethod: *mut c_void,
    pub CallNonvirtualCharMethodV: *mut c_void,
    pub CallNonvirtualCharMethodA: *mut c_void,
    pub CallNonvirtualShortMethod: *mut c_void,
    pub CallNonvirtualShortMethodV: *mut c_void,
    pub CallNonvirtualShortMethodA: *mut c_void,
    pub CallNonvirtualIntMethod: *mut c_void,
    pub CallNonvirtualIntMethodV: *mut c_void,
    pub CallNonvirtualIntMethodA: *mut c_void,
    pub CallNonvirtualLongMethod: *mut c_void,
    pub CallNonvirtualLongMethodV: *mut c_void,
    pub CallNonvirtualLongMethodA: *mut c_void,
    pub CallNonvirtualFloatMethod: *mut c_void,
    pub CallNonvirtualFloatMethodV: *mut c_void,
    pub CallNonvirtualFloatMethodA: *mut c_void,
    pub CallNonvirtualDoubleMethod: *mut c_void,
    pub CallNonvirtualDoubleMethodV: *mut c_void,
    pub CallNonvirtualDoubleMethodA: *mut c_void,
    pub CallNonvirtualVoidMethod: *mut c_void,
    pub CallNonvirtualVoidMethodV: *mut c_void,
    pub CallNonvirtualVoidMethodA: *mut c_void,

    pub GetFieldID: unsafe extern "C" fn(*mut JNIEnv, jclass, *const c_char, *const c_char) -> jfieldID,
    pub GetObjectField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jobject,
    pub GetBooleanField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jboolean,
    pub GetByteField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jbyte,
    pub GetCharField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jchar,
    pub GetShortField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jshort,
    pub GetIntField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jint,
    pub GetLongField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jlong,
    pub GetFloatField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jfloat,
    pub GetDoubleField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jdouble,
    pub SetObjectField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jobject),
    pub SetBooleanField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jboolean),
    pub SetByteField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jbyte),
    pub SetCharField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jchar),
    pub SetShortField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jshort),
    pub SetIntField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jint),
    pub SetLongField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jlong),
    pub SetFloatField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jfloat),
    pub SetDoubleField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jdouble),

    pub GetStaticMethodID: unsafe extern "C" fn(*mut JNIEnv, jclass, *const c_char, *const c_char) -> jmethodID,
    pub CallStaticObjectMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jobject,
    pub CallStaticObjectMethodV: *mut c_void,
    pub CallStaticObjectMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jobject,
    pub CallStaticBooleanMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jboolean,
    pub CallStaticBooleanMethodV: *mut c_void,
    pub CallStaticBooleanMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jboolean,
    pub CallStaticByteMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jbyte,
    pub CallStaticByteMethodV: *mut c_void,
    pub CallStaticByteMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jbyte,
    pub CallStaticCharMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jchar,
    pub CallStaticCharMethodV: *mut c_void,
    pub CallStaticCharMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jchar,
    pub CallStaticShortMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jshort,
    pub CallStaticShortMethodV: *mut c_void,
    pub CallStaticShortMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jshort,
    pub CallStaticIntMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jint,
    pub CallStaticIntMethodV: *mut c_void,
    pub CallStaticIntMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jint,
    pub CallStaticLongMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jlong,
    pub CallStaticLongMethodV: *mut c_void,
    pub CallStaticLongMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jlong,
    pub CallStaticFloatMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jfloat,
    pub CallStaticFloatMethodV: *mut c_void,
    pub CallStaticFloatMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jfloat,
    pub CallStaticDoubleMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jdouble,
    pub CallStaticDoubleMethodV: *mut c_void,
    pub CallStaticDoubleMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jdouble,
    pub CallStaticVoidMethod: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...),
    pub CallStaticVoidMethodV: *mut c_void,
    pub CallStaticVoidMethodA: unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue),

    pub GetStaticFieldID: unsafe extern "C" fn(*mut JNIEnv, jclass, *const c_char, *const c_char) -> jfieldID,
    pub GetStaticObjectField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jobject,
    pub GetStaticBooleanField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jboolean,
    pub GetStaticByteField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jbyte,
    pub GetStaticCharField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jchar,
    pub GetStaticShortField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jshort,
    pub GetStaticIntField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jint,
    pub GetStaticLongField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jlong,
    pub GetStaticFloatField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jfloat,
    pub GetStaticDoubleField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jdouble,
    pub SetStaticObjectField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jobject),
    pub SetStaticBooleanField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jboolean),
    pub SetStaticByteField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jbyte),
    pub SetStaticCharField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jchar),
    pub SetShortField2: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jshort),
    pub SetStaticIntField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jint),
    pub SetStaticLongField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jlong),
    pub SetStaticFloatField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jfloat),
    pub SetStaticDoubleField: unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jdouble),

    pub NewString: unsafe extern "C" fn(*mut JNIEnv, *const jchar, jsize) -> jstring,
    pub GetStringLength: unsafe extern "C" fn(*mut JNIEnv, jstring) -> jsize,
    pub GetStringChars: unsafe extern "C" fn(*mut JNIEnv, jstring, *mut jboolean) -> *const jchar,
    pub ReleaseStringChars: unsafe extern "C" fn(*mut JNIEnv, jstring, *const jchar),
    pub NewStringUTF: unsafe extern "C" fn(*mut JNIEnv, *const c_char) -> jstring,
    pub GetStringUTFLength: unsafe extern "C" fn(*mut JNIEnv, jstring) -> jsize,
    pub GetStringUTFChars: unsafe extern "C" fn(*mut JNIEnv, jstring, *mut jboolean) -> *const c_char,
    pub ReleaseStringUTFChars: unsafe extern "C" fn(*mut JNIEnv, jstring, *const c_char),

    pub GetArrayLength: unsafe extern "C" fn(*mut JNIEnv, jarray) -> jsize,
    pub NewObjectArray: unsafe extern "C" fn(*mut JNIEnv, jsize, jclass, jobject) -> jobjectArray,
    pub GetObjectArrayElement: unsafe extern "C" fn(*mut JNIEnv, jobjectArray, jsize) -> jobject,
    pub SetObjectArrayElement: unsafe extern "C" fn(*mut JNIEnv, jobjectArray, jsize, jobject),

    pub NewBooleanArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jbooleanArray,
    pub NewByteArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jbyteArray,
    pub NewCharArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jcharArray,
    pub NewShortArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jshortArray,
    pub NewIntArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jintArray,
    pub NewLongArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jlongArray,
    pub NewFloatArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jfloatArray,
    pub NewDoubleArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jdoubleArray,

    pub GetBooleanArrayElements: unsafe extern "C" fn(*mut JNIEnv, jbooleanArray, *mut jboolean) -> *mut jboolean,
    pub GetByteArrayElements: unsafe extern "C" fn(*mut JNIEnv, jbyteArray, *mut jboolean) -> *mut jbyte,
    pub GetCharArrayElements: unsafe extern "C" fn(*mut JNIEnv, jcharArray, *mut jboolean) -> *mut jchar,
    pub GetShortArrayElements: unsafe extern "C" fn(*mut JNIEnv, jshortArray, *mut jboolean) -> *mut jshort,
    pub GetIntArrayElements: unsafe extern "C" fn(*mut JNIEnv, jintArray, *mut jboolean) -> *mut jint,
    pub GetLongArrayElements: unsafe extern "C" fn(*mut JNIEnv, jlongArray, *mut jboolean) -> *mut jlong,
    pub GetFloatArrayElements: unsafe extern "C" fn(*mut JNIEnv, jfloatArray, *mut jboolean) -> *mut jfloat,
    pub GetDoubleArrayElements: unsafe extern "C" fn(*mut JNIEnv, jdoubleArray, *mut jboolean) -> *mut jdouble,

    pub ReleaseBooleanArrayElements: unsafe extern "C" fn(*mut JNIEnv, jbooleanArray, *mut jboolean, jint),
    pub ReleaseByteArrayElements: unsafe extern "C" fn(*mut JNIEnv, jbyteArray, *mut jbyte, jint),
    pub ReleaseCharArrayElements: unsafe extern "C" fn(*mut JNIEnv, jcharArray, *mut jchar, jint),
    pub ReleaseShortArrayElements: unsafe extern "C" fn(*mut JNIEnv, jshortArray, *mut jshort, jint),
    pub ReleaseIntArrayElements: unsafe extern "C" fn(*mut JNIEnv, jintArray, *mut jint, jint),
    pub ReleaseLongArrayElements: unsafe extern "C" fn(*mut JNIEnv, jlongArray, *mut jlong, jint),
    pub ReleaseFloatArrayElements: unsafe extern "C" fn(*mut JNIEnv, jfloatArray, *mut jfloat, jint),
    pub ReleaseDoubleArrayElements: unsafe extern "C" fn(*mut JNIEnv, jdoubleArray, *mut jdouble, jint),

    pub GetBooleanArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jbooleanArray, jsize, jsize, *mut jboolean),
    pub GetByteArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jbyteArray, jsize, jsize, *mut jbyte),
    pub GetCharArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jcharArray, jsize, jsize, *mut jchar),
    pub GetShortArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jshortArray, jsize, jsize, *mut jshort),
    pub GetIntArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jintArray, jsize, jsize, *mut jint),
    pub GetLongArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jlongArray, jsize, jsize, *mut jlong),
    pub GetFloatArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jfloatArray, jsize, jsize, *mut jfloat),
    pub GetDoubleArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jdoubleArray, jsize, jsize, *mut jdouble),

    pub SetBooleanArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jbooleanArray, jsize, jsize, *const jboolean),
    pub SetByteArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jbyteArray, jsize, jsize, *const jbyte),
    pub SetCharArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jcharArray, jsize, jsize, *const jchar),
    pub SetShortArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jshortArray, jsize, jsize, *const jshort),
    pub SetIntArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jintArray, jsize, jsize, *const jint),
    pub SetLongArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jlongArray, jsize, jsize, *const jlong),
    pub SetFloatArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jfloatArray, jsize, jsize, *const jfloat),
    pub SetDoubleArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jdoubleArray, jsize, jsize, *const jdouble),

    pub RegisterNatives: unsafe extern "C" fn(*mut JNIEnv, jclass, *const JNINativeMethod, jint) -> jint,
    pub UnregisterNatives: unsafe extern "C" fn(*mut JNIEnv, jclass) -> jint,

    pub MonitorEnter: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jint,
    pub MonitorExit: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jint,

    pub GetJavaVM: unsafe extern "C" fn(*mut JNIEnv, *mut *mut JavaVM) -> jint,

    pub GetStringRegion: unsafe extern "C" fn(*mut JNIEnv, jstring, jsize, jsize, *mut jchar),
    pub GetStringUTFRegion: unsafe extern "C" fn(*mut JNIEnv, jstring, jsize, jsize, *mut c_char),

    pub GetPrimitiveArrayCritical: unsafe extern "C" fn(*mut JNIEnv, jarray, *mut jboolean) -> *mut c_void,
    pub ReleasePrimitiveArrayCritical: unsafe extern "C" fn(*mut JNIEnv, jarray, *mut c_void, jint),

    pub GetStringCritical: unsafe extern "C" fn(*mut JNIEnv, jstring, *mut jboolean) -> *const jchar,
    pub ReleaseStringCritical: unsafe extern "C" fn(*mut JNIEnv, jstring, *const jchar),

    pub NewWeakGlobalRef: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jweak,
    pub DeleteWeakGlobalRef: unsafe extern "C" fn(*mut JNIEnv, jweak),

    pub ExceptionCheck: unsafe extern "C" fn(*mut JNIEnv) -> jboolean,

    pub NewDirectByteBuffer: unsafe extern "C" fn(*mut JNIEnv, *mut c_void, jlong) -> jobject,
    pub GetDirectBufferAddress: unsafe extern "C" fn(*mut JNIEnv, jobject) -> *mut c_void,
    pub GetDirectBufferCapacity: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jlong,

    pub GetObjectRefType: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jint,
}

pub type JNIEnv = *const JNINativeInterface;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JNIInvokeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub DestroyJavaVM: unsafe extern "C" fn(*mut JavaVM) -> jint,
    pub AttachCurrentThread: unsafe extern "C" fn(*mut JavaVM, *mut *mut c_void, *mut c_void) -> jint,
    pub DetachCurrentThread: unsafe extern "C" fn(*mut JavaVM) -> jint,
    pub GetEnv: unsafe extern "C" fn(*mut JavaVM, *mut *mut c_void, jint) -> jint,
    pub AttachCurrentThreadAsDaemon: unsafe extern "C" fn(*mut JavaVM, *mut *mut c_void, *mut c_void) -> jint,
}

pub type JavaVM = *const JNIInvokeInterface;

//
// Safe Ergonomic Rust Wrapper around JNIEnv
//

/// Type-safe wrapper around raw `*mut JNIEnv` providing high-level safe abstractions.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct SafeJniEnv {
    env: *mut JNIEnv,
}

impl SafeJniEnv {
    #[inline]
    pub const unsafe fn from_raw(env: *mut JNIEnv) -> Self {
        Self { env }
    }

    #[inline]
    pub const fn as_raw(&self) -> *mut JNIEnv {
        self.env
    }

    #[inline]
    unsafe fn iface(&self) -> JNINativeInterface {
        unsafe { **self.env }
    }

    #[inline]
    pub fn find_class(&self, name: &CStr) -> Option<jclass> {
        unsafe {
            let res = (self.iface().FindClass)(self.env, name.as_ptr());
            if res.is_null() {
                None
            } else {
                Some(res)
            }
        }
    }

    #[inline]
    pub fn throw_new(&self, clazz: jclass, message: &CStr) -> bool {
        unsafe {
            (self.iface().ThrowNew)(self.env, clazz, message.as_ptr()) == JNI_OK
        }
    }

    #[inline]
    pub fn is_same_object(&self, a: jobject, b: jobject) -> bool {
        unsafe {
            (self.iface().IsSameObject)(self.env, a, b) != JNI_FALSE
        }
    }

    #[inline]
    pub fn new_string_utf(&self, s: &CStr) -> Option<jstring> {
        unsafe {
            let res = (self.iface().NewStringUTF)(self.env, s.as_ptr());
            if res.is_null() {
                None
            } else {
                Some(res)
            }
        }
    }

    #[inline]
    pub fn get_string_utf_chars<'a>(&self, s: jstring) -> Option<JniUtfStringGuard<'a>> {
        if s.is_null() {
            return None;
        }
        unsafe {
            let chars = (self.iface().GetStringUTFChars)(self.env, s, ptr::null_mut());
            if chars.is_null() {
                None
            } else {
                Some(JniUtfStringGuard {
                    env: self.env,
                    string: s,
                    chars,
                    _marker: std::marker::PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn register_natives(&self, clazz: jclass, methods: &[JNINativeMethod]) -> bool {
        unsafe {
            (self.iface().RegisterNatives)(self.env, clazz, methods.as_ptr(), methods.len() as jint) == JNI_OK
        }
    }
}

/// RAII Guard for UTF-8 string returned by GetStringUTFChars
pub struct JniUtfStringGuard<'a> {
    env: *mut JNIEnv,
    string: jstring,
    chars: *const c_char,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> JniUtfStringGuard<'a> {
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.chars) }
    }

    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }
}

impl<'a> Deref for JniUtfStringGuard<'a> {
    type Target = CStr;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_c_str()
    }
}

impl<'a> Drop for JniUtfStringGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            let iface = **self.env;
            (iface.ReleaseStringUTFChars)(self.env, self.string, self.chars);
        }
    }
}

impl<'a> fmt::Debug for JniUtfStringGuard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_c_str())
    }
}
