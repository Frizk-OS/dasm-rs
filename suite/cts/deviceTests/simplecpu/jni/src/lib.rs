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

/// Simple LCG matching standard C rand() with srand(seed)
struct SimpleRand {
    state: u64,
}

impl SimpleRand {
    fn new(seed: u32) -> Self {
        Self {
            state: seed as u64,
        }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.state / 65536) % 32768) as u32
    }

    fn init_int_slice(&mut self, slice: &mut [i32]) {
        for item in slice.iter_mut() {
            *item = self.next_u32() as i32;
        }
    }

    fn init_float_slice(&mut self, slice: &mut [f32]) {
        for item in slice.iter_mut() {
            *item = self.next_u32() as f32;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_android_cts_simplecpu_CpuNative_runSort(
    _env: *mut JNIEnv,
    _clazz: jclass,
    number_elements: jint,
    repetition: jint,
) -> jdouble {
    if number_elements <= 0 || repetition <= 0 {
        return 0.0;
    }

    let n = number_elements as usize;
    let mut data = vec![0i32; n];
    let mut total_time_ms = 0.0f64;

    for _ in 0..repetition {
        let mut rng = SimpleRand::new(0);
        rng.init_int_slice(&mut data);

        let start = Instant::now();
        data.sort_unstable();
        let elapsed = start.elapsed();
        total_time_ms += elapsed.as_secs_f64() * 1000.0;
    }

    total_time_ms
}

fn do_matrix_multiplication(a: &[f32], b: &[f32], c: &mut [f32], n: usize) {
    const M: usize = 8;
    for i in 0..n {
        for j in (0..n).step_by(M) {
            let mut sum = [0.0f32; M];
            for k in 0..n {
                let a_val = a[i * n + k];
                for offset in 0..M {
                    sum[offset] += a_val * b[k * n + j + offset];
                }
            }
            for offset in 0..M {
                c[i * n + j + offset] = sum[offset];
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_android_cts_simplecpu_CpuNative_runMatrixMultiplication(
    _env: *mut JNIEnv,
    _clazz: jclass,
    n_param: jint,
    repetition: jint,
) -> jdouble {
    if n_param <= 0 || repetition <= 0 {
        return 0.0;
    }

    let n = n_param as usize;
    let size = n * n;
    let mut a = vec![0.0f32; size];
    let mut b = vec![0.0f32; size];
    let mut c = vec![0.0f32; size];

    let mut total_time_ms = 0.0f64;

    for _ in 0..repetition {
        let mut rng_a = SimpleRand::new(0);
        rng_a.init_float_slice(&mut a);

        let mut rng_b = SimpleRand::new(1);
        rng_b.init_float_slice(&mut b);

        let start = Instant::now();
        do_matrix_multiplication(&a, &b, &mut c, n);
        let elapsed = start.elapsed();
        total_time_ms += elapsed.as_secs_f64() * 1000.0;
    }

    total_time_ms
}
