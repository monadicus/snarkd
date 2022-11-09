## Build Guide

### 1. Install CUDA toolkit

The easiest way to install the CUDA toolkit on linux is to run:
- Linux:
  ```bash
  sudo apt update
  sudo apt install nvidia-cuda-toolkit
  
  nvcc --version
  ```

The alternative is to follow the instructions below:

- Windows or Linux - https://developer.nvidia.com/cuda-downloads
- macOS - https://developer.nvidia.com/nvidia-cuda-toolkit-developer-tools-mac-hosts


### 2. Build the snarkVM MSM CUDA Program

This may be necessary to ensure the compatibility of your GPU drivers.

```
  cd snarkVM/algorithms/src/msm/variable_base/blst_377_cuda
  ./build.sh
```
