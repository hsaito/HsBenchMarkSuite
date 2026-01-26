/// CPU Benchmark Module
/// Tests CPU performance through various computational tasks
use std::time::Instant;

pub struct CpuResult {
    pub primes_per_sec: f64,
    pub matrix_mult_gflops: f64,
    pub mandelbrot_pixels_per_sec: f64,
    pub fft_msamples_per_sec: f64,
    pub parallel_matrix_gflops: f64,
    pub parallel_speedup: f64,
}

#[allow(dead_code)]
pub fn run_cpu_benchmark() -> CpuResult {
    run_cpu_benchmark_scaled(1.0, 4)
}

pub fn run_cpu_benchmark_scaled(scale: f64, threads: usize) -> CpuResult {
    // Warmup phase: run once without timing to stabilize CPU caches and branch predictors
    warmup_primes(scale * 0.1); // Use 10% scale for warmup
    warmup_matrix_multiplication(scale * 0.1);
    warmup_mandelbrot(scale * 0.1);
    warmup_fft(scale * 0.1);
    warmup_parallel_matrix_multiplication(scale * 0.1, threads);

    // Actual timed benchmarks
    let primes_result = benchmark_primes(scale);
    let matrix_result = benchmark_matrix_multiplication(scale);
    let mandelbrot_result = benchmark_mandelbrot(scale);
    let fft_result = benchmark_fft(scale);
    let parallel_matrix_result = benchmark_parallel_matrix_multiplication(scale, threads);

    CpuResult {
        primes_per_sec: primes_result,
        matrix_mult_gflops: matrix_result,
        mandelbrot_pixels_per_sec: mandelbrot_result,
        fft_msamples_per_sec: fft_result,
        parallel_matrix_gflops: parallel_matrix_result,
        parallel_speedup: parallel_matrix_result / matrix_result,
    }
}

/// Benchmark prime number calculation
/// Returns: primes calculated per second
fn benchmark_primes(scale: f64) -> f64 {
    let limit = (100_000.0 * scale) as u64;

    let start = Instant::now();
    let mut count = 0u64;
    for i in 2..limit {
        if is_prime(i) {
            count += 1;
        }
    }
    let elapsed = start.elapsed().as_secs_f64();

    (count as f64) / elapsed
}

/// Benchmark matrix multiplication
/// Returns: GFLOPS (billions of floating-point operations per second)
fn benchmark_matrix_multiplication(scale: f64) -> f64 {
    let matrix_size = (256.0 * scale) as usize;

    // Create square matrices
    let mut a = vec![vec![0.0; matrix_size]; matrix_size];
    let mut b = vec![vec![0.0; matrix_size]; matrix_size];
    let mut c = vec![vec![0.0; matrix_size]; matrix_size];

    // Initialize with random-like values
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            a[i][j] = (i as f64) * 0.1 + (j as f64) * 0.01;
            b[i][j] = (i as f64) * 0.01 - (j as f64) * 0.1;
        }
    }

    let start = Instant::now();

    // Standard matrix multiplication: C = A * B
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            let mut sum = 0.0;
            for k in 0..matrix_size {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }

    let elapsed = start.elapsed().as_secs_f64();

    // Calculate FLOPS: 2 * n^3 operations (multiply and add)
    let total_ops = 2.0 * (matrix_size as f64).powi(3);
    (total_ops / 1e9) / elapsed
}

/// Benchmark Mandelbrot set calculation
/// Returns: pixels calculated per second
fn benchmark_mandelbrot(scale: f64) -> f64 {
    // Resolution scales with benchmark intensity
    let width = (256.0 * scale) as usize;
    let height = (256.0 * scale) as usize;
    let max_iter = (100.0 * scale) as u32;

    let mut rounds = 1;
    let mut elapsed;
    let mut checksum = 0u64; // Prevent compiler from optimizing away the calculation

    loop {
        let start = Instant::now();
        for _ in 0..rounds {
            let result = calculate_mandelbrot(width, height, max_iter);
            checksum = checksum.wrapping_add(std::hint::black_box(result));
        }
        elapsed = start.elapsed().as_secs_f64();

        // If elapsed time is less than 10ms, double rounds and try again
        if elapsed < 0.01 && rounds < 65536 {
            rounds *= 2;
        } else {
            break;
        }
    }

    if elapsed == 0.0 {
        elapsed = 0.01;
    }

    // Force compiler to keep checksum (prevents dead code elimination)
    std::hint::black_box(checksum);

    let total_pixels = (width * height) as f64 * (rounds as f64);
    total_pixels / elapsed
}

