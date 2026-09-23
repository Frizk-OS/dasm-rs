/*
 * Copyright (C) 2011-2013 The Android Open Source Project
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

const PR_CAPBSET_READ: c_int = 23;
const PR_GET_NO_NEW_PRIVS: c_int = 39;

const AF_NETLINK: c_int = 16;
const SOCK_DGRAM: c_int = 2;
const NETLINK_KOBJECT_UEVENT: c_int = 15;

const ENOENT: c_int = 2;

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

#[repr(C)]
struct SockaddrNl {
    nl_family: u16,
    nl_pad: u16,
    nl_pid: u32,
    nl_groups: u32,
}

#[repr(C)]
struct Iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}

#[repr(C)]
struct Msghdr {
    msg_name: *mut c_void,
    msg_namelen: u32,
    msg_iov: *mut Iovec,
    msg_iovlen: usize,
    msg_control: *mut c_void,
    msg_controllen: usize,
    msg_flags: c_int,
}

unsafe extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn stat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn lstat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn chmod(path: *const c_char, mode: u32) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn sendmsg(sockfd: c_int, msg: *const Msghdr, flags: c_int) -> isize;
    fn getpwuid(uid: u32) -> *mut Passwd;
    fn getgrgid(gid: u32) -> *mut Group;
    fn strlen(s: *const c_char) -> usize;
}

// Cached Field IDs for android.os.cts.FileUtils$FileStatus
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

// ---------------------------------------------------------------------------
// android.os.cts.CpuFeatures
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_os_cts_CpuFeatures_isArmCpu(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    if cfg!(target_arch = "arm") {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_os_cts_CpuFeatures_isArm7Compatible(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    if cfg!(target_arch = "arm") {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_os_cts_CpuFeatures_isMipsCpu(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    if cfg!(target_arch = "mips") {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_os_cts_CpuFeatures_isX86Cpu(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

// ---------------------------------------------------------------------------
// android.os.cts.OSFeatures
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_os_cts_OSFeatures_getNoNewPrivs(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jint {
    unsafe { prctl(PR_GET_NO_NEW_PRIVS, 0, 0, 0, 0) as jint }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_os_cts_OSFeatures_prctlCapBsetRead(
    _env: *mut JNIEnv,
    _thiz: jobject,
    i: jint,
) -> jint {
    unsafe { prctl(PR_CAPBSET_READ, i as c_int, 0, 0, 0) as jint }
}

// ---------------------------------------------------------------------------
// android.os.cts.FileUtils
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_os_cts_FileUtils_getUserName(
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
pub unsafe extern "C" fn Java_android_os_cts_FileUtils_getGroupName(
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
pub unsafe extern "C" fn Java_android_os_cts_FileUtils_setPermissions(
    env: *mut JNIEnv,
    _thiz: jobject,
    path: jstring,
    mode: jint,
) -> jint {
    if path.is_null() {
        return -1;
    }
    let chars = unsafe { ((**env).GetStringUTFChars)(env, path, ptr::null_mut()) };
    if chars.is_null() {
        return -1;
    }

    if unsafe { strlen(chars) } == 0 {
        unsafe { ((**env).ReleaseStringUTFChars)(env, path, chars) };
        return ENOENT;
    }

    let ret = unsafe { chmod(chars, mode as u32) };
    let errno_val = if ret == 0 {
        0
    } else {
        std::io::Error::last_os_error().raw_os_error().unwrap_or(1)
    };
    unsafe { ((**env).ReleaseStringUTFChars)(env, path, chars) };
    errno_val as jint
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_os_cts_FileUtils_getFileStatus(
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
    }

    JNI_TRUE
}

// ---------------------------------------------------------------------------
// android.net.cts.NetlinkSocket
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_net_cts_NetlinkSocket_create_1native(
    env: *mut JNIEnv,
    _clazz: jclass,
    file_descriptor: jobject,
) {
    let sock = unsafe { socket(AF_NETLINK, SOCK_DGRAM, NETLINK_KOBJECT_UEVENT) };
    if sock < 0 {
        let ex_cls = unsafe {
            let name = b"java/net/SocketException\0";
            ((**env).FindClass)(env, name.as_ptr() as *const c_char)
        };
        if !ex_cls.is_null() {
            let msg = b"Can't create socket\0";
            unsafe { ((**env).ThrowNew)(env, ex_cls, msg.as_ptr() as *const c_char) };
        }
        return;
    }

    // Set descriptor field: FileDescriptor.descriptor
    if !file_descriptor.is_null() {
        let fd_cls = unsafe { ((**env).GetObjectClass)(env, file_descriptor) };
        if !fd_cls.is_null() {
            let fid = unsafe {
                ((**env).GetFieldID)(
                    env,
                    fd_cls,
                    b"descriptor\0".as_ptr() as *const c_char,
                    b"I\0".as_ptr() as *const c_char,
                )
            };
            if !fid.is_null() {
                unsafe { ((**env).SetIntField)(env, file_descriptor, fid, sock as jint) };
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_net_cts_NetlinkSocket_sendmsg(
    env: *mut JNIEnv,
    _clazz: jclass,
    file_descriptor: jobject,
    pid: jint,
    packet: jbyteArray,
) -> jint {
    if packet.is_null() || file_descriptor.is_null() {
        return -1;
    }

    let bytes = unsafe { ((**env).GetByteArrayElements)(env, packet, ptr::null_mut()) };
    let length = unsafe { ((**env).GetArrayLength)(env, packet) } as usize;

    let mut snl: SockaddrNl = unsafe { mem::zeroed() };
    snl.nl_family = AF_NETLINK as u16;
    snl.nl_pid = pid as u32;

    let mut iov = Iovec {
        iov_base: bytes as *mut c_void,
        iov_len: length,
    };

    let msg = Msghdr {
        msg_name: &mut snl as *mut _ as *mut c_void,
        msg_namelen: mem::size_of::<SockaddrNl>() as u32,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };

    // Get sock from FileDescriptor
    let fd_cls = unsafe { ((**env).GetObjectClass)(env, file_descriptor) };
    let mut sock = -1;
    if !fd_cls.is_null() {
        let fid = unsafe {
            ((**env).GetFieldID)(
                env,
                fd_cls,
                b"descriptor\0".as_ptr() as *const c_char,
                b"I\0".as_ptr() as *const c_char,
            )
        };
        if !fid.is_null() {
            sock = unsafe { ((**env).GetIntField)(env, file_descriptor, fid) };
        }
    }

    let ret = if sock >= 0 {
        unsafe { sendmsg(sock, &msg, 0) as jint }
    } else {
        -1
    };

    unsafe { ((**env).ReleaseByteArrayElements)(env, packet, bytes, 0) };
    ret
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

fn register_all_classes(env: *mut JNIEnv) -> bool {
    let cpu_methods = [
        JNINativeMethod {
            name: b"isArmCpu\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_CpuFeatures_isArmCpu as *mut c_void,
        },
        JNINativeMethod {
            name: b"isArm7Compatible\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_CpuFeatures_isArm7Compatible as *mut c_void,
        },
        JNINativeMethod {
            name: b"isMipsCpu\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_CpuFeatures_isMipsCpu as *mut c_void,
        },
        JNINativeMethod {
            name: b"isX86Cpu\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_CpuFeatures_isX86Cpu as *mut c_void,
        },
    ];

    let os_methods = [
        JNINativeMethod {
            name: b"getNoNewPrivs\0".as_ptr() as *const c_char,
            signature: b"()I\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_OSFeatures_getNoNewPrivs as *mut c_void,
        },
        JNINativeMethod {
            name: b"prctlCapBsetRead\0".as_ptr() as *const c_char,
            signature: b"(I)I\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_OSFeatures_prctlCapBsetRead as *mut c_void,
        },
    ];

    let file_utils_methods = [
        JNINativeMethod {
            name: b"getFileStatus\0".as_ptr() as *const c_char,
            signature: b"(Ljava/lang/String;Landroid/os/cts/FileUtils$FileStatus;Z)Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_FileUtils_getFileStatus as *mut c_void,
        },
        JNINativeMethod {
            name: b"getUserName\0".as_ptr() as *const c_char,
            signature: b"(I)Ljava/lang/String;\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_FileUtils_getUserName as *mut c_void,
        },
        JNINativeMethod {
            name: b"getGroupName\0".as_ptr() as *const c_char,
            signature: b"(I)Ljava/lang/String;\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_FileUtils_getGroupName as *mut c_void,
        },
        JNINativeMethod {
            name: b"setPermissions\0".as_ptr() as *const c_char,
            signature: b"(Ljava/lang/String;I)I\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_os_cts_FileUtils_setPermissions as *mut c_void,
        },
    ];

    let netlink_methods = [
        JNINativeMethod {
            name: b"create_native\0".as_ptr() as *const c_char,
            signature: b"(Ljava/io/FileDescriptor;)V\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_net_cts_NetlinkSocket_create_1native as *mut c_void,
        },
        JNINativeMethod {
            name: b"sendmsg\0".as_ptr() as *const c_char,
            signature: b"(Ljava/io/FileDescriptor;I[B)I\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_net_cts_NetlinkSocket_sendmsg as *mut c_void,
        },
    ];

    unsafe {
        let cpu_cls = b"android/os/cts/CpuFeatures\0".as_ptr() as *const c_char;
        let os_cls = b"android/os/cts/OSFeatures\0".as_ptr() as *const c_char;
        let file_cls = b"android/os/cts/FileUtils\0".as_ptr() as *const c_char;
        let netlink_cls = b"android/net/cts/NetlinkSocket\0".as_ptr() as *const c_char;

        register_natives(env, cpu_cls, &cpu_methods)
            && register_natives(env, os_cls, &os_methods)
            && register_natives(env, file_cls, &file_utils_methods)
            && register_natives(env, netlink_cls, &netlink_methods)
    }
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
    if !register_all_classes(env) {
        return JNI_ERR;
    }

    JNI_VERSION_1_4
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_cpu_features() {
        let is_x86 = unsafe { Java_android_os_cts_CpuFeatures_isX86Cpu(ptr::null_mut(), ptr::null_mut()) };
        let is_arm = unsafe { Java_android_os_cts_CpuFeatures_isArmCpu(ptr::null_mut(), ptr::null_mut()) };
        let is_mips = unsafe { Java_android_os_cts_CpuFeatures_isMipsCpu(ptr::null_mut(), ptr::null_mut()) };

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            assert_eq!(is_x86, JNI_TRUE);
            assert_eq!(is_arm, JNI_FALSE);
            assert_eq!(is_mips, JNI_FALSE);
        }
    }

    #[test]
    fn test_os_features() {
        let no_new_privs = unsafe { Java_android_os_cts_OSFeatures_getNoNewPrivs(ptr::null_mut(), ptr::null_mut()) };
        assert!(no_new_privs >= 0);

        let cap_bset = unsafe { Java_android_os_cts_OSFeatures_prctlCapBsetRead(ptr::null_mut(), ptr::null_mut(), 0) };
        assert!(cap_bset >= 0);
    }

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
    fn test_stat_structure() {
        let mut s: Stat = unsafe { mem::zeroed() };
        let path = b"/dev/null\0";
        let res = unsafe { stat(path.as_ptr() as *const c_char, &mut s) };
        assert_eq!(res, 0);
        assert_ne!(s.st_mode, 0);
    }
}
