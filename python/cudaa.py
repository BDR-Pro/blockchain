from numba import cuda
import numpy as np
import time
import os
import sys

"""
CUDA Matrix Multiplication and Performance Analysis Script

This script is designed to test the performance of matrix multiplication using both NumPy and CUDA. 
It is structured to provide insights into the system and GPU capabilities, as well as to give comparative 
performance metrics between CPU (via NumPy) and GPU (via CUDA).

Dependencies:
- Numba: Provides CUDA support in Python, allowing for GPU-accelerated applications.
- NumPy: A library for the Python programming language, adding support for large, multi-dimensional 
  arrays and matrices, along with a large collection of high-level mathematical functions to operate on these arrays.
- Python Standard Libraries: os, sys, time.

The script performs the following actions:
1. Sets the CUDA_PATH environment variable for CUDA toolkit access.
2. Defines a series of functions for printing system and GPU information, performance testing, and CUDA operations.
3. Executes a NumPy matrix multiplication test one million times to measure CPU performance.
4. Prepares matrices and executes a CUDA-based matrix multiplication to measure GPU performance.
5. Calculates and prints out the FLOPS (Floating Point Operations Per Second) to evaluate the performance.

Functions:
- print_cuda_results(execution_time, flops): Prints out the execution time and FLOPS for the CUDA operation.
- print_system_info(): Prints out system and GPU configuration details.
- numpy_matmul_performance_test(): Performs and times matrix multiplication using NumPy to gauge CPU performance.
- execute_and_measure_flops(cuda_kernel, kernel_args, num_operations, threads_per_block, is_2d): General function for executing a CUDA kernel and calculating its performance metrics.
- matmul(A, B, C): CUDA kernel for matrix multiplication.
- prepare_matrices(N): Prepares square matrices of size N for multiplication.
- allocate_device_memory(A, B, C): Allocates GPU memory for the matrices.

Usage:
1. Ensure CUDA and Numba are correctly installed and configured on your system.
2. Run the script in an environment that supports CUDA (e.g., on a system with a compatible NVIDIA GPU).
3. Review the printed system information and performance metrics.

Note: Adjust the size of the matrices (N) and the execution count in the numpy_matmul_performance_test() 
function as needed based on your system's capabilities and the desired thoroughness of the test.

Author: Bader Alotaibi [GitHub: bdr-pro]
Date: 2024
"""


# Set environment variable for CUDA_PATH
os.environ['CUDA_PATH'] = 'C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.4'

def print_cuda_results(execution_time, flops):
    print(f'CUDA matrix 1,000,000 multiplication execution time: {execution_time} seconds')
    print(f'CUDA matrix multiplication FLOPS: {flops:.2e}')
    print(f'CUDA matrix multiplication TFLOPS: {flops / 1e12:.2f}')


def print_system_info():
    # Print the system information
    print(f'OS: {os.name}')
    print(f'CPU: {os.cpu_count()} cores')
    print("")
    print("#####################################")
    print("")
    print(f"cuda.gpus {cuda.gpus}")
    print("")
    print("#####################################")
    print("")
    print(f"cuda.detect {cuda.detect()}")
    print("")
    print("#####################################")
    print("")
    print(f"cuda.get_current_device {cuda.get_current_device()}")
    print("#####################################")
    print("")
    print(f'CUDA: {os.environ["CUDA_PATH"]}')
    print("#####################################")
    print("")
    print(f'CUDA driver: {cuda.driver.get_version()}')
    print("#####################################")
    print("")
    print(f'CUDA compute capability: {cuda.gpus[0].compute_capability}')
    print("#####################################")
    print("")
    print(f'CUDA cores: {cuda.gpus[0].MULTIPROCESSOR_COUNT}')
    print(f'CUDA max threads per block: {cuda.gpus[0].MAX_THREADS_PER_BLOCK}')
    print(f'CUDA max block dimensions: {cuda.gpus[0].MAX_BLOCK_DIM_X}, {cuda.gpus[0].MAX_BLOCK_DIM_Y}, {cuda.gpus[0].MAX_BLOCK_DIM_Z}')
    print(f'CUDA max grid dimensions: {cuda.gpus[0].MAX_GRID_DIM_X}, {cuda.gpus[0].MAX_GRID_DIM_Y}, {cuda.gpus[0].MAX_GRID_DIM_Z}')
    print(f'CUDA max shared memory per block: {cuda.gpus[0].MAX_SHARED_MEMORY_PER_BLOCK / 1024:.2f} KB')
    print(f'CUDA warp size: {cuda.gpus[0].WARP_SIZE}')
    print(f'CUDA max registers per block: {cuda.gpus[0].MAX_REGISTERS_PER_BLOCK}')
    print(f'CUDA max registers per multiprocessor: {cuda.gpus[0].MAX_REGISTERS_PER_MULTIPROCESSOR}')
    print(f'CUDA max memory pitch: {cuda.gpus[0].MAX_PITCH}')
    print(f'CUDA memory clock rate: {cuda.gpus[0].MEMORY_CLOCK_RATE / 1024:.2f} MHz')
    print(f'CUDA L2 cache size: {cuda.gpus[0].L2_CACHE_SIZE / 1024:.2f} KB')
    print(f'CUDA max threads per block: {cuda.gpus[0].MAX_THREADS_PER_BLOCK}')
    print(f'CUDA max registers per block: {cuda.gpus[0].MAX_REGISTERS_PER_BLOCK}')
    print(f'CUDA max registers per multiprocessor: {cuda.gpus[0].MAX_REGISTERS_PER_MULTIPROCESSOR}')
    print(f'CUDA max memory pitch: {cuda.gpus[0].MAX_PITCH}')
    
    


