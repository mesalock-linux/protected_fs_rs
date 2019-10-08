// Copyright 2019 MesaTEE Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// For Xargo
#![feature(rustc_private)]

#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

// For Xargo in edition 2018
extern crate sgx_trts;
extern crate sgx_types;

mod deps;
mod protected_fs;
mod sgx_fs_inner;
mod sgx_tprotected_fs;
pub use crate::protected_fs::*;
