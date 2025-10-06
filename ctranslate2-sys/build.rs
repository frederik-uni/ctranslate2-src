// build.rs
//
// Copyright (c) 2023-2025 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

use std::fs::read_dir;
use std::path::PathBuf;
use std::{env, path::Path};

use ctranslate2_src_build_support::dnnl::build_dnnl;
use ctranslate2_src_build_support::download::download_helper;
use ctranslate2_src_build_support::file_changes::watch_dir_recursively;

use ctranslate2_src_build_support::native::cuda_root;
use ctranslate2_src_build_support::windows_crt_patch::patch_cmake_runtime_flags;
use ctranslate2_src_build_support::{Os, export, link_libraries, native::build_native};
use ctranslate2_src_build_support::{link_dynamic_libraries, submodules};

fn main() {
    if cfg!(feature = "export-vendor") {
        export(&lib_path, &modules, &modules2);
    }

    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .file("cpp/translator_wrapper.cpp")
        .include("include")
        .include(include_path)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .compile("translator_wrapper");

    // Bindgen
    let bindings = bindgen::Builder::default()
        .header("include/translator_wrapper.h")
        .clang_args(&["-x", "c++", "-std=c++17"])
        .blocklist_item("_LIBCPP_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("translator_bindings.rs"))
        .expect("Couldn't write bindings!");
}
