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

//! Get the metadata of the current enclave.
//!
//! This mod has clear interface and is easy to understand. Currently we don't
//! have time for its documents.

use sgx_types::metadata::*;
use sgx_types::*;

pub const LAYOUT_ENTRY_NUM: usize = 42;

#[link(name = "sgx_trts")]
extern "C" {
    static g_global_data: global_data_t;
    static g_cpu_feature_indicator: uint64_t;
    static g_cpu_core_num: uint32_t;
    static EDMM_supported: c_int;
    static g_peak_heap_used: size_t;
    static g_peak_rsrv_mem_committed: size_t;
    pub fn get_thread_data() -> *const c_void;
    pub fn get_enclave_base() -> *const c_void;
    pub fn get_heap_base() -> *const c_void;
    pub fn get_heap_size() -> size_t;
    pub fn get_rsrv_base() -> *const c_void;
    pub fn get_rsrv_size() -> size_t;
}

#[repr(C)]
pub struct global_data_t {
    pub sdk_version: usize,
    pub enclave_size: usize,
    pub heap_offset: usize,
    pub heap_size: usize,
    pub rsrv_offset: usize,
    pub rsrv_size: usize,
    pub rsrv_executable: usize,
    pub thread_policy: usize,
    pub tcs_max_num: usize,
    pub td_template: thread_data_t,
    pub tcs_template: [u8; TCS_TEMPLATE_SIZE],
    pub layout_entry_num: u32,
    pub reserved: u32,
    pub layout_table: [layout_t; LAYOUT_ENTRY_NUM],
    pub enclave_image_address: u64,
    pub elrange_start_address: u64,
    pub elrange_size: u64,
}

#[repr(C)]
pub struct thread_data_t {
    pub self_addr: usize,
    pub last_sp: usize,
    pub stack_base_addr: usize,
    pub stack_limit_addr: usize,
    pub first_ssa_gpr: usize,
    pub stack_guard: usize,
    pub flags: usize,
    pub xsave_size: usize,
    pub last_error: usize,
    pub m_next: usize,
    pub tls_addr: usize,
    pub tls_array: usize,
    pub exception_flag: usize,
    pub cxx_thread_info: [usize; 6],
    pub stack_commit_addr: usize,
}

#[derive(Copy, Clone)]
pub struct SgxGlobalData {
    enclave_base: usize,
    enclave_size: usize,
    heap_base: usize,
    heap_offset: usize,
    heap_size: usize,
    rsrv_base: usize,
    rsrv_offset: usize,
    rsrv_size: usize,
    thread_policy: SgxThreadPolicy,
    static_tcs_num: u32, // minpool thread + utility thread
    eremove_tcs_num: u32,
    dyn_tcs_num: u32,
    max_tcs_num: u32,
    elrange_base: usize,
    elrange_size: usize,
}

impl Default for SgxGlobalData {
    fn default() -> Self {
        Self::new()
    }
}

impl SgxGlobalData {
    ///
    /// get global_data.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn new() -> Self {
        let (static_num, eremove_num, dyn_num) = rsgx_get_tcs_num();
        SgxGlobalData {
            enclave_base: rsgx_get_enclave_base() as usize,
            enclave_size: rsgx_get_enclave_size(),
            heap_base: rsgx_get_heap_base() as usize,
            heap_offset: rsgx_get_heap_offset(),
            heap_size: rsgx_get_heap_size(),
            rsrv_base: rsgx_get_rsrv_base() as usize,
            rsrv_offset: rsgx_get_rsrv_offset(),
            rsrv_size: rsgx_get_rsrv_size(),
            thread_policy: rsgx_get_thread_policy(),
            static_tcs_num: static_num,
            eremove_tcs_num: eremove_num,
            dyn_tcs_num: dyn_num,
            max_tcs_num: rsgx_get_tcs_max_num(),
            elrange_base: rsgx_get_elrange_base() as usize,
            elrange_size: rsgx_get_elrange_size(),
        }
    }

    ///
    /// enclave_base is to get enclave map base address.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn enclave_base(&self) -> usize {
        self.enclave_base
    }
    ///
    /// enclave_size is to get enclave map size.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn enclave_size(&self) -> usize {
        self.enclave_size
    }
    ///
    /// heap_base is to get heap base address.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn heap_base(&self) -> usize {
        self.heap_base
    }
    ///
    /// heap_offset is to get heap offset.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn heap_offset(&self) -> usize {
        self.heap_offset
    }
    ///
    /// heap_size is to get heap size.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn heap_size(&self) -> usize {
        self.heap_size
    }
    ///
    /// rsrv_base is to get rsrv base address.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn rsrv_base(&self) -> usize {
        self.rsrv_base
    }
    ///
    /// rsrv_offset is to get heap offset.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn rsrv_offset(&self) -> usize {
        self.rsrv_offset
    }
    ///
    /// rsrv_size is to get heap size.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn rsrv_size(&self) -> usize {
        self.rsrv_size
    }
    ///
    /// thread_policy is to get TCS policy.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn thread_policy(&self) -> SgxThreadPolicy {
        self.thread_policy
    }

    pub fn get_static_tcs_num(&self) -> u32 {
        self.static_tcs_num
    }

    pub fn get_eremove_tcs_num(&self) -> u32 {
        self.eremove_tcs_num
    }

    pub fn get_dyn_tcs_num(&self) -> u32 {
        self.dyn_tcs_num
    }

    pub fn get_tcs_max_num(&self) -> u32 {
        self.max_tcs_num
        //if rsgx_is_supported_EDMM() {
        //    if self.dyn_tcs_num != 0 {
        //        self.static_tcs_num + self.dyn_tcs_num - 1 // - 1 is utility thread
        //    } else {
        //        self.static_tcs_num
        //    }
        //} else {
        //    self.static_tcs_num + self.eremove_tcs_num
        //}
    }

    ///
    /// elrange_base is to get enclave range base address.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn elrange_base(&self) -> usize {
        self.elrange_base
    }

    ///
    /// elrange_size is to get enclave range size.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn elrange_size(&self) -> usize {
        self.elrange_size
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Default)]
pub struct SgxThreadData {
    td_addr: usize,
    last_sp: usize,
    stack_base_addr: usize,
    stack_limit_addr: usize,
    first_ssa_gpr: usize,
    stack_guard: usize,
    xsave_size: usize,
    last_error: usize,
    tls_addr: usize,
    tls_array: usize,
    exception_flag: usize,
    cxx_thread_info: [usize; 6],
}

