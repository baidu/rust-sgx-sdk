// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

use libc::{self, c_int, c_void, iovec, off64_t, size_t, ssize_t};
#[cfg(not(target_env="musl"))]
use libc::{c_ulong};
use std::io::Error;

#[no_mangle]
pub extern "C" fn u_read_ocall(
    error: *mut c_int,
    fd: c_int,
    buf: *mut c_void,
    count: size_t,
) -> ssize_t {
    let mut errno = 0;
    let ret = unsafe { libc::read(fd, buf, count) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_pread64_ocall(
    error: *mut c_int,
    fd: c_int,
    buf: *mut c_void,
    count: size_t,
    offset: off64_t,
) -> ssize_t {
    let mut errno = 0;
    let ret = unsafe { libc::pread64(fd, buf, count, offset) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_readv_ocall(
    error: *mut c_int,
    fd: c_int,
    iov: *const iovec,
    iovcnt: c_int,
) -> ssize_t {
    let mut errno = 0;
    let ret = unsafe { libc::readv(fd, iov, iovcnt) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_preadv64_ocall(
    error: *mut c_int,
    fd: c_int,
    iov: *const iovec,
    iovcnt: c_int,
    offset: off64_t,
) -> ssize_t {
    let mut errno = 0;
    let ret = unsafe { libc::preadv64(fd, iov, iovcnt, offset) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_write_ocall(
    error: *mut c_int,
    fd: c_int,
    buf: *const c_void,
    count: size_t,
) -> ssize_t {
    let mut errno = 0;
    let ret = unsafe { libc::write(fd, buf, count) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_pwrite64_ocall(
    error: *mut c_int,
    fd: c_int,
    buf: *const c_void,
    count: size_t,
    offset: off64_t,
) -> ssize_t {
    let mut errno = 0;
    let ret = unsafe { libc::pwrite64(fd, buf, count, offset) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_writev_ocall(
    error: *mut c_int,
    fd: c_int,
    iov: *const iovec,
    iovcnt: c_int,
) -> ssize_t {
    let mut errno = 0;
    let ret = unsafe { libc::writev(fd, iov, iovcnt) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_pwritev64_ocall(
    error: *mut c_int,
    fd: c_int,
    iov: *const iovec,
    iovcnt: c_int,
    offset: off64_t,
) -> ssize_t {
    let mut errno = 0;
    let ret = unsafe { libc::pwritev64(fd, iov, iovcnt, offset) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_fcntl_arg0_ocall(error: *mut c_int, fd: c_int, cmd: c_int) -> c_int {
    let mut errno = 0;
    let ret = unsafe { libc::fcntl(fd, cmd) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_fcntl_arg1_ocall(
    error: *mut c_int,
    fd: c_int,
    cmd: c_int,
    arg: c_int,
) -> c_int {
    let mut errno = 0;
    let ret = unsafe { libc::fcntl(fd, cmd, arg) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[cfg(all(target_env="musl", not(target_env="")))]
#[no_mangle]
pub extern "C" fn u_ioctl_arg0_ocall(error: * mut c_int,
                                     fd: c_int,
                                     request: c_int) -> c_int {
    let mut errno = 0;
    let ret = unsafe { libc::ioctl(fd, request) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe { *error = errno; }
    }
    ret
}

#[cfg(not(target_env="musl"))]
#[no_mangle]
pub extern "C" fn u_ioctl_arg0_ocall(error: *mut c_int, fd: c_int, request: c_int) -> c_int {
    let mut errno = 0;
    let ret = unsafe { libc::ioctl(fd, request as c_ulong) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[cfg(all(target_env="musl", not(target_env="")))]
#[no_mangle]
pub extern "C" fn u_ioctl_arg1_ocall(error: * mut c_int,
                                     fd: c_int,
                                     request: c_int,
                                     arg: * const c_int) -> c_int {
    let mut errno = 0;
    let ret = unsafe { libc::ioctl(fd, request, arg) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe { *error = errno; }
    }
    ret
}

#[cfg(not(target_env="musl"))]
#[no_mangle]
pub extern "C" fn u_ioctl_arg1_ocall(
    error: *mut c_int,
    fd: c_int,
    request: c_int,
    arg: *mut c_int,
) -> c_int {
    let mut errno = 0;
    let ret = unsafe { libc::ioctl(fd, request as c_ulong, arg) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}

#[no_mangle]
pub extern "C" fn u_close_ocall(error: *mut c_int, fd: c_int) -> c_int {
    let mut errno = 0;
    let ret = unsafe { libc::close(fd) };
    if ret < 0 {
        errno = Error::last_os_error().raw_os_error().unwrap_or(0);
    }
    if !error.is_null() {
        unsafe {
            *error = errno;
        }
    }
    ret
}
