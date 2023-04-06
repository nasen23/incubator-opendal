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

mod error;

use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::CStr;
use std::ffi::c_uchar;
use std::str::FromStr;

use opendal::services::Memory;
use opendal::ErrorKind;
use opendal::Operator;
use opendal::Result;
use opendal::Scheme;


#[repr(C)]
pub struct od_buf(Vec<u8>);

macro_rules! ffi_try {
    ($ex:expr) => {
        match $ex {
            Ok(v) => v,
            Err(err) => {
                crate::error::set_last_error(err);
                return -1;
            }
        }
    };
}

fn build_operator(scheme: Scheme, options: Vec<(String, String)>) -> Result<Operator> {
    use opendal::services::*;

    let op = match scheme {
        opendal::Scheme::Azblob => {
            opendal::Operator::from_iter::<Azblob>(options.into_iter())?.finish()
        }
        opendal::Scheme::Azdfs => {
            opendal::Operator::from_iter::<Azdfs>(options.into_iter())?.finish()
        }
        opendal::Scheme::Fs => opendal::Operator::from_iter::<Fs>(options.into_iter())?.finish(),
        opendal::Scheme::Gcs => opendal::Operator::from_iter::<Gcs>(options.into_iter())?.finish(),
        opendal::Scheme::Ghac => {
            opendal::Operator::from_iter::<Ghac>(options.into_iter())?.finish()
        }
        opendal::Scheme::Http => {
            opendal::Operator::from_iter::<Http>(options.into_iter())?.finish()
        }
        opendal::Scheme::Ipmfs => {
            opendal::Operator::from_iter::<Ipmfs>(options.into_iter())?.finish()
        }
        opendal::Scheme::Memory => {
            opendal::Operator::from_iter::<Memory>(options.into_iter())?.finish()
        }
        opendal::Scheme::Obs => opendal::Operator::from_iter::<Obs>(options.into_iter())?.finish(),
        opendal::Scheme::Oss => opendal::Operator::from_iter::<Oss>(options.into_iter())?.finish(),
        opendal::Scheme::S3 => opendal::Operator::from_iter::<S3>(options.into_iter())?.finish(),
        opendal::Scheme::Webdav => {
            opendal::Operator::from_iter::<Webdav>(options.into_iter())?.finish()
        }
        opendal::Scheme::Webhdfs => {
            opendal::Operator::from_iter::<Webhdfs>(options.into_iter())?.finish()
        }
        _ => {
            return Err(opendal::Error::new(
                opendal::ErrorKind::Unexpected,
                "not supported scheme",
            ))
        }
    };

    Ok(op)
}

unsafe fn parse_options(
    options: *const *const *const i8,
    options_len: isize,
) -> Vec<(String, String)> {
    (0..options_len)
        .map(|i| unsafe {
            let tuple = *options.offset(i);
            (
                CStr::from_ptr(*tuple).to_str().unwrap().into(),
                CStr::from_ptr(*tuple.offset(1)).to_str().unwrap().into(),
            )
        })
        .collect()
}

/// Hello, OpenDAL!
// #[no_mangle]
// pub extern "C" fn hello_opendal() {
//     let op = Operator::new(Memory::default()).unwrap().finish();
//     println!("{op:?}")
// }

/// `opendal_operator` is the entry for all public blocking apis.
pub struct od_operator(opendal::BlockingOperator);

/// Create a new blocking `opendal_operator` with the given `scheme` and options.
#[no_mangle]
pub extern "C" fn od_operator_new(
    operator: *mut *mut od_operator,
    scheme: *const c_char,
    options: *const *const *const c_char,
    options_len: isize,
) -> c_int {
    let scheme = unsafe { CStr::from_ptr(scheme).to_str().expect("invalid str") };
    let scheme = Scheme::from_str(scheme).expect("invalid scheme");
    let options = unsafe { parse_options(options, options_len) };
    let op = od_operator(ffi_try!(build_operator(scheme, options)).blocking());
    unsafe { operator.write(Box::into_raw(Box::new(op))) }
    0
}

/// Free a previously created operator.
#[no_mangle]
pub extern "C" fn od_operator_free(operator: *mut od_operator) {
    unsafe { drop(Box::from_raw(operator)) }
}

#[no_mangle]
pub extern "C" fn od_operator_read(
    operator: *mut od_operator,
    path: *const c_char,
    buf: *mut *mut u8,
    size: *mut usize,
) -> c_int {
    let path = unsafe { CStr::from_ptr(path).to_str().expect("invalid str") };
    let mut vec = ffi_try!(unsafe { operator.read() }.0.read(path));
    unsafe { buf.write(vec.as_mut_ptr()) };
    unsafe { size.write(vec.len()) };
    std::mem::forget(vec);
    0
}
