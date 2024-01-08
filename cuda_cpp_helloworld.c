#include <stdio.h>

#include "cuda_runtime.h"
#include "device_launch_parameters.h"

// Device code
__global__ void cuda_kernel() { printf("Hello World from GPU!\n"); }

// Host code
int main() {
  printf("Hello World from CPU!\n");

  cuda_kernel<<<1, 1>>>();
  cudaDeviceSynchronize();

  cudaDeviceReset();
  return 0;
}