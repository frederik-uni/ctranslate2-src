# CTranslate2-SRC

## Prerequisites

The installation of [CMake](https://cmake.org/) is required to compile the library.

## Additional notes for Windows

Setting the environment variable `RUSTFLAGS=-C target-feature=+crt-static` might be required.

## env variables
- CTRANSLATE2_RELEASE = [default = "4.6.0"]
- CUDA_TOOLKIT_ROOT_DIR

## features
- `vendor`: Use prebuilt binaries
- `shared`: Build with ctranslate2 as shared library
- `crt-dynamic`: crt is statically linked on Windows-static builds. to link crt dynamically, use `crt-dynamic`

## native features
These features only do something if `vendor` is not used

- `cuda`: Enables CUDA support
  - `cudnn`: Enables cuDNN support
  - `cuda-dynamic-loading`: Enables dynamic loading of CUDA libraries at runtime instead of static linking (requires
    CUDA >= 11)
    - `cuda-small-binary`: Reduces binary size by compressing device code
- `mkl`: Enables [Intel MKL](https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html) support
- `openblas`: Enables [OpenBLAS](https://www.openblas.net/) support (OpenBLAS needs to be installed manually
  via [vcpkg](https://vcpkg.io) on Windows)
- `ruy`: Enables [Ruy](https://github.com/google/ruy) support
- `accelerate`: Enables [Apple Accelerate](https://developer.apple.com/documentation/accelerate) support (macOS only)
- `dnnl`: Enables [oneDNN](https://www.intel.com/content/www/us/en/developer/tools/oneapi/onednn.html) support
- `openmp-runtime-comp`: Enables OpenMP runtime support
- `openmp-runtime-intel`: Enables OpenMP runtime support for Intel compilers
- `msse4_1`: Enables MSSE4.1 support
- os-defaults
- `tensor-parallel`:
  Enables [Tensor Parallelism](https://huggingface.co/docs/text-generation-inference/conceptual/tensor_parallelism)
  - `flash-attention`:
    Enables [Flash Attention](https://huggingface.co/docs/text-generation-inference/conceptual/flash_attention)

## Prebuilt binaries + used features
- macos static x86_64[openmp_intel, dnnl, mkl]
- macos static aarch64[accelerate, ruy]
- linux static x86_64[openmp_comp, cuda, cudnn, cuda_small_binary, cuda-dynamic-loading, dnnl, mkl, tensor_parallel, msse4_1]
- linux static aarch64[openmp_comp, ruy, openblas]
- windows static x86_64[openmp_intel, cuda, cudnn, cuda_small_binary, cuda-dynamic-loading, dnnl, mkl]
- windows static dynamic-crt x86_64[openmp_intel, cuda, cudnn, cuda_small_binary, cuda-dynamic-loading, dnnl, mkl]
- macos shared aarch64[accelerate, ruy]
- linux shared x86_64[openmp_comp, cuda, cudnn, cuda_small_binary, cuda-dynamic-loading, dnnl, mkl, tensor_parallel, msse4_1]
- linux shared aarch64[openmp_comp, ruy, openblas]
- windows shared x86_64[openmp_intel, cuda, cudnn, cuda_small_binary, cuda-dynamic-loading, dnnl, mkl]
