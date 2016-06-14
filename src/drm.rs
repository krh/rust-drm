extern crate libc;

use std::ffi::CString;
use self::libc::{c_int, c_uint, c_ulong};
use self::libc::{open, ioctl};
use std::io::{Result, Error};
use std::slice::from_raw_parts_mut;

pub struct Device {
    fd: c_int,
    context_id: u32,
    offset: u64
}

#[derive(Debug)]
pub struct Buffer {
    handle: u32,
    size: usize,
    offset: u64,
    map: * mut u32,
    cursor: isize
}

#[repr(C)]
struct drm_i915_getparam {
    param: c_uint,
    value: *mut c_int
}

#[repr(C)]
struct drm_i915_gem_create {
    size: u64,
    handle: u32,
    pad: u32
}

#[repr(C)]
struct drm_i915_gem_context_create {
    ctx_id: u32,
    pad: u32
}

#[repr(C)]
struct drm_i915_gem_mmap {
    handle: u32,
    pad: u32,
    offset: u64,
    size: u64,
    addr_ptr: u64,
    flags: u64
}

//const I915_MMAP_WC: u64 = 0x1;

const DRM_IOCTL_I915_GETPARAM: c_ulong = 0xc0106446;
const DRM_IOCTL_I915_GEM_CREATE: c_ulong = 0xc010645b;
const DRM_IOCTL_I915_GEM_CONTEXT_CREATE: c_ulong = 0xc008646d;
const DRM_IOCTL_I915_GEM_MMAP: c_ulong = 0xc028645e;

fn align(v: u64, a: u64) -> u64 { v + (a - 1) & !(a - 1) }

impl Device {
    pub fn new() -> Result<Device> {
        let cstr = CString::new("/dev/dri/renderD128").unwrap();
        let fd: c_int = unsafe { open(cstr.as_ptr(), 0) };

        let arg = drm_i915_gem_context_create { ctx_id: 0, pad: 0 };
        let r = unsafe {
            ioctl(fd, DRM_IOCTL_I915_GEM_CONTEXT_CREATE, &arg)
        };

        if r != 0 { return Err(Error::last_os_error()) }

        Ok(Device { fd: fd, context_id: arg.ctx_id, offset: 0 })
    }

    pub fn get_param(&mut self, param : u32) -> Result<u32> {
        let mut v: c_int = 0;
        let arg = drm_i915_getparam { param: param, value: &mut v };
        let r = unsafe { ioctl(self.fd, DRM_IOCTL_I915_GETPARAM, &arg) };

        match r {
            0 => Ok(v as u32),
            _ => Err(Error::last_os_error())
        }
    }

    pub fn create_buffer(&mut self, size : u64) -> Result<Buffer> {
        let arg = drm_i915_gem_create { handle: 0, size: size, pad: 0 };

        let r = unsafe { ioctl(self.fd, DRM_IOCTL_I915_GEM_CREATE, &arg) };

        if r != 0 { return Err(Error::last_os_error()) }

        let arg = drm_i915_gem_mmap {
            handle: arg.handle,
            pad: 0,
            offset: 0,
            size: size,
            addr_ptr: 0,
            flags: 0
        };

        let r = unsafe { ioctl(self.fd, DRM_IOCTL_I915_GEM_MMAP, &arg) };

        if r != 0 { return Err(Error::last_os_error()) }

        let offset = align(self.offset, 4096);
        match r {
            0 => Ok(Buffer { handle: arg.handle, size: size as usize, offset: offset,
                             map: arg.addr_ptr as *mut u32,
                             cursor: 0 }),
            _ => Err(Error::last_os_error())
        }
    }
}

impl Buffer {
    pub fn emit(&mut self, dw: &[u32]) {
        let slice: &mut [u32] = unsafe {
            from_raw_parts_mut(self.map.offset(self.cursor), dw.len())
        };

        for n in dw {
            println!("emit dw 0x{:x}", n)
        }

        slice.clone_from_slice(&dw);
        self.cursor += 4 * dw.len() as isize;
    }

    pub fn dump(&self) {
        let slice: &mut [u32] = unsafe {
            from_raw_parts_mut(self.map.offset(self.cursor), 4)
        };

        for n in 0..4 {
            println!("dw {}: {}", n, slice[n])
        }
    }
}
