use std::path::PathBuf;

use crate::{Os, dnnl::build_dnnl};

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
        build_dnnl(!shared);
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
