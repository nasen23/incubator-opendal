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

use std::cell::RefCell;

use opendal::Error;

#[repr(C)]
pub enum od_code {
    /// OpenDAL don't know what happened here, and no actions other than just
    /// returning it back. For example, s3 returns an internal service error.
    ODE_UNEXPECTED = 1,
    /// Underlying service doesn't support this operation.
    ODE_UNSUPPORTED,

    /// The config for backend is invalid.
    ODE_CONFIG_INVALID,
    /// The given path is not found.
    ODE_NOT_FOUND,
    /// The given path doesn't have enough permission for this operation
    ODE_PERMISSION_DENIED,
    /// The given path is a directory.
    ODE_IS_A_DIRECTORY,
    /// The given path is not a directory.
    ODE_NOT_A_DIRECTORY,
    /// The given path already exists thus we failed to the specified operation on it.
    ODE_ALREADY_EXISTS,
    /// Requests that sent to this path is over the limit, please slow down.
    ODE_RATE_LIMITED,
}

impl From<opendal::ErrorKind> for od_code {
    fn from(v: opendal::ErrorKind) -> Self {
        use opendal::ErrorKind::*;
        use od_code::*;

        match v {
            Unexpected => ODE_UNEXPECTED,
            Unsupported => ODE_UNSUPPORTED,
            ConfigInvalid => ODE_CONFIG_INVALID,
            NotFound => ODE_NOT_FOUND,
            PermissionDenied => ODE_PERMISSION_DENIED,
            IsADirectory => ODE_IS_A_DIRECTORY,
            NotADirectory => ODE_NOT_A_DIRECTORY,
            AlreadyExists => ODE_ALREADY_EXISTS,
            RateLimited => ODE_RATE_LIMITED,
            _ => unimplemented!("unsupported error kind"),
        }
    }
}

thread_local! {
    /// `errno` style error handling.
    static ERROR: RefCell<Option<Error>> = RefCell::new(None);
}

pub(crate) fn set_last_error(error: Error) {
    ERROR.with(|prev| {
        prev.replace(Some(error));
    })
}

#[no_mangle]
pub fn od_err_code() -> Option<od_code> {
    ERROR.with(|err| err.borrow().as_ref().map(|e| e.kind().into()))
}

