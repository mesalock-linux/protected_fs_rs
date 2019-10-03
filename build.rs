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

use cfg_if::cfg_if;

#[cfg(feature = "mesalock_sgx")]
use std::env;

use cmake;
#[cfg(not(feature = "mesalock_sgx"))]
fn build_non_sgx_protected_fs_c_with_cmake() {
    let dst = cmake::Config::new("protected_fs_c")
                            .define("NON_SGX_PROTECTED_FS", "ON")
                            .very_verbose(true)
                            .build();
    println!("cargo:rustc-link-search=native={}/bin", dst.display());
    println!("cargo:rustc-link-lib=static=tprotected_fs");
    println!("cargo:rustc-link-lib=static=uprotected_fs");
}

#[cfg(feature = "mesalock_sgx")]
fn build_sgx_protected_fs_c_with_cmake(sdk_dir: String) {
    let dst = cmake::Config::new("protected_fs_c")
                            .define("NON_SGX_PROTECTED_FS", "OFF")
                            .define("SGX_SDK", sdk_dir)
                            .very_verbose(true)
                            .build();
    println!("cargo:rustc-link-search=native={}/bin", dst.display());
    println!("cargo:rustc-link-lib=static=tprotected_fs");
}

cfg_if! {
    if #[cfg(feature = "mesalock_sgx")] {
        fn build() {
            let sdk_dir = env::var("SGX_SDK").unwrap_or("/opt/intel/sgxsdk".into());
            //println!("cargo:rustc-link-search=native={}/lib64", sdk_dir);
            build_sgx_protected_fs_c_with_cmake(sdk_dir.clone());
        }
    } else {
        fn build() {
            build_non_sgx_protected_fs_c_with_cmake();
            println!("cargo:rustc-link-lib=crypto");
            println!("cargo:rustc-link-lib=stdc++");
        }
    }
}

fn main() {
    build();
}