impl SgxThreadData {
    ///
    /// get thread_data per thread.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    #[allow(clippy::cast_ptr_alignment)]
    pub fn current() -> Self {
        let td = unsafe {
            let p = rsgx_get_thread_data();
            &*p
        };
        SgxThreadData {
            td_addr: td.self_addr,
            last_sp: td.last_sp,
            stack_base_addr: td.stack_base_addr,
            stack_limit_addr: td.stack_limit_addr,
            first_ssa_gpr: td.first_ssa_gpr,
            stack_guard: td.stack_guard,
            xsave_size: td.xsave_size,
            last_error: td.last_error,
            tls_addr: td.tls_addr,
            tls_array: td.tls_array,
            exception_flag: td.exception_flag,
            cxx_thread_info: td.cxx_thread_info,
        }
    }

    pub unsafe fn from_raw(raw: sgx_thread_t) -> Self {
        let p = raw as *const thread_data_t;
        let td = &*p;
        SgxThreadData {
            td_addr: td.self_addr,
            last_sp: td.last_sp,
            stack_base_addr: td.stack_base_addr,
            stack_limit_addr: td.stack_limit_addr,
            first_ssa_gpr: td.first_ssa_gpr,
            stack_guard: td.stack_guard,
            xsave_size: td.xsave_size,
            last_error: td.last_error,
            tls_addr: td.tls_addr,
            tls_array: td.tls_array,
            exception_flag: td.exception_flag,
            cxx_thread_info: td.cxx_thread_info,
        }
    }

    ///
    /// td_base is to get TD base address per thread.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn td_base(&self) -> usize {
        self.td_addr
    }
    ///
    /// stack_base is to get stack base address per thread.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn stack_base(&self) -> usize {
        self.stack_base_addr
    }
    ///
    /// stack_limit is to get stack limit per thread.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn stack_limit(&self) -> usize {
        self.stack_limit_addr
    }
    ///
    /// tls_base is to get tls base address per thread.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn tls_base(&self) -> usize {
        self.tls_addr
    }
    ///
    /// last_error is to get last error per thread.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn last_error(&self) -> usize {
        self.last_error
    }
    ///
    /// exception_flag is to get exception flag per thread.
    ///
    /// **Note**
    ///
    /// This API is only an experimental funtion.
    ///
    pub fn exception_flag(&self) -> usize {
        self.exception_flag
    }

    pub fn get_tcs(&self) -> usize {
        self.stack_base() + STATIC_STACK_SIZE + SE_GUARD_PAGE_SIZE
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SgxThreadPolicy {
    Bound,
    Unbound,
}

///
/// rsgx_get_thread_data is to get TD base address per thread.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_thread_data() -> *const thread_data_t {
    unsafe { get_thread_data() as *const thread_data_t }
}

///
/// rsgx_get_enclave_base is to get enclave image base address.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_enclave_base() -> *const u8 {
    unsafe { get_enclave_base() as *const u8 }
}

///
/// rsgx_get_enclave_size is to get enclave image size.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_enclave_size() -> usize {
    unsafe { g_global_data.enclave_size }
}