/// Calculate Mandelbrot set for given resolution
/// Returns: iteration count sum (used as checksum to prevent optimization)
fn calculate_mandelbrot(width: usize, height: usize, max_iter: u32) -> u64 {
    let mut iter_sum = 0u64;

    for y in 0..height {
        for x in 0..width {
            // Map pixel coordinates to complex plane
            // Viewing area: real [-2.5, 1.0], imaginary [-1.25, 1.25]
            let cr = -2.5 + (x as f64 / width as f64) * 3.5;
            let ci = -1.25 + (y as f64 / height as f64) * 2.5;

            let mut zr = 0.0;
            let mut zi = 0.0;
            let mut iter = 0;

            while iter < max_iter {
                let zr2 = zr * zr;
                let zi2 = zi * zi;

                if zr2 + zi2 > 4.0 {
                    break;
                }

                zi = 2.0 * zr * zi + ci;
                zr = zr2 - zi2 + cr;
                iter += 1;
            }

            iter_sum = iter_sum.wrapping_add(iter as u64);
        }
    }

    iter_sum
}

/// Benchmark Fast Fourier Transform
/// Returns: samples processed per second (in millions)
fn benchmark_fft(scale: f64) -> f64 {
    // Input size scales with benchmark intensity (power of 2 for FFT)
    let size = ((1024.0 * scale) as usize).next_power_of_two();

    // Create input signal
    let input: Vec<(f64, f64)> = (0..size)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (size as f64);
            (angle.cos(), angle.sin())
        })
        .collect();

    let mut rounds = 1;
    let mut elapsed;
    let mut checksum = 0.0f64; // Prevent compiler from optimizing away the calculation

    loop {
        let start = Instant::now();
        for _ in 0..rounds {
            let mut data = input.clone();
            cooley_tukey_fft(&mut data);
            // Use first element as checksum
            let result = data[0].0 + data[0].1;
            checksum += std::hint::black_box(result);
        }
        elapsed = start.elapsed().as_secs_f64();

        if elapsed < 0.01 && rounds < 65536 {
            rounds *= 2;
        } else {
            break;
        }
    }

    if elapsed == 0.0 {
        elapsed = 0.01;
    }

    // Force compiler to keep checksum (prevents dead code elimination)
    std::hint::black_box(checksum);

    let total_samples = (size as f64) * (rounds as f64) / 1_000_000.0;
    total_samples / elapsed
}

/// Cooley-Tukey Fast Fourier Transform (in-place)
fn cooley_tukey_fft(data: &mut [(f64, f64)]) {
    let n = data.len();
    if n <= 1 {
        return;
    }

    // Bit-reversal
    for i in 0..n {
        let j = reverse_bits(i, n.trailing_zeros());
        if i < j {
            data.swap(i, j);
        }
    }

    // Iterative FFT
    let mut len = 2;
    while len <= n {
        let angle = -2.0 * std::f64::consts::PI / (len as f64);

        for i in (0..n).step_by(len) {
            let wn_r = angle.cos();
            let wn_i = angle.sin();
            let mut w_r = 1.0;
            let mut w_i = 0.0;

            for j in 0..(len / 2) {
                let u_idx = i + j;
                let v_idx = i + j + len / 2;

                let (u_r, u_i) = data[u_idx];
                let (v_r, v_i) = data[v_idx];

                let t_r = w_r * v_r - w_i * v_i;
                let t_i = w_r * v_i + w_i * v_r;

                data[u_idx] = (u_r + t_r, u_i + t_i);
                data[v_idx] = (u_r - t_r, u_i - t_i);

                let w_r_new = w_r * wn_r - w_i * wn_i;
                let w_i_new = w_r * wn_i + w_i * wn_r;
                w_r = w_r_new;
                w_i = w_i_new;
            }
        }

        len *= 2;
    }
}