# NumPy matrix multiplication performance test function
def numpy_matmul_performance_test():
    n = 100  # Define matrix size for practical execution times
    A = np.random.rand(n, n).astype(np.float32)
    B = np.random.rand(n, n).astype(np.float32)
    
    start_time = time.time()
    for index, _ in enumerate(range(1_000_000)):
        np.dot(A, B)  # Perform matrix multiplication
        # Print the completion percentage and overwrite the previous line
        print(f"Completion percentage {(index / 1_000_000 * 100):.2f}% of NumPy matrix multiplication performance test")
        sys.stdout.write("\033[F")  # Move the cursor up one line
        sys.stdout.write("\033[K")  # Clear the line
    end_time = time.time()
    execution_time = end_time - start_time
    # Print execution time for the NumPy matrix multiplication
    print(f'NumPy matrix multiplication (1,000,000 times) execution time: {execution_time} seconds')

# General function to execute a CUDA kernel and calculate FLOPS
def execute_and_measure_flops(cuda_kernel, kernel_args, num_operations, threads_per_block=(16, 16), is_2d=True, iterations=1):
    if is_2d:
        n_x, n_y = kernel_args[0].shape
        blocks_per_grid_x = (n_x + threads_per_block[0] - 1) // threads_per_block[0]
        blocks_per_grid_y = (n_y + threads_per_block[1] - 1) // threads_per_block[1]
        blocks_per_grid = (blocks_per_grid_x, blocks_per_grid_y)
    else:  # Fallback for 1D, though not used here, kept for generality
        n = kernel_args[0].size
        blocks_per_grid = (n + threads_per_block - 1) // threads_per_block

    # Time the kernel execution
    start_time = time.time()
    for index , _ in enumerate(range(iterations)):
        cuda_kernel[blocks_per_grid, threads_per_block](*kernel_args)
        print(f"Completion percentage {(index / 1_000_000 * 100):.2f}% of CUDA matrix multiplication performance test")
        sys.stdout.write("\033[F")  # Move the cursor up one line
        sys.stdout.write("\033[K")  # Clear the line
        
    cuda.synchronize()  # Wait for the GPU to finish
    end_time = time.time()

    # Calculate and return execution time and FLOPS
    execution_time = end_time - start_time
    flops = num_operations / execution_time
    return execution_time, flops

# Define the CUDA kernel for matrix multiplication
@cuda.jit
def matmul(A, B, C):
    row, col = cuda.grid(2)  # Get the 2D thread indices
    if row < C.shape[0] and col < C.shape[1]:
        tmp = 0.0
        for k in range(A.shape[1]):
            tmp += A[row, k] * B[k, col]
        C[row, col] = tmp

def prepare_matrices(N):
    # Prepare the input matrices
    A = np.random.rand(N, N).astype(np.float32)
    B = np.random.rand(N, N).astype(np.float32)
    C = np.zeros((N, N), dtype=np.float32)
    return A, B, C

def allocate_device_memory(A, B, C):
    # Allocate memory on the GPU for the matrices
    A_device = cuda.to_device(A)
    B_device = cuda.to_device(B)
    C_device = cuda.device_array_like(C)
    return A_device, B_device, C_device



# Main function to execute matrix multiplication in CUDA and measure performance
if __name__ == "__main__":
    # Print the environment and device information
    print_system_info()

    # Perform and print the NumPy matrix multiplication performance test
    numpy_matmul_performance_test()

    # Prepare matrices and parameters for CUDA matrix multiplication
    N = 512  # Size of the square matrices for CUDA operation
    A, B, C = prepare_matrices(N)

    # Allocate memory on the GPU for the matrices
    A_device, B_device, C_device = allocate_device_memory(A, B, C)

    # Number of floating-point operations for square matrix multiplication
    num_operations = 2 * N**3

    # Execute the CUDA kernel and measure FLOPS for matrix multiplication
    execution_time, flops = execute_and_measure_flops(
        matmul,  # CUDA kernel function
        (A_device, B_device, C_device),  # Arguments for the CUDA kernel
        num_operations,  # Total floating-point operations
        (16, 16),  # Threads per block
        True,  # Indicate it's a 2D operation
        1_000_000 # Number of times to execute the multiplication
    )

    # Print the CUDA matrix multiplication results
    print_cuda_results(execution_time, flops)
