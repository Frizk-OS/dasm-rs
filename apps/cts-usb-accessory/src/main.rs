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

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const USB_DIR_OUT: u8 = 0x00;
const USB_DIR_IN: u8 = 0x80;
const USB_TYPE_VENDOR: u8 = 0x40;

const ACCESSORY_GET_PROTOCOL: u8 = 51;
const ACCESSORY_SEND_STRING: u8 = 52;
const ACCESSORY_START: u8 = 53;

const ACCESSORY_STRING_MANUFACTURER: u16 = 0;
const ACCESSORY_STRING_MODEL: u16 = 1;
const ACCESSORY_STRING_DESCRIPTION: u16 = 2;
const ACCESSORY_STRING_VERSION: u16 = 3;
const ACCESSORY_STRING_URI: u16 = 4;
const ACCESSORY_STRING_SERIAL: u16 = 5;

const USB_DT_DEVICE: u8 = 1;
const USB_DT_CONFIG: u8 = 2;
const USB_DT_INTERFACE: u8 = 4;
const USB_DT_ENDPOINT: u8 = 5;

const USB_ENDPOINT_DIR_MASK: u8 = 0x80;

#[repr(C)]
struct UsbdevfsCtrltransfer {
    b_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
    timeout: u32,
    data: *mut std::ffi::c_void,
}

#[repr(C)]
struct UsbdevfsBulktransfer {
    ep: u32,
    len: u32,
    timeout: u32,
    data: *mut std::ffi::c_void,
}

// Linux usbfs ioctl codes
// _IOWR('U', 0, struct usbdevfs_ctrltransfer)
const USBDEVFS_CONTROL: libc::c_ulong = 0xc0185500;
// _IOWR('U', 2, struct usbdevfs_bulktransfer)
const USBDEVFS_BULK: libc::c_ulong = 0xc0185502;
// _IOR('U', 15, unsigned int)
const USBDEVFS_CLAIMINTERFACE: libc::c_ulong = 0x8004550f;
// _IOR('U', 16, unsigned int)
const USBDEVFS_RELEASEINTERFACE: libc::c_ulong = 0x80045510;

mod libc {
    pub type c_int = i32;
    pub type c_ulong = u64;

    unsafe extern "C" {
        pub fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    }
}

fn usb_control_transfer(
    fd: RawFd,
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    buffer: &mut [u8],
    timeout_ms: u32,
) -> Result<usize, String> {
    let mut ctrl = UsbdevfsCtrltransfer {
        b_request_type: request_type,
        b_request: request,
        w_value: value,
        w_index: index,
        w_length: buffer.len() as u16,
        timeout: timeout_ms,
        data: buffer.as_mut_ptr() as *mut std::ffi::c_void,
    };

    let ret = unsafe { libc::ioctl(fd, USBDEVFS_CONTROL, &mut ctrl as *mut _) };
    if ret < 0 {
        Err(format!("ioctl USBDEVFS_CONTROL failed: {}", std::io::Error::last_os_error()))
    } else {
        Ok(ret as usize)
    }
}

fn usb_bulk_transfer(
    fd: RawFd,
    ep: u32,
    buffer: &mut [u8],
    timeout_ms: u32,
) -> Result<usize, String> {
    let mut bulk = UsbdevfsBulktransfer {
        ep,
        len: buffer.len() as u32,
        timeout: timeout_ms,
        data: buffer.as_mut_ptr() as *mut std::ffi::c_void,
    };

    let ret = unsafe { libc::ioctl(fd, USBDEVFS_BULK, &mut bulk as *mut _) };
    if ret < 0 {
        Err(format!("ioctl USBDEVFS_BULK failed: {}", std::io::Error::last_os_error()))
    } else {
        Ok(ret as usize)
    }
}

fn usb_claim_interface(fd: RawFd, interface_number: u32) -> Result<(), String> {
    let mut iface = interface_number;
    let ret = unsafe { libc::ioctl(fd, USBDEVFS_CLAIMINTERFACE, &mut iface as *mut _) };
    if ret < 0 {
        Err(format!("ioctl USBDEVFS_CLAIMINTERFACE failed: {}", std::io::Error::last_os_error()))
    } else {
        Ok(())
    }
}

fn send_string(fd: RawFd, index: u16, text: &str) -> Result<(), String> {
    let mut bytes = text.as_bytes().to_vec();
    bytes.push(0); // NUL-terminated
    usb_control_transfer(
        fd,
        USB_DIR_OUT | USB_TYPE_VENDOR,
        ACCESSORY_SEND_STRING,
        0,
        index,
        &mut bytes,
        1000,
    )?;
    thread::sleep(Duration::from_millis(10));
    Ok(())
}

