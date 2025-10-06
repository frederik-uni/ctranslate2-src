use std::{
    env,
    fs::read_dir,
    path::{Path, PathBuf},
};

use crate::{
    Os,
    dnnl::build_dnnl,
    download::download_helper,
    export,
    file_changes::watch_dir_recursively,
    link_dynamic_libraries, link_libraries,
    native::{build_native, cuda_root},
    submodules,
    windows_crt_patch::patch_cmake_runtime_flags,
};

pub fn link(
    os: Os,
    cuda: bool,
    cudnn: bool,
    cuda_dynamic_loading: bool,
    openblas: bool,
    dnnl: bool,
    accelarate: bool,
    openmp_comp: bool,
    openmp_intel: bool,
    cuda_root: Option<PathBuf>,
    shared: bool,
) {
    if cuda && !shared {
        if let Some(cuda) = cuda_root {
            println!("cargo:rustc-link-search={}", cuda.join("lib").display());
            println!("cargo:rustc-link-search={}", cuda.join("lib64").display());
            println!("cargo:rustc-link-search={}", cuda.join("lib/x64").display());
        }

        println!("cargo:rustc-link-lib=static=cudart_static");
        if cudnn {
            println!("cargo:rustc-link-lib=cudnn");
        }
        if !cuda_dynamic_loading {
            if os == Os::Win {
                println!("cargo:rustc-link-lib=static=cublas");
                println!("cargo:rustc-link-lib=static=cublasLt");
            } else {
                println!("cargo:rustc-link-lib=static=cublas_static");
                println!("cargo:rustc-link-lib=static=cublasLt_static");
                println!("cargo:rustc-link-lib=static=culibos");
            }
        }
    }

    if openblas && !shared {
        println!("cargo:rustc-link-lib=static=openblas");
    }
    if accelarate {
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
    if dnnl {
        // build_dnnl(!shared);
    }
    if openmp_comp && !shared {
        println!("cargo:rustc-link-lib=gomp");
    } else if openmp_intel && !shared {
        if os == Os::Win {
            println!("cargo:rustc-link-lib=dylib=libiomp5md");
        } else {
            println!("cargo:rustc-link-lib=iomp5");
        }
    }
}

#[cfg(not(target_os = "windows"))]
const PATH_SEPARATOR: char = ':';

#[cfg(target_os = "windows")]
const PATH_SEPARATOR: char = ';';

fn add_search_paths(key: &str) {
    println!("cargo:rerun-if-env-changed={}", key);
    if let Ok(library_path) = env::var(key) {
        library_path
            .split(PATH_SEPARATOR)
            .filter(|v| !v.is_empty())
            .for_each(|v| {
                println!("cargo:rustc-link-search={}", v);
            });
    }
}

fn get_download_link(
    os: Os,
    version: &str,
    aarch64: bool,
    shared: bool,
    crt_dyn: bool,
) -> Option<String> {
    Some(format!(
        "https://github.com/frederik-uni/ctranslate2-src/releases/download/v{version}/ctranslate2-{}{}-{}-{}.tar.gz",
        if shared { "shared" } else { "static" },
        if crt_dyn && os == Os::Win { "-crt" } else { "" },
        match os {
            Os::Win => "windows",
            Os::Mac => "macos",
            Os::Linux => "linux",
            Os::Unknown => return None,
        },
        match aarch64 {
            true => "arm64",
            false => "x86_64",
        }
    ))
}

fn get_dir() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn link_vendor(os: Os, aarch64: bool, shared: bool) {
    match (os, aarch64) {
        (Os::Win, false) => {
            link(
                os, true, true, true, false, true, false, false, true, None, shared,
            );
        }
        (Os::Mac, true) => {
            link(
                os, false, false, false, false, false, true, false, false, None, shared,
            );
        }
        (Os::Linux, true) => {
            link(
                os, false, false, false, true, false, false, true, false, None, shared,
            );
        }
        (Os::Mac, false) => {
            link(
                os, false, false, false, false, true, false, false, true, None, shared,
            );
        }
        (Os::Linux, false) => {
            link(
                os, true, true, true, false, false, false, true, false, None, shared,
            );
        }
        _ => panic!("Unsupported platform"),
    }
}

fn load_vendor(os: Os, aarch64: bool, shared: bool, crt_dynamic: bool) -> Option<PathBuf> {
    let main_dir = get_dir();
    let out_dir = main_dir.join("ctranslate2-vendor");

    let dyn_dir = out_dir.join("dyn");
    let url = get_download_link(os, "4.6.0", aarch64, shared, crt_dynamic)?;
    download_helper(&url, &out_dir, true)?;

    watch_dir_recursively(&dyn_dir);

    let files = dyn_dir
        .read_dir()
        .map(|v| v.into_iter().filter_map(|v| v.ok()).collect::<Vec<_>>())
        .unwrap_or_default()
        .iter()
        .map(|v| v.path())
        .filter(|p| {
            let ext = p
                .extension()
                .and_then(|v| v.to_str())
                .unwrap_or_default()
                .to_lowercase();
            ext == "dll" || ext == "so" || ext == "dylib"
        })
        .collect::<Vec<_>>();
    println!(
        "cargo:warning=Required dylibs are in: {}",
        main_dir.display()
    );
    for file in files {
        println!("cargo:warning=- {}", file.display());
        let tar = main_dir.join(file.file_name().unwrap_or_default());
        std::fs::copy(&file, &tar).unwrap();
        // Github actions has sometimes some issues with finding files. I hope that fixes it
        println!("cargo:rerun-if-changed={}", tar.display());
    }

    println!("cargo:rustc-link-search=native={}", dyn_dir.display());
    Some(out_dir.join("lib"))
}

pub fn main(
    (
        os,
        aarch64,
        cuda,
        cudnn,
        cuda_dynamic_loading,
        mkl,
        openblas,
        ruy,
        accelarate,
        tensor_parallel,
        dnnl,
        openmp_comp,
        openmp_intel,
        msse4_1,
        flash_attention,
        cuda_small_binary,
        shared,
        vendor,
        crt_dynamic,
        export_vendor,
        path,
    ): (
        Os,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        Option<&Path>,
    ),
) -> PathBuf {
    add_search_paths("LIBRARY_PATH");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/sys");
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=CTranslate2");

    let mut found = None;

    if vendor {
        link_vendor(os, aarch64, shared);
        found = load_vendor(os, aarch64, shared, crt_dynamic);
    }
    let (lib_path, include_path) = if let Some(found) = found {
        (found.clone(), found.join("include"))
    } else {
        add_search_paths("CMAKE_LIBRARY_PATH");
        link(
            os,
            cuda,
            cudnn,
            cuda_dynamic_loading,
            openblas,
            dnnl,
            accelarate,
            openmp_comp,
            openmp_intel,
            Some(cuda_root()).expect("CUDA_TOOLKIT_ROOT_DIR is not specified"),
            shared,
        );
        let p = if let Some(path) = path {
            path.to_path_buf()
        } else {
            let release =
                std::env::var("CTRANSLATE2_RELEASE").unwrap_or_else(|_| "4.6.0".to_owned());
            let url = format!(
                "https://github.com/OpenNMT/CTranslate2/archive/refs/tags/v{release}.tar.gz"
            );

            let p = format!("CTranslate2-{release}");
            let p = get_dir().join(Path::new(&p));
            let d = &get_dir();
            if !p.exists() {
                download_helper(&url, d, false).unwrap();
            }
            for module in submodules::get_submodules_helper(d, &release) {
                if !module.exists()
                    || read_dir(module)
                        .unwrap()
                        .into_iter()
                        .filter_map(|v| v.ok())
                        .count()
                        < 2
                {
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }
            }
            if !p.exists() {
                panic!("CTranslate2-{release} not found locally")
            }
            if os == Os::Win {
                patch_cmake_runtime_flags(p.join("CMakeLists.txt"), crt_dynamic).unwrap();
            }
            p
        };

        (
            build_native(
                &p,
                os,
                cuda,
                cudnn,
                cuda_dynamic_loading,
                aarch64,
                mkl,
                openblas,
                ruy,
                accelarate,
                tensor_parallel,
                msse4_1,
                dnnl,
                openmp_comp,
                openmp_intel,
                flash_attention,
                cuda_small_binary,
                shared,
            ),
            p.join("include"),
        )
    };

    let modules = link_libraries(&lib_path);
    let modules2 = link_dynamic_libraries(&lib_path);

    if export_vendor {
        export(&lib_path, &modules, &modules2);
    }
    include_path
}
