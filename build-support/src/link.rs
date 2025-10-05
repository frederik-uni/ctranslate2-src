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
) {
    if cuda {
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

    if openblas {
        println!("cargo:rustc-link-lib=static=openblas");
    }
    if accelarate {
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }
    if dnnl {
        build_dnnl(os != Os::Mac);
    }
    if openmp_comp {
        println!("cargo:rustc-link-lib=gomp");
    } else if openmp_intel {
        if os == Os::Win {
            println!("cargo:rustc-link-lib=dylib=libiomp5md");
        } else {
            println!("cargo:rustc-link-lib=iomp5");
        }
    }
}