fn check_device(path: &Path) -> Option<()> {
    let mut file = match OpenOptions::new().read(true).write(true).open(path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut desc = vec![0u8; 18];
    if file.read_exact(&mut desc).is_err() {
        return None;
    }

    if desc[0] != 18 || desc[1] != USB_DT_DEVICE {
        return None;
    }

    let vendor_id = u16::from_le_bytes([desc[8], desc[9]]);
    let product_id = u16::from_le_bytes([desc[10], desc[11]]);

    let fd = file.as_raw_fd();

    if vendor_id == 0x18D1 && (product_id == 0x2D00 || product_id == 0x2D01) {
        println!("Found Android device in accessory mode ({:04x}:{:04x}) at {:?}", vendor_id, product_id, path);

        // Read remaining descriptor data
        let mut rest = Vec::new();
        let _ = file.read_to_end(&mut rest);

        let mut intf_num: Option<u8> = None;
        let mut ep_in: Option<u8> = None;
        let mut ep_out: Option<u8> = None;

        let mut offset = 0;
        while offset + 2 <= rest.len() {
            let len = rest[offset] as usize;
            if len == 0 || offset + len > rest.len() {
                break;
            }
            let dtype = rest[offset + 1];
            if dtype == USB_DT_INTERFACE && len >= 9 {
                intf_num = Some(rest[offset + 2]);
            } else if dtype == USB_DT_ENDPOINT && len >= 7 {
                let ep_addr = rest[offset + 2];
                if (ep_addr & USB_ENDPOINT_DIR_MASK) == USB_DIR_IN {
                    ep_in = Some(ep_addr);
                } else {
                    ep_out = Some(ep_addr);
                }
            }
            offset += len;
        }

        let intf = intf_num.unwrap_or(0);
        let ep_in_addr = ep_in.unwrap_or(0x81);
        let ep_out_addr = ep_out.unwrap_or(0x01);

        if let Err(e) = usb_claim_interface(fd, intf as u32) {
            eprintln!("Failed to claim interface {}: {}", intf, e);
            return None;
        }

        println!("Claimed interface {}. Endpoints: IN=0x{:02x}, OUT=0x{:02x}", intf, ep_in_addr, ep_out_addr);

        let running = Arc::new(AtomicBool::new(true));
        let mut num = 0;
        let mut buffer = [0u8; 16384];

        while running.load(Ordering::SeqCst) {
            match usb_bulk_transfer(fd, ep_in_addr as u32, &mut buffer, 1000) {
                Ok(n) if n > 0 => {
                    let recv_slice = &buffer[..n];
                    print!("[RECV] ");
                    let _ = std::io::stdout().write_all(recv_slice);
                    println!();

                    let msg = format!("Message from Android accessory #{}", num);
                    num += 1;
                    println!("[SENT] {}", msg);
                    let mut msg_bytes = msg.into_bytes();
                    let _ = usb_bulk_transfer(fd, ep_out_addr as u32, &mut msg_bytes, 1000);
                }
                Ok(_) => {}
                Err(_) => {
                    // Timeout or disconnected
                }
            }
        }
    } else {
        // Attempt to switch to accessory mode
        println!("Found possible Android device ({:04x}:{:04x}) - attempting to switch to accessory mode...", vendor_id, product_id);
        let mut proto_buf = [0u8; 2];
        if let Ok(n) = usb_control_transfer(
            fd,
            USB_DIR_IN | USB_TYPE_VENDOR,
            ACCESSORY_GET_PROTOCOL,
            0,
            0,
            &mut proto_buf,
            1000,
        ) {
            if n == 2 {
                let proto = u16::from_le_bytes(proto_buf);
                println!("Device supports protocol version {}", proto);
            }
        }

        let _ = send_string(fd, ACCESSORY_STRING_MANUFACTURER, "Android CTS");
        let _ = send_string(fd, ACCESSORY_STRING_MODEL, "CTS USB Accessory");
        let _ = send_string(fd, ACCESSORY_STRING_DESCRIPTION, "CTS USB Accessory");
        let _ = send_string(fd, ACCESSORY_STRING_VERSION, "1.0");
        let _ = send_string(fd, ACCESSORY_STRING_URI, "http://source.android.com/compatibility/cts-intro.html");
        let _ = send_string(fd, ACCESSORY_STRING_SERIAL, "1234567890");

        let mut dummy = [];
        let _ = usb_control_transfer(
            fd,
            USB_DIR_OUT | USB_TYPE_VENDOR,
            ACCESSORY_START,
            0,
            0,
            &mut dummy,
            1000,
        );
    }

    Some(())
}

fn scan_usb_devices() -> Vec<PathBuf> {
    let mut devices = Vec::new();
    let usb_root = Path::new("/dev/bus/usb");
    if let Ok(busses) = fs::read_dir(usb_root) {
        for bus in busses.flatten() {
            if let Ok(devs) = fs::read_dir(bus.path()) {
                for dev in devs.flatten() {
                    devices.push(dev.path());
                }
            }
        }
    }
    devices
}

fn main() {
    println!("CTS USB Accessory Tester (Rust 2024)");

    let devices = scan_usb_devices();
    if devices.is_empty() {
        println!("No USB devices found under /dev/bus/usb.");
        return;
    }

    println!("Scanning {} USB device(s)...", devices.len());
    for dev in devices {
        check_device(&dev);
    }
}
