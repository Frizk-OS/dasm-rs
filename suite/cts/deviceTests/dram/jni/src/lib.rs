/*
 * Copyright (C) 2012 The Android Open Source Project
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

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_variables, dead_code)]

use std::ffi::{c_char, c_void};
use std::time::Instant;

pub type jint = i32;
pub type jdouble = f64;
pub type jobject = *mut c_void;
pub type jclass = jobject;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct JNINativeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub reserved3: *mut c_void,

    pub GetVersion: *mut c_void,
    pub DefineClass: *mut c_void,
    pub FindClass: unsafe extern "C" fn(*mut JNIEnv, *const c_char) -> jclass,
    pub FromReflectedMethod: *mut c_void,
    pub FromReflectedField: *mut c_void,
    pub ToReflectedMethod: *mut c_void,
    pub GetSuperclass: *mut c_void,
    pub IsAssignableFrom: *mut c_void,
    pub ToReflectedField: *mut c_void,

    pub Throw: *mut c_void,
    pub ThrowNew: unsafe extern "C" fn(*mut JNIEnv, jclass, *const c_char) -> jint,
}

pub type JNIEnv = *const JNINativeInterface;

unsafe fn throw_oom(env: *mut JNIEnv) {
    unsafe {
        let interface = **env;
        let class_name = c"java/lang/OutOfMemoryError";
        let message = c"No memory";
        let clazz = (interface.FindClass)(env, class_name.as_ptr());
        if !clazz.is_null() {
            (interface.ThrowNew)(env, clazz, message.as_ptr());
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_android_cts_dram_MemoryNative_runMemcpy(
    env: *mut JNIEnv,
    _clazz: jclass,
    buffer_size: jint,
    repetition: jint,
) -> jdouble {
    if buffer_size <= 0 || repetition <= 0 {
        return 0.0;
    }

    let size = buffer_size as usize;
    let mut src = vec![0u8; size];
    let mut dst = vec![0u8; size];

    let start = Instant::now();
    for i in 0..repetition {
        dst.copy_from_slice(&src);
        src[size - 1] = (i & 0xff) as u8;
    }
    let elapsed = start.elapsed();
    elapsed.as_secs_f64() * 1000.0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_android_cts_dram_MemoryNative_runMemset(
    env: *mut JNIEnv,
    _clazz: jclass,
    buffer_size: jint,
    repetition: jint,
    c: jint,
) -> jdouble {
    if buffer_size <= 0 || repetition <= 0 {
        return 0.0;
    }

    let size = buffer_size as usize;
    let mut dst = vec![0u8; size];

    let start = Instant::now();
    for i in 0..repetition {
        let fill_byte = ((c + i) & 0xff) as u8;
        dst.fill(fill_byte);
    }
    let elapsed = start.elapsed();
    elapsed.as_secs_f64() * 1000.0
}
