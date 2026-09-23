/*
 * Copyright (C) 2010 The Android Open Source Project
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

pub mod jni_sys;

use jni_sys::*;
use std::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

const X_OK: c_int = 1;

#[repr(C)]
struct Stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: i64,
    st_blksize: c_long,
    st_blocks: i64,
    st_atime: c_long,
    st_atime_nsec: c_long,
    st_mtime: c_long,
    st_mtime_nsec: c_long,
    st_ctime: c_long,
    st_ctime_nsec: c_long,
    __unused: [c_long; 3],
}

#[repr(C)]
struct Passwd {
    pw_name: *mut c_char,
    pw_passwd: *mut c_char,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: *mut c_char,
    pw_dir: *mut c_char,
    pw_shell: *mut c_char,
}

#[repr(C)]
struct Group {
    gr_name: *mut c_char,
    gr_passwd: *mut c_char,
    gr_gid: u32,
    gr_mem: *mut *mut c_char,
}

unsafe extern "C" {
    fn stat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn lstat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn getpwuid(uid: u32) -> *mut Passwd;
    fn getgrgid(gid: u32) -> *mut Group;
}

// Cached Field IDs for com.android.cts.verifier.os.FileUtils$FileStatus
static G_FILE_STATUS_DEV: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_INO: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_MODE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_NLINK: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_UID: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_GID: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_SIZE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_BLKSIZE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_BLOCKS: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_ATIME: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_MTIME: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_CTIME: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_EXECUTABLE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());

// ---------------------------------------------------------------------------
// com.android.cts.verifier.os.FileUtils
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_android_cts_verifier_os_FileUtils_getUserName(
    env: *mut JNIEnv,
    _thiz: jobject,
    uid: jint,
) -> jstring {
    let pwd = unsafe { getpwuid(uid as u32) };
    if pwd.is_null() {
        return ptr::null_mut();
    }
    let name_ptr = unsafe { (*pwd).pw_name };
    if name_ptr.is_null() {
        return ptr::null_mut();
    }
    unsafe { ((**env).NewStringUTF)(env, name_ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_android_cts_verifier_os_FileUtils_getGroupName(
    env: *mut JNIEnv,
    _thiz: jobject,
    gid: jint,
) -> jstring {
    let grp = unsafe { getgrgid(gid as u32) };
    if grp.is_null() {
        return ptr::null_mut();
    }
    let name_ptr = unsafe { (*grp).gr_name };
    if name_ptr.is_null() {
        return ptr::null_mut();
    }
    unsafe { ((**env).NewStringUTF)(env, name_ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_com_android_cts_verifier_os_FileUtils_getFileStatus(
    env: *mut JNIEnv,
    _thiz: jobject,
    path: jstring,
    file_status: jobject,
    stat_links: jboolean,
) -> jboolean {
    if path.is_null() {
        return JNI_FALSE;
    }
    let chars = unsafe { ((**env).GetStringUTFChars)(env, path, ptr::null_mut()) };
    if chars.is_null() {
        return JNI_FALSE;
    }

    let mut s: Stat = unsafe { mem::zeroed() };
    let res = if stat_links == JNI_TRUE {
        unsafe { lstat(chars, &mut s) }
    } else {
        unsafe { stat(chars, &mut s) }
    };

    let is_exec = unsafe { access(chars, X_OK) } == 0;
    unsafe { ((**env).ReleaseStringUTFChars)(env, path, chars) };

    if res != 0 {
        return JNI_FALSE;
    }

    if !file_status.is_null() {
        let set_int = |atomic_fid: &AtomicPtr<c_void>, field_name: &[u8], val: i32| {
            let mut fid = atomic_fid.load(Ordering::Relaxed);
            if fid.is_null() {
                let cls = unsafe { ((**env).GetObjectClass)(env, file_status) };
                if !cls.is_null() {
                    fid = unsafe {
                        ((**env).GetFieldID)(
                            env,
                            cls,
                            field_name.as_ptr() as *const c_char,
                            b"I\0".as_ptr() as *const c_char,
                        )
                    };
                    atomic_fid.store(fid, Ordering::Relaxed);
                }
            }
            if !fid.is_null() {
                unsafe { ((**env).SetIntField)(env, file_status, fid, val) };
            }
        };

        let set_long = |atomic_fid: &AtomicPtr<c_void>, field_name: &[u8], val: i64| {
            let mut fid = atomic_fid.load(Ordering::Relaxed);
            if fid.is_null() {
                let cls = unsafe { ((**env).GetObjectClass)(env, file_status) };
                if !cls.is_null() {
                    fid = unsafe {
                        ((**env).GetFieldID)(
                            env,
                            cls,
                            field_name.as_ptr() as *const c_char,
                            b"J\0".as_ptr() as *const c_char,
                        )
                    };
                    atomic_fid.store(fid, Ordering::Relaxed);
                }
            }
            if !fid.is_null() {
                unsafe { ((**env).SetLongField)(env, file_status, fid, val) };
            }
        };

        let set_bool = |atomic_fid: &AtomicPtr<c_void>, field_name: &[u8], val: bool| {
            let mut fid = atomic_fid.load(Ordering::Relaxed);
            if fid.is_null() {
                let cls = unsafe { ((**env).GetObjectClass)(env, file_status) };
                if !cls.is_null() {
                    fid = unsafe {
                        ((**env).GetFieldID)(
                            env,
                            cls,
                            field_name.as_ptr() as *const c_char,
                            b"Z\0".as_ptr() as *const c_char,
                        )
                    };
                    atomic_fid.store(fid, Ordering::Relaxed);
                }
            }
            if !fid.is_null() {
                unsafe {
                    ((**env).SetBooleanField)(
                        env,
                        file_status,
                        fid,
                        if val { JNI_TRUE } else { JNI_FALSE },
                    )
                };
            }
        };

        set_int(&G_FILE_STATUS_DEV, b"dev\0", s.st_dev as i32);
        set_int(&G_FILE_STATUS_INO, b"ino\0", s.st_ino as i32);
        set_int(&G_FILE_STATUS_MODE, b"mode\0", s.st_mode as i32);
        set_int(&G_FILE_STATUS_NLINK, b"nlink\0", s.st_nlink as i32);
        set_int(&G_FILE_STATUS_UID, b"uid\0", s.st_uid as i32);
        set_int(&G_FILE_STATUS_GID, b"gid\0", s.st_gid as i32);
        set_long(&G_FILE_STATUS_SIZE, b"size\0", s.st_size);
        set_int(&G_FILE_STATUS_BLKSIZE, b"blksize\0", s.st_blksize as i32);
        set_long(&G_FILE_STATUS_BLOCKS, b"blocks\0", s.st_blocks);
        set_long(&G_FILE_STATUS_ATIME, b"atime\0", s.st_atime as i64);
        set_long(&G_FILE_STATUS_MTIME, b"mtime\0", s.st_mtime as i64);
        set_long(&G_FILE_STATUS_CTIME, b"ctime\0", s.st_ctime as i64);
        set_bool(&G_FILE_STATUS_EXECUTABLE, b"executable\0", is_exec);
    }

    JNI_TRUE
}

// ---------------------------------------------------------------------------
// JNI Dynamic Registration
// ---------------------------------------------------------------------------

unsafe fn register_natives(
    env: *mut JNIEnv,
    class_name: *const c_char,
    methods: &[JNINativeMethod],
) -> bool {
    let clazz = unsafe { ((**env).FindClass)(env, class_name) };
    if clazz.is_null() {
        return false;
    }
    let res = unsafe { ((**env).RegisterNatives)(env, clazz, methods.as_ptr(), methods.len() as jint) };
    res == JNI_OK
}

fn register_verifier_file_utils(env: *mut JNIEnv) -> bool {
    let methods = [
        JNINativeMethod {
            name: b"getFileStatus\0".as_ptr() as *const c_char,
            signature: b"(Ljava/lang/String;Lcom/android/cts/verifier/os/FileUtils$FileStatus;Z)Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_com_android_cts_verifier_os_FileUtils_getFileStatus as *mut c_void,
        },
        JNINativeMethod {
            name: b"getUserName\0".as_ptr() as *const c_char,
            signature: b"(I)Ljava/lang/String;\0".as_ptr() as *const c_char,
            fn_ptr: Java_com_android_cts_verifier_os_FileUtils_getUserName as *mut c_void,
        },
        JNINativeMethod {
            name: b"getGroupName\0".as_ptr() as *const c_char,
            signature: b"(I)Ljava/lang/String;\0".as_ptr() as *const c_char,
            fn_ptr: Java_com_android_cts_verifier_os_FileUtils_getGroupName as *mut c_void,
        },
    ];

    let file_cls = b"com/android/cts/verifier/os/FileUtils\0".as_ptr() as *const c_char;
    unsafe { register_natives(env, file_cls, &methods) }
}

// ---------------------------------------------------------------------------
// JNI_OnLoad
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_OnLoad(vm: *mut JavaVM, _reserved: *mut c_void) -> jint {
    let mut env_ptr: *mut c_void = ptr::null_mut();
    let res = unsafe {
        ((**vm).GetEnv)(vm, &mut env_ptr, JNI_VERSION_1_4)
    };
    if res != JNI_OK {
        return JNI_ERR;
    }

    let env = env_ptr as *mut JNIEnv;
    if !register_verifier_file_utils(env) {
        return JNI_ERR;
    }

    JNI_VERSION_1_4
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_passwd_and_group() {
        let pwd = unsafe { getpwuid(0) };
        assert!(!pwd.is_null());
        let name = unsafe { CStr::from_ptr((*pwd).pw_name) };
        assert_eq!(name.to_str().unwrap(), "root");

        let grp = unsafe { getgrgid(0) };
        assert!(!grp.is_null());
        let gname = unsafe { CStr::from_ptr((*grp).gr_name) };
        assert_eq!(gname.to_str().unwrap(), "root");
    }

    #[test]
    fn test_stat_and_access() {
        let mut s: Stat = unsafe { mem::zeroed() };
        let path = b"/bin/sh\0";
        let res = unsafe { stat(path.as_ptr() as *const c_char, &mut s) };
        assert_eq!(res, 0);

        let is_exec = unsafe { access(path.as_ptr() as *const c_char, X_OK) };
        assert_eq!(is_exec, 0);
    }
}
