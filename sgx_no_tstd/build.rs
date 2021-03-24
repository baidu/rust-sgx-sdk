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
// under the License.

extern crate sgx_build_helper as build_helper;

use std::env;
use std::process::Command;
use build_helper::{run, native_lib_boilerplate};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target = env::var("TARGET").expect("TARGET was not set");
    let host = env::var("HOST").expect("HOST was not set");

    let _ = build_libunwind(&host, &target);
}

fn build_libunwind(host: &str, target: &str) -> Result<(), ()> {
    let filter = vec![
        "m4",
        "config",
        "autom4te.cache",
        "Makefile.in",
        "config.h.in",
        "configure",
        "aclocal.m4",
        "INSTALL"];
    let native = native_lib_boilerplate(
                    "sgx_unwind/libunwind",
                    "libunwind",
                    "unwind",
                    "src/.libs",
                    &filter)?;

    let mut cflags = String::new();
    cflags += " -fstack-protector -ffreestanding -nostdinc -fvisibility=hidden -fpie -fno-strict-overflow -fno-delete-null-pointer-checks";
    cflags += " -O2";

    let mitigation_cflags = " -mindirect-branch-register -mfunction-return=thunk-extern";
    let mitigation_asflags = " -fno-plt";
    let mitigation_loadflags = " -Wa,-mlfence-after-load=yes -Wa,-mlfence-before-ret=not";
    let mitigation_cfflags = " -Wa,-mlfence-before-indirect-branch=register -Wa,-mlfence-before-ret=not";
    let mitigation = env::var("MITIGATION_CVE_2020_0551").unwrap_or_default();
    match mitigation.as_ref() {
        "LOAD" => {
            cflags += mitigation_cflags;
            cflags += mitigation_asflags;
            cflags += mitigation_loadflags;
        },
        "CF" => {
            cflags += mitigation_cflags;
            cflags += mitigation_asflags;
            cflags += mitigation_cfflags;
        },
        _  => {},
    }

    run(Command::new("sh")
                .current_dir(&native.out_dir)
                .arg(native.src_dir.join("autogen.sh").to_str().unwrap())
                .arg(format!("--host={}", build_helper::gnu_target(target)))
                .arg(format!("--build={}", build_helper::gnu_target(host)))
                .env("CFLAGS", cflags));

    run(Command::new(build_helper::make(host))
                .current_dir(&native.out_dir)
                .arg(format!("INCDIR={}", native.src_dir.display()))
                .arg("-j5"));
    Ok(())
}