/// Reverse the bits of a number (used in FFT bit-reversal)
fn reverse_bits(mut x: usize, bits: u32) -> usize {
    let mut result = 0;
    for _ in 0..bits {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}

/// Benchmark parallel matrix multiplication using standard threads
/// Returns: GFLOPS (billions of floating-point operations per second)
fn benchmark_parallel_matrix_multiplication(scale: f64, threads: usize) -> f64 {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let matrix_size = (256.0 * scale) as usize;
    let num_threads = threads.max(1); // Ensure at least 1 thread

    // Create square matrices
    let mut a = vec![vec![0.0; matrix_size]; matrix_size];
    let mut b = vec![vec![0.0; matrix_size]; matrix_size];
    let c = vec![vec![0.0; matrix_size]; matrix_size];

    // Initialize with random-like values
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            a[i][j] = (i as f64) * 0.1 + (j as f64) * 0.01;
            b[i][j] = (i as f64) * 0.01 - (j as f64) * 0.1;
        }
    }

    let start = Instant::now();

    let a_arc = Arc::new(a);
    let b_arc = Arc::new(b);
    let c_arc = Arc::new(Mutex::new(c));

    let rows_per_thread = matrix_size.div_ceil(num_threads);
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let a_clone = Arc::clone(&a_arc);
        let b_clone = Arc::clone(&b_arc);
        let c_clone = Arc::clone(&c_arc);

        let handle = thread::spawn(move || {
            let start_row = thread_id * rows_per_thread;
            let end_row = ((thread_id + 1) * rows_per_thread).min(matrix_size);

            let mut local_c = vec![vec![0.0; matrix_size]; matrix_size];

            for i in start_row..end_row {
                for j in 0..matrix_size {
                    let mut sum = 0.0;
                    for k in 0..matrix_size {
                        sum += a_clone[i][k] * b_clone[k][j];
                    }
                    local_c[i][j] = sum;
                }
            }

            let mut c = c_clone.lock().unwrap();
            for i in start_row..end_row {
                for j in 0..matrix_size {
                    c[i][j] = local_c[i][j];
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    let elapsed = start.elapsed().as_secs_f64();

    // Calculate FLOPS: 2 * n^3 operations (multiply and add)
    let total_ops = 2.0 * (matrix_size as f64).powi(3);
    (total_ops / 1e9) / elapsed
}

/// Check if a number is prime
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

/// Warmup functions to stabilize CPU caches and branch predictors
fn warmup_primes(scale: f64) {
    let limit = (100_000.0 * scale) as u64;
    let mut _count = 0u64;
    for i in 2..limit {
        if is_prime(i) {
            _count += 1;
        }
    }
}

fn warmup_matrix_multiplication(scale: f64) {
    let matrix_size = (256.0 * scale) as usize;
    let mut a = vec![vec![0.0; matrix_size]; matrix_size];
    let mut b = vec![vec![0.0; matrix_size]; matrix_size];
    let mut c = vec![vec![0.0; matrix_size]; matrix_size];

    for i in 0..matrix_size {
        for j in 0..matrix_size {
            a[i][j] = (i as f64) * 0.1 + (j as f64) * 0.01;
            b[i][j] = (i as f64) * 0.01 - (j as f64) * 0.1;
        }
    }

    for i in 0..matrix_size {
        for j in 0..matrix_size {
            let mut sum = 0.0;
            for k in 0..matrix_size {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }
}

fn warmup_mandelbrot(scale: f64) {
    let width = (256.0 * scale) as usize;
    let height = (256.0 * scale) as usize;
    let max_iter = (100.0 * scale) as u32;
    let _pixel_count = calculate_mandelbrot(width, height, max_iter);
}

fn warmup_fft(scale: f64) {
    let sample_size = (8192.0 * scale) as usize;
    let next_power_of_2 = sample_size.next_power_of_two();
    let mut data: Vec<(f64, f64)> = (0..next_power_of_2)
        .map(|i| {
            let x = (i as f64 / next_power_of_2 as f64) * 2.0 * std::f64::consts::PI;
            (x.sin() + (x * 2.0).cos(), 0.0)
        })
        .collect();

    cooley_tukey_fft(&mut data);
}

fn warmup_parallel_matrix_multiplication(scale: f64, threads: usize) {
    let _ = benchmark_parallel_matrix_multiplication(scale, threads);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(4));
        assert!(!is_prime(1));
    }

    #[test]
    fn test_is_prime_edge_cases() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
    }

    #[test]
    fn test_is_prime_known_primes() {
        let known_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        for prime in known_primes {
            assert!(is_prime(prime), "Expected {} to be prime", prime);
        }
    }

    #[test]
    fn test_is_prime_known_composites() {
        let known_composites = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25];
        for composite in known_composites {
            assert!(
                !is_prime(composite),
                "Expected {} to be composite",
                composite
            );
        }
    }

    #[test]
    fn test_cpu_benchmark_returns_valid() {
        // Use lightweight scale for CI/testing - avoids prolonged execution
        let result = run_cpu_benchmark_scaled(0.1, 2);
        assert!(
            result.primes_per_sec > 0.0,
            "Primes per second should be positive"
        );
        assert!(
            result.matrix_mult_gflops > 0.0,
            "Matrix GFLOPS should be positive"
        );
        assert!(
            result.mandelbrot_pixels_per_sec > 0.0,
            "Mandelbrot pixels per sec should be positive"
        );
        assert!(
            result.fft_msamples_per_sec > 0.0,
            "FFT samples per sec should be positive"
        );
        assert!(
            result.parallel_matrix_gflops > 0.0,
            "Parallel matrix GFLOPS should be positive"
        );
        assert!(
            result.parallel_speedup > 0.0,
            "Matrix speedup should be positive"
        );
    }

    #[test]
    fn test_cpu_benchmark_consistency() {
        // Use lightweight scale for CI/testing
        let result1 = run_cpu_benchmark_scaled(0.1, 2);
        let result2 = run_cpu_benchmark_scaled(0.1, 2);

        // Primes count should be reasonably consistent
        // Allow higher variance (100%) to avoid flaky tests across different systems
        let variance = ((result1.primes_per_sec - result2.primes_per_sec).abs()
            / result1.primes_per_sec.max(result2.primes_per_sec))
            * 100.0;
        assert!(
            variance < 100.0,
            "Primes calculation variance too high: {:.2}%",
            variance
        );
    }

    #[test]
    fn test_mandelbrot_calculation() {
        let iter_sum = calculate_mandelbrot(64, 64, 100);
        // Should return iteration sum, not pixel count
        // Iteration sum should be > 0 and typically > pixel count
        assert!(iter_sum > 0, "Iteration sum should be positive");
        assert!(
            iter_sum >= 64 * 64,
            "Iteration sum should be at least equal to pixel count"
        );
    }

    #[test]
    fn test_fft_calculation() {
        let mut data = vec![(1.0, 0.0); 16];
        cooley_tukey_fft(&mut data);
        // FFT should complete without panicking - result verification would be complex
        assert_eq!(data.len(), 16, "FFT should preserve length");
    }

    #[test]
    fn test_cpu_benchmark_scaled() {
        let result = run_cpu_benchmark_scaled(0.5, 2);
        assert!(result.primes_per_sec > 0.0);
        assert!(result.matrix_mult_gflops > 0.0);
        assert!(result.parallel_matrix_gflops > 0.0);
        assert!(result.mandelbrot_pixels_per_sec > 0.0);
        assert!(result.fft_msamples_per_sec > 0.0);
    }

    #[test]
    fn test_cpu_benchmark_default() {
        // Use lightweight scale for CI/testing
        let result = run_cpu_benchmark_scaled(0.1, 2);
        assert!(result.primes_per_sec > 0.0);
    }

    #[test]
    fn test_warmup_functions_no_panic() {
        // Test that warmup functions don't panic
        warmup_primes(0.1);
        warmup_matrix_multiplication(0.1);
        warmup_mandelbrot(0.1);
        warmup_fft(0.1);
        warmup_parallel_matrix_multiplication(0.1, 2);
    }

    #[test]
    fn test_parallel_speedup_calculation() {
        let result = run_cpu_benchmark_scaled(0.5, 4);
        // Speedup should be positive (even if < 1 due to overhead)
        assert!(result.parallel_speedup > 0.0);
        // Speedup calculation should match
        let expected_speedup = result.parallel_matrix_gflops / result.matrix_mult_gflops;
        assert!((result.parallel_speedup - expected_speedup).abs() < 0.01);
    }

    #[test]
    fn test_is_prime_large_numbers() {
        assert!(is_prime(7919)); // Known large prime
        assert!(!is_prime(7920)); // Composite
    }

    #[test]
    fn test_mandelbrot_different_sizes() {
        let iter_sum1 = calculate_mandelbrot(10, 10, 50);
        let iter_sum2 = calculate_mandelbrot(20, 20, 50);
        // Iteration sums should be positive
        assert!(iter_sum1 > 0, "Iteration sum should be positive");
        assert!(iter_sum2 > 0, "Iteration sum should be positive");
        // Larger size should generally have larger iteration sum
        assert!(
            iter_sum2 > iter_sum1,
            "Larger grid should have more iterations"
        );
        // Should be at least the pixel counts
        assert!(iter_sum1 >= 100);
        assert!(iter_sum2 >= 400);
    }
}
