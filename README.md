# env variables
- CTRANSLATE2_RELEASE = [default = "4.6.0"]

# features
- `vendor`: Use prebuilt binaries
- `shared`: Build with ctranslate2 as shared library
- `crt-dynamic`: crt is statically linked on Windows-static builds. to link crt dynamically, use `crt-dynamic`

# native features
These features only do something if `vendor` is not used

- cuda
  - cudnn
  - cuda-dynamic-loading
  - cuda-small-binary
- mkl
- openblas
- ruy
- accelerate
- tensor-parallel
- dnnl
- openmp-runtime-comp
- openmp-runtime-intel
- msse4_1
- os-defaults
- flash-attention

# Prebuilt binaries + used features
- macos x86_64[openmp_intel, dnnl, mkl]
- macos aarch64[accelerate, ruy]
- linux x86_64[openmp_comp, cuda, cudnn, cuda_small_binary, cuda-dynamic-loading, dnnl, mkl, tensor_parallel, msse4_1]
- linux aarch64[openmp_comp, ruy, openblas]
- windows x86_64[openmp_intel, cuda, cudnn, cuda_small_binary, cuda-dynamic-loading, dnnl, mkl]
