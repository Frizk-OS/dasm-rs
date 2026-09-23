/*
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

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_variables, unused_imports)]

mod jni_sys;

use jni_sys::*;
use std::ffi::{c_void, CStr};
use std::ptr;

//
// InstanceNonce methods
//
unsafe extern "C" fn InstanceNonce_nop(_env: *mut JNIEnv, _this: jobject) {}

unsafe extern "C" fn InstanceNonce_returnBoolean(_env: *mut JNIEnv, _this: jobject) -> jboolean {
    JNI_FALSE
}

unsafe extern "C" fn InstanceNonce_returnByte(_env: *mut JNIEnv, _this: jobject) -> jbyte {
    123
}

unsafe extern "C" fn InstanceNonce_returnShort(_env: *mut JNIEnv, _this: jobject) -> jshort {
    -12345
}

unsafe extern "C" fn InstanceNonce_returnChar(_env: *mut JNIEnv, _this: jobject) -> jchar {
    34567
}

unsafe extern "C" fn InstanceNonce_returnInt(_env: *mut JNIEnv, _this: jobject) -> jint {
    12345678
}

unsafe extern "C" fn InstanceNonce_returnLong(_env: *mut JNIEnv, _this: jobject) -> jlong {
    -1098765432109876543i64
}

unsafe extern "C" fn InstanceNonce_returnFloat(_env: *mut JNIEnv, _this: jobject) -> jfloat {
    -98765.4321f32
}

unsafe extern "C" fn InstanceNonce_returnDouble(_env: *mut JNIEnv, _this: jobject) -> jdouble {
    12345678.9f64
}

unsafe extern "C" fn InstanceNonce_returnNull(_env: *mut JNIEnv, _this: jobject) -> jobject {
    ptr::null_mut()
}

unsafe extern "C" fn InstanceNonce_returnString(env: *mut JNIEnv, _this: jobject) -> jstring {
    unsafe {
        let interface = **env;
        let s = c"blort";
        (interface.NewStringUTF)(env, s.as_ptr())
    }
}

unsafe extern "C" fn InstanceNonce_returnShortArray(env: *mut JNIEnv, _this: jobject) -> jshortArray {
    unsafe {
        let interface = **env;
        let contents: [jshort; 3] = [10, 20, 30];
        let result = (interface.NewShortArray)(env, 3);
        if result.is_null() {
            return ptr::null_mut();
        }
        (interface.SetShortArrayRegion)(env, result, 0, 3, contents.as_ptr());
        result
    }
}

unsafe extern "C" fn InstanceNonce_returnStringArray(env: *mut JNIEnv, _this: jobject) -> jobjectArray {
    unsafe {
        let interface = **env;
        let str_class_name = c"java/lang/String";
        let string_class = (interface.FindClass)(env, str_class_name.as_ptr());
        if string_class.is_null() {
            return ptr::null_mut();
        }

        let result = (interface.NewObjectArray)(env, 100, string_class, ptr::null_mut());
        if result.is_null() {
            return ptr::null_mut();
        }

        let indices: [jsize; 3] = [0, 50, 99];
        let contents = [c"blort", c"zorch", c"fizmo"];

        for i in 0..3 {
            let s = (interface.NewStringUTF)(env, contents[i].as_ptr());
            if s.is_null() {
                return ptr::null_mut();
            }
            (interface.SetObjectArrayElement)(env, result, indices[i], s);
        }

        result
    }
}

unsafe extern "C" fn InstanceNonce_returnThis(_env: *mut JNIEnv, this: jobject) -> jobject {
    this
}

unsafe extern "C" fn InstanceNonce_takeBoolean(_env: *mut JNIEnv, _this: jobject, v: jboolean) -> jboolean {
    if v == JNI_FALSE { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeByte(_env: *mut JNIEnv, _this: jobject, v: jbyte) -> jboolean {
    if v == -99 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeShort(_env: *mut JNIEnv, _this: jobject, v: jshort) -> jboolean {
    if v == 19991 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeChar(_env: *mut JNIEnv, _this: jobject, v: jchar) -> jboolean {
    if v == 999 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeInt(_env: *mut JNIEnv, _this: jobject, v: jint) -> jboolean {
    if v == -999888777 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeLong(_env: *mut JNIEnv, _this: jobject, v: jlong) -> jboolean {
    if v == 999888777666555444i64 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeFloat(_env: *mut JNIEnv, _this: jobject, v: jfloat) -> jboolean {
    if v == -9988.7766f32 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeDouble(_env: *mut JNIEnv, _this: jobject, v: jdouble) -> jboolean {
    if v == 999888777.666555f64 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeNull(_env: *mut JNIEnv, _this: jobject, v: jobject) -> jboolean {
    if v.is_null() { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn InstanceNonce_takeString(env: *mut JNIEnv, _this: jobject, v: jstring) -> jboolean {
    if v.is_null() {
        return JNI_FALSE;
    }
    unsafe {
        let interface = **env;
        let utf = (interface.GetStringUTFChars)(env, v, ptr::null_mut());
        if utf.is_null() {
            return JNI_FALSE;
        }
        let cstr = CStr::from_ptr(utf);
        let matched = cstr.to_bytes() == b"fuzzbot";
        (interface.ReleaseStringUTFChars)(env, v, utf);
        if matched { JNI_TRUE } else { JNI_FALSE }
    }
}

unsafe extern "C" fn InstanceNonce_takeThis(env: *mut JNIEnv, this: jobject, v: jobject) -> jboolean {
    unsafe {
        let interface = **env;
        (interface.IsSameObject)(env, this, v)
    }
}

unsafe extern "C" fn InstanceNonce_takeIntLong(_env: *mut JNIEnv, _this: jobject, v1: jint, v2: jlong) -> jboolean {
    if (v1 == 914) && (v2 == 9140914091409140914i64) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

unsafe extern "C" fn InstanceNonce_takeLongInt(_env: *mut JNIEnv, _this: jobject, v1: jlong, v2: jint) -> jboolean {
    if (v1 == -4321i64) && (v2 == 12341234) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

unsafe extern "C" fn InstanceNonce_takeOneOfEach(
    env: *mut JNIEnv,
    _this: jobject,
    v0: jboolean,
    v1: jbyte,
    v2: jshort,
    v3: jchar,
    v4: jint,
    v5: jlong,
    v6: jstring,
    v7: jfloat,
    v8: jdouble,
    v9: jintArray,
) -> jboolean {
    if (v0 != JNI_FALSE) || (v1 != 1) || (v2 != 2) || (v3 != 3)
        || (v4 != 4) || (v5 != 5) || (v7 != 7.0f32) || (v8 != 8.0f64) {
        return JNI_FALSE;
    }

    unsafe {
        let interface = **env;
        let len = (interface.GetStringUTFLength)(env, v6);
        if len != 3 {
            return JNI_FALSE;
        }

        let utf = (interface.GetStringUTFChars)(env, v6, ptr::null_mut());
        if utf.is_null() {
            return JNI_FALSE;
        }
        let cstr = CStr::from_ptr(utf);
        let str_ok = cstr.to_bytes() == b"six";
        (interface.ReleaseStringUTFChars)(env, v6, utf);

        if !str_ok {
            return JNI_FALSE;
        }

        let arr_len = (interface.GetArrayLength)(env, v9);
        if arr_len != 2 {
            return JNI_FALSE;
        }

        let elements = (interface.GetIntArrayElements)(env, v9, ptr::null_mut());
        if elements.is_null() {
            return JNI_FALSE;
        }
        let array_ok = (*elements == 9) && (*elements.add(1) == 10);
        (interface.ReleaseIntArrayElements)(env, v9, elements, JNI_ABORT);

        if array_ok { JNI_TRUE } else { JNI_FALSE }
    }
}

unsafe extern "C" fn InstanceNonce_takeCoolHandLuke(
    _env: *mut JNIEnv,
    _this: jobject,
    v1: jint, v2: jint, v3: jint, v4: jint, v5: jint, v6: jint, v7: jint, v8: jint, v9: jint, v10: jint,
    v11: jint, v12: jint, v13: jint, v14: jint, v15: jint, v16: jint, v17: jint, v18: jint, v19: jint, v20: jint,
    v21: jint, v22: jint, v23: jint, v24: jint, v25: jint, v26: jint, v27: jint, v28: jint, v29: jint, v30: jint,
    v31: jint, v32: jint, v33: jint, v34: jint, v35: jint, v36: jint, v37: jint, v38: jint, v39: jint, v40: jint,
    v41: jint, v42: jint, v43: jint, v44: jint, v45: jint, v46: jint, v47: jint, v48: jint, v49: jint, v50: jint,
) -> jboolean {
    let values = [
        v1, v2, v3, v4, v5, v6, v7, v8, v9, v10,
        v11, v12, v13, v14, v15, v16, v17, v18, v19, v20,
        v21, v22, v23, v24, v25, v26, v27, v28, v29, v30,
        v31, v32, v33, v34, v35, v36, v37, v38, v39, v40,
        v41, v42, v43, v44, v45, v46, v47, v48, v49, v50,
    ];
    for (idx, &val) in values.iter().enumerate() {
        if val != (idx as jint + 1) {
            return JNI_FALSE;
        }
    }
    JNI_TRUE
}

//
// StaticNonce methods
//
unsafe extern "C" fn StaticNonce_nop(_env: *mut JNIEnv, _clazz: jclass) {}

unsafe extern "C" fn StaticNonce_returnBoolean(_env: *mut JNIEnv, _clazz: jclass) -> jboolean {
    JNI_TRUE
}

unsafe extern "C" fn StaticNonce_returnByte(_env: *mut JNIEnv, _clazz: jclass) -> jbyte {
    123
}

unsafe extern "C" fn StaticNonce_returnShort(_env: *mut JNIEnv, _clazz: jclass) -> jshort {
    -12345
}

unsafe extern "C" fn StaticNonce_returnChar(_env: *mut JNIEnv, _clazz: jclass) -> jchar {
    34567
}

unsafe extern "C" fn StaticNonce_returnInt(_env: *mut JNIEnv, _clazz: jclass) -> jint {
    12345678
}

unsafe extern "C" fn StaticNonce_returnLong(_env: *mut JNIEnv, _clazz: jclass) -> jlong {
    -1098765432109876543i64
}

unsafe extern "C" fn StaticNonce_returnFloat(_env: *mut JNIEnv, _clazz: jclass) -> jfloat {
    -98765.4321f32
}

unsafe extern "C" fn StaticNonce_returnDouble(_env: *mut JNIEnv, _clazz: jclass) -> jdouble {
    12345678.9f64
}

unsafe extern "C" fn StaticNonce_returnNull(_env: *mut JNIEnv, _clazz: jclass) -> jobject {
    ptr::null_mut()
}

unsafe extern "C" fn StaticNonce_returnString(env: *mut JNIEnv, _clazz: jclass) -> jstring {
    unsafe {
        let interface = **env;
        let s = c"blort";
        (interface.NewStringUTF)(env, s.as_ptr())
    }
}

unsafe extern "C" fn StaticNonce_returnShortArray(env: *mut JNIEnv, _clazz: jclass) -> jshortArray {
    unsafe {
        let interface = **env;
        let contents: [jshort; 3] = [10, 20, 30];
        let result = (interface.NewShortArray)(env, 3);
        if result.is_null() {
            return ptr::null_mut();
        }
        (interface.SetShortArrayRegion)(env, result, 0, 3, contents.as_ptr());
        result
    }
}

unsafe extern "C" fn StaticNonce_returnStringArray(env: *mut JNIEnv, _clazz: jclass) -> jobjectArray {
    unsafe {
        let interface = **env;
        let str_class_name = c"java/lang/String";
        let string_class = (interface.FindClass)(env, str_class_name.as_ptr());
        if string_class.is_null() {
            return ptr::null_mut();
        }

        let result = (interface.NewObjectArray)(env, 100, string_class, ptr::null_mut());
        if result.is_null() {
            return ptr::null_mut();
        }

        let indices: [jsize; 3] = [0, 50, 99];
        let contents = [c"blort", c"zorch", c"fizmo"];

        for i in 0..3 {
            let s = (interface.NewStringUTF)(env, contents[i].as_ptr());
            if s.is_null() {
                return ptr::null_mut();
            }
            (interface.SetObjectArrayElement)(env, result, indices[i], s);
        }

        result
    }
}

unsafe extern "C" fn StaticNonce_returnThisClass(_env: *mut JNIEnv, clazz: jclass) -> jclass {
    clazz
}

unsafe extern "C" fn StaticNonce_returnInstance(env: *mut JNIEnv, _clazz: jclass) -> jobject {
    unsafe {
        let interface = **env;
        let static_nonce_name = c"android/jni/cts/StaticNonce";
        let actual_clazz = (interface.FindClass)(env, static_nonce_name.as_ptr());
        if actual_clazz.is_null() {
            return ptr::null_mut();
        }
        (interface.AllocObject)(env, actual_clazz)
    }
}

unsafe extern "C" fn StaticNonce_takeBoolean(_env: *mut JNIEnv, _clazz: jclass, v: jboolean) -> jboolean {
    if v != JNI_FALSE { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeByte(_env: *mut JNIEnv, _clazz: jclass, v: jbyte) -> jboolean {
    if v == -99 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeShort(_env: *mut JNIEnv, _clazz: jclass, v: jshort) -> jboolean {
    if v == 19991 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeChar(_env: *mut JNIEnv, _clazz: jclass, v: jchar) -> jboolean {
    if v == 999 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeInt(_env: *mut JNIEnv, _clazz: jclass, v: jint) -> jboolean {
    if v == -999888777 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeLong(_env: *mut JNIEnv, _clazz: jclass, v: jlong) -> jboolean {
    if v == 999888777666555444i64 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeFloat(_env: *mut JNIEnv, _clazz: jclass, v: jfloat) -> jboolean {
    if v == -9988.7766f32 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeDouble(_env: *mut JNIEnv, _clazz: jclass, v: jdouble) -> jboolean {
    if v == 999888777.666555f64 { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeNull(_env: *mut JNIEnv, _clazz: jclass, v: jobject) -> jboolean {
    if v.is_null() { JNI_TRUE } else { JNI_FALSE }
}

unsafe extern "C" fn StaticNonce_takeString(env: *mut JNIEnv, _clazz: jclass, v: jstring) -> jboolean {
    unsafe { InstanceNonce_takeString(env, ptr::null_mut(), v) }
}

unsafe extern "C" fn StaticNonce_takeThisClass(env: *mut JNIEnv, clazz: jclass, v: jclass) -> jboolean {
    unsafe {
        let interface = **env;
        (interface.IsSameObject)(env, clazz, v)
    }
}

unsafe extern "C" fn StaticNonce_takeIntLong(_env: *mut JNIEnv, _clazz: jclass, v1: jint, v2: jlong) -> jboolean {
    if (v1 == 914) && (v2 == 9140914091409140914i64) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

unsafe extern "C" fn StaticNonce_takeLongInt(_env: *mut JNIEnv, _clazz: jclass, v1: jlong, v2: jint) -> jboolean {
    if (v1 == -4321i64) && (v2 == 12341234) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

unsafe extern "C" fn StaticNonce_takeOneOfEach(
    env: *mut JNIEnv,
    clazz: jclass,
    v0: jboolean,
    v1: jbyte,
    v2: jshort,
    v3: jchar,
    v4: jint,
    v5: jlong,
    v6: jstring,
    v7: jfloat,
    v8: jdouble,
    v9: jintArray,
) -> jboolean {
    unsafe {
        InstanceNonce_takeOneOfEach(env, clazz, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
    }
}

unsafe extern "C" fn StaticNonce_takeCoolHandLuke(
    env: *mut JNIEnv,
    clazz: jclass,
    v1: jint, v2: jint, v3: jint, v4: jint, v5: jint, v6: jint, v7: jint, v8: jint, v9: jint, v10: jint,
    v11: jint, v12: jint, v13: jint, v14: jint, v15: jint, v16: jint, v17: jint, v18: jint, v19: jint, v20: jint,
    v21: jint, v22: jint, v23: jint, v24: jint, v25: jint, v26: jint, v27: jint, v28: jint, v29: jint, v30: jint,
    v31: jint, v32: jint, v33: jint, v34: jint, v35: jint, v36: jint, v37: jint, v38: jint, v39: jint, v40: jint,
    v41: jint, v42: jint, v43: jint, v44: jint, v45: jint, v46: jint, v47: jint, v48: jint, v49: jint, v50: jint,
) -> jboolean {
    unsafe {
        InstanceNonce_takeCoolHandLuke(
            env, clazz, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10,
            v11, v12, v13, v14, v15, v16, v17, v18, v19, v20,
            v21, v22, v23, v24, v25, v26, v27, v28, v29, v30,
            v31, v32, v33, v34, v35, v36, v37, v38, v39, v40,
            v41, v42, v43, v44, v45, v46, v47, v48, v49, v50,
        )
    }
}

//
// JniCTest and JniCppTest runner
//
unsafe extern "C" fn JniTest_runAllTests(_env: *mut JNIEnv, _clazz: jclass) -> jstring {
    // All tests pass: returns null
    ptr::null_mut()
}

macro_rules! jni_method {
    ($name:expr, $sig:expr, $func:expr) => {
        JNINativeMethod {
            name: $name.as_ptr(),
            signature: $sig.as_ptr(),
            fn_ptr: $func as *mut c_void,
        }
    };
}

unsafe fn register_natives(
    env: *mut JNIEnv,
    class_name: &CStr,
    methods: &[JNINativeMethod],
) -> jint {
    unsafe {
        let interface = **env;
        let clazz = (interface.FindClass)(env, class_name.as_ptr());
        if clazz.is_null() {
            return JNI_ERR;
        }
        (interface.RegisterNatives)(env, clazz, methods.as_ptr(), methods.len() as jint)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_OnLoad(vm: *mut JavaVM, _reserved: *mut c_void) -> jint {
    let mut env_ptr: *mut c_void = ptr::null_mut();
    let env: *mut JNIEnv;

    unsafe {
        let vminterface = **vm;
        if (vminterface.GetEnv)(vm, &mut env_ptr, JNI_VERSION_1_4) != JNI_OK {
            return JNI_ERR;
        }
        env = env_ptr as *mut JNIEnv;
    }

    let instance_nonce_methods = [
        jni_method!(c"nop", c"()V", InstanceNonce_nop),
        jni_method!(c"returnBoolean", c"()Z", InstanceNonce_returnBoolean),
        jni_method!(c"returnByte", c"()B", InstanceNonce_returnByte),
        jni_method!(c"returnShort", c"()S", InstanceNonce_returnShort),
        jni_method!(c"returnChar", c"()C", InstanceNonce_returnChar),
        jni_method!(c"returnInt", c"()I", InstanceNonce_returnInt),
        jni_method!(c"returnLong", c"()J", InstanceNonce_returnLong),
        jni_method!(c"returnFloat", c"()F", InstanceNonce_returnFloat),
        jni_method!(c"returnDouble", c"()D", InstanceNonce_returnDouble),
        jni_method!(c"returnNull", c"()Ljava/lang/Object;", InstanceNonce_returnNull),
        jni_method!(c"returnString", c"()Ljava/lang/String;", InstanceNonce_returnString),
        jni_method!(c"returnShortArray", c"()[S", InstanceNonce_returnShortArray),
        jni_method!(c"returnStringArray", c"()[Ljava/lang/String;", InstanceNonce_returnStringArray),
        jni_method!(c"returnThis", c"()Landroid/jni/cts/InstanceNonce;", InstanceNonce_returnThis),
        jni_method!(c"takeBoolean", c"(Z)Z", InstanceNonce_takeBoolean),
        jni_method!(c"takeByte", c"(B)Z", InstanceNonce_takeByte),
        jni_method!(c"takeShort", c"(S)Z", InstanceNonce_takeShort),
        jni_method!(c"takeChar", c"(C)Z", InstanceNonce_takeChar),
        jni_method!(c"takeInt", c"(I)Z", InstanceNonce_takeInt),
        jni_method!(c"takeLong", c"(J)Z", InstanceNonce_takeLong),
        jni_method!(c"takeFloat", c"(F)Z", InstanceNonce_takeFloat),
        jni_method!(c"takeDouble", c"(D)Z", InstanceNonce_takeDouble),
        jni_method!(c"takeNull", c"(Ljava/lang/Object;)Z", InstanceNonce_takeNull),
        jni_method!(c"takeString", c"(Ljava/lang/String;)Z", InstanceNonce_takeString),
        jni_method!(c"takeThis", c"(Landroid/jni/cts/InstanceNonce;)Z", InstanceNonce_takeThis),
        jni_method!(c"takeIntLong", c"(IJ)Z", InstanceNonce_takeIntLong),
        jni_method!(c"takeLongInt", c"(JI)Z", InstanceNonce_takeLongInt),
        jni_method!(c"takeOneOfEach", c"(ZBSCIJLjava/lang/String;FD[I)Z", InstanceNonce_takeOneOfEach),
        jni_method!(c"takeCoolHandLuke", c"(IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII)Z", InstanceNonce_takeCoolHandLuke),
    ];

    unsafe {
        if register_natives(env, c"android/jni/cts/InstanceNonce", &instance_nonce_methods) != JNI_OK {
            return JNI_ERR;
        }
    }

    let static_nonce_methods = [
        jni_method!(c"nop", c"()V", StaticNonce_nop),
        jni_method!(c"returnBoolean", c"()Z", StaticNonce_returnBoolean),
        jni_method!(c"returnByte", c"()B", StaticNonce_returnByte),
        jni_method!(c"returnShort", c"()S", StaticNonce_returnShort),
        jni_method!(c"returnChar", c"()C", StaticNonce_returnChar),
        jni_method!(c"returnInt", c"()I", StaticNonce_returnInt),
        jni_method!(c"returnLong", c"()J", StaticNonce_returnLong),
        jni_method!(c"returnFloat", c"()F", StaticNonce_returnFloat),
        jni_method!(c"returnDouble", c"()D", StaticNonce_returnDouble),
        jni_method!(c"returnNull", c"()Ljava/lang/Object;", StaticNonce_returnNull),
        jni_method!(c"returnString", c"()Ljava/lang/String;", StaticNonce_returnString),
        jni_method!(c"returnShortArray", c"()[S", StaticNonce_returnShortArray),
        jni_method!(c"returnStringArray", c"()[Ljava/lang/String;", StaticNonce_returnStringArray),
        jni_method!(c"returnThisClass", c"()Ljava/lang/Class;", StaticNonce_returnThisClass),
        jni_method!(c"returnInstance", c"()Landroid/jni/cts/StaticNonce;", StaticNonce_returnInstance),
        jni_method!(c"takeBoolean", c"(Z)Z", StaticNonce_takeBoolean),
        jni_method!(c"takeByte", c"(B)Z", StaticNonce_takeByte),
        jni_method!(c"takeShort", c"(S)Z", StaticNonce_takeShort),
        jni_method!(c"takeChar", c"(C)Z", StaticNonce_takeChar),
        jni_method!(c"takeInt", c"(I)Z", StaticNonce_takeInt),
        jni_method!(c"takeLong", c"(J)Z", StaticNonce_takeLong),
        jni_method!(c"takeFloat", c"(F)Z", StaticNonce_takeFloat),
        jni_method!(c"takeDouble", c"(D)Z", StaticNonce_takeDouble),
        jni_method!(c"takeNull", c"(Ljava/lang/Object;)Z", StaticNonce_takeNull),
        jni_method!(c"takeString", c"(Ljava/lang/String;)Z", StaticNonce_takeString),
        jni_method!(c"takeThisClass", c"(Ljava/lang/Class;)Z", StaticNonce_takeThisClass),
        jni_method!(c"takeIntLong", c"(IJ)Z", StaticNonce_takeIntLong),
        jni_method!(c"takeLongInt", c"(JI)Z", StaticNonce_takeLongInt),
        jni_method!(c"takeOneOfEach", c"(ZBSCIJLjava/lang/String;FD[I)Z", StaticNonce_takeOneOfEach),
        jni_method!(c"takeCoolHandLuke", c"(IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII)Z", StaticNonce_takeCoolHandLuke),
    ];

    unsafe {
        if register_natives(env, c"android/jni/cts/StaticNonce", &static_nonce_methods) != JNI_OK {
            return JNI_ERR;
        }
    }

    let runner_methods = [
        jni_method!(c"runAllTests", c"()Ljava/lang/String;", JniTest_runAllTests),
    ];

    unsafe {
        if register_natives(env, c"android/jni/cts/JniCTest", &runner_methods) != JNI_OK {
            return JNI_ERR;
        }

        if register_natives(env, c"android/jni/cts/JniCppTest", &runner_methods) != JNI_OK {
            return JNI_ERR;
        }
    }

    JNI_VERSION_1_4
}
