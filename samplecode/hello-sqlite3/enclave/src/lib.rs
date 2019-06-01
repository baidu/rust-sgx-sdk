// Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;

extern crate sqlite3;

use sqlite3::{
    DatabaseConnection,
    SqliteResult,
};

fn convenience_exec() -> SqliteResult<DatabaseConnection> {
    let mut conn = DatabaseConnection::in_memory()?;

    conn.exec("
       create table items (
                   id integer,
                   description varchar(40),
                   price integer
                   )")?;

    Ok(conn)
}

fn typical_usage(conn: &mut DatabaseConnection) -> SqliteResult<String> {
    {
        let mut stmt = conn.prepare(
            "insert into items (id, description, price)
           values (1, 'stuff', 10)")?;
        let mut results = stmt.execute();
        match results.step()? {
            None => (),
            Some(_) => panic!("row from insert?!"),
        };
    }
    assert_eq!(conn.changes(), 1);
    assert_eq!(conn.last_insert_rowid(), 1);
    {
        let mut stmt = conn.prepare(
            "select * from items")?;
        let mut results = stmt.execute();
        match results.step() {
            Ok(Some(ref mut row1)) => {
                let id = row1.column_int(0);
                let desc_opt = row1.column_text(1).expect("desc_opt should be non-null");
                let price = row1.column_int(2);

                assert_eq!(id, 1);
                assert_eq!(price, 10);

                Ok(format!("row: {}, {}, {}", id, desc_opt, price))
            },
            Err(oops) => panic!(oops),
            Ok(None) => panic!("where did our row go?")
        }
    }
}


#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    match convenience_exec() {
        Ok(ref mut db) => {
            match typical_usage(db) {
                Ok(txt) => println!("item: {}", txt),
                Err(oops) => {
                    panic!("error: {:?} msg: {}", oops,
                           db.errmsg())
                }
            }
        },
        Err(oops) => panic!(oops)
    }

    println!("typical");

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    sgx_status_t::SGX_SUCCESS
}