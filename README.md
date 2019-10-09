# Rust bindings for ProtectedFS

[![Build Status](http://ci.mesalock-linux.org/api/badges/mesalock-linux/protected_fs_rs/status.svg)](http://ci.mesalock-linux.org/mesalock-linux/protected_fs_rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

`protected_fs_rs` is a rust binding for [protected_fs](https://github.com/intel/linux-sgx/tree/master/sdk/protected_fs) in Intel SGX Linux SDK. Most of the rust code is borrowed from [sgx_tprotected_fs](https://github.com/baidu/rust-sgx-sdk/tree/master/sgx_tprotected_fs) in Rust SGX SDK.

Beyond the original SGX-only implementations, `protected_fs_rs` now supports ***running in both SGX and Non-SGX environment***. We ported the [original C implementations](https://github.com/intel/linux-sgx/tree/master/sdk/protected_fs) in  [protected_fs_c](https://github.com/mesalock-linux/protected_fs_rs/tree/master/protected_fs_c) subdirectory and replaced the compile toolchains with CMake. Please refer to [build.rs](https://github.com/mesalock-linux/protected_fs_rs/blob/master/build.rs) for more information about building the C code.