///
/// rsgx_get_heap_base is to get enclave heap base address.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_heap_base() -> *const u8 {
    unsafe { get_heap_base() as *const u8 }
}

///
/// rsgx_get_heap_offset is to get enclave heap offset.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_heap_offset() -> usize {
    unsafe { g_global_data.heap_offset }
}

///
/// rsgx_get_heap_size is to get enclave heap size.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_heap_size() -> usize {
    unsafe { get_heap_size() }
}

///
/// rsgx_get_rsrv_base is to get enclave rsrv base address.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_rsrv_base() -> *const u8 {
    unsafe { get_rsrv_base() as *const u8 }
}

///
/// rsgx_get_rsrv_offset is to get enclave rsrv offset.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_rsrv_offset() -> usize {
    unsafe { g_global_data.rsrv_offset }
}

///
/// rsgx_get_rsrv_size is to get enclave rsrv size.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_rsrv_size() -> usize {
    unsafe { get_rsrv_size() }
}

///
/// rsgx_get_thread_policy is to get TCS management policy.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_thread_policy() -> SgxThreadPolicy {
    unsafe {
        if g_global_data.thread_policy != 0 {
            SgxThreadPolicy::Unbound
        } else {
            SgxThreadPolicy::Bound
        }
    }
}

///
/// rsgx_get_global_data is to get global_data base address.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_global_data() -> *const global_data_t {
    unsafe { &g_global_data as *const global_data_t }
}

///
/// rsgx_get_tcs_max_num is to get max tcs number.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_tcs_max_num() -> u32 {
    unsafe { g_global_data.tcs_max_num as u32 }
}

pub fn rsgx_get_tcs_num() -> (u32, u32, u32) {
    let gd = unsafe {
        let p = rsgx_get_global_data();
        &*p
    };

    let mut static_tcs_num: u32 = 0;
    let mut eremove_tcs_num: u32 = 0;
    let mut dyn_tcs_num: u32 = 0;
    let layout_table = &gd.layout_table[0..gd.layout_entry_num as usize];
    unsafe {
        traversal_layout(
            &mut static_tcs_num,
            &mut dyn_tcs_num,
            &mut eremove_tcs_num,
            layout_table,
        );
    }

    unsafe fn traversal_layout(
        static_num: &mut u32,
        dyn_num: &mut u32,
        eremove_num: &mut u32,
        layout_table: &[layout_t],
    ) {
        for (i, layout) in layout_table.iter().enumerate() {
            if !is_group_id!(layout.group.id as u32) {
                if (layout.entry.attributes & PAGE_ATTR_EADD) != 0 {
                    if (layout.entry.content_offset != 0) && (layout.entry.si_flags == SI_FLAGS_TCS)
                    {
                        if (layout.entry.attributes & PAGE_ATTR_EREMOVE) == 0 {
                            *static_num += 1;
                        } else {
                            *eremove_num += 1;
                        }
                    }
                }
                if (layout.entry.attributes & PAGE_ATTR_POST_ADD) != 0 {
                    if layout.entry.id == LAYOUT_ID_TCS_DYN as u16 {
                        *dyn_num += 1;
                    }
                }
            } else {
                for _ in 0..layout.group.load_times {
                    traversal_layout(
                        static_num,
                        dyn_num,
                        eremove_num,
                        &layout_table[i - layout.group.entry_count as usize..i],
                    )
                }
            }
        }
    }
    (static_tcs_num, eremove_tcs_num, dyn_tcs_num)
}

#[inline]
pub fn rsgx_is_supported_EDMM() -> bool {
    // cpu support
    // sgx driver support
    // sgx sdk support ( uRTS and enclave metadata)
    // hw mode
    unsafe { EDMM_supported != 0 }
}

#[inline]
pub fn rsgx_get_cpu_feature() -> u64 {
    unsafe { g_cpu_feature_indicator }
}

#[inline]
pub fn rsgx_get_cpu_core_num() -> u32 {
    unsafe { g_cpu_core_num }
}

#[inline]
pub fn rsgx_get_peak_heap_used() -> usize {
    unsafe { g_peak_heap_used }
}

#[inline]
pub fn rsgx_get_peak_rsrv_mem_committed() -> usize {
    unsafe { g_peak_rsrv_mem_committed }
}

///
/// rsgx_get_elrange_base is to get enclave range base address.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_elrange_base() -> *const u8 {
    unsafe { g_global_data.elrange_start_address as *const u8 }
}

///
/// rsgx_get_elrange_size is to get enclave range size.
///
/// **Note**
///
/// This API is only an experimental funtion.
///
#[inline]
pub fn rsgx_get_elrange_size() -> usize {
    unsafe { g_global_data.elrange_size as usize }
}
