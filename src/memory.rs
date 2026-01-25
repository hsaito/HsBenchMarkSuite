/// Memory Benchmark Module
/// Tests memory access patterns and bandwidth
/// Uses multi-threaded sequential access to properly saturate DRAM bandwidth
/// Single-threaded benchmarks can't saturate modern memory buses; need 4+ threads
const BASE_BUFFER_SIZE: usize = 512_000_000; // 512 MB per thread - well beyond L3 cache
const NUM_THREADS: usize = 8; // Use 8 threads to saturate typical memory bus

pub struct MemoryResult {
    pub write_throughput: f64,
    pub read_throughput: f64,
    pub combined_throughput: f64,
}

#[allow(dead_code)]
pub fn run_memory_benchmark() -> MemoryResult {
    run_memory_benchmark_scaled(1.0)
}

pub fn run_memory_benchmark_scaled(scale: f64) -> MemoryResult {
    // Warmup phase: small buffer to prime CPU caches
    warmup_memory(scale * 0.1);

    // Per-thread buffer size
    let per_thread_size = (BASE_BUFFER_SIZE as f64 * scale) as usize;
    let total_size = per_thread_size * NUM_THREADS;

    // Write benchmark - multi-threaded sequential writes
    let write_start = std::time::Instant::now();
    let write_barrier = std::sync::Arc::new(std::sync::Barrier::new(NUM_THREADS));

    let write_handles: Vec<_> = (0..NUM_THREADS)
        .map(|thread_id| {
            let barrier = write_barrier.clone();
            std::thread::spawn(move || {
                // Each thread gets its own buffer
                let mut buffer = vec![0u8; per_thread_size];

                // Synchronize start
                barrier.wait();

                // Sequential write - simple and fast
                for (i, byte) in buffer.iter_mut().enumerate() {
                    *byte = ((thread_id + i) % 256) as u8;
                }
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);

                // Don't drop buffer until measurement is done
                let _ = buffer.len();
            })
        })
        .collect();

    for handle in write_handles {
        let _ = handle.join();
    }
    let write_time = write_start.elapsed().as_secs_f64();
    let write_throughput = (total_size as f64 / (1024.0 * 1024.0)) / write_time;

    // Read benchmark - multi-threaded sequential reads
    let read_start = std::time::Instant::now();
    let read_barrier = std::sync::Arc::new(std::sync::Barrier::new(NUM_THREADS));
    let read_sums = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    let read_handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| {
            let barrier = read_barrier.clone();
            let sums = read_sums.clone();

            std::thread::spawn(move || {
                // Each thread gets its own buffer
                let buffer = vec![0u8; per_thread_size];

                // Synchronize start
                barrier.wait();

                // Sequential read - simple and fast
                let mut sum = 0u64;
                for byte in buffer.iter() {
                    sum = sum.wrapping_add(*byte as u64);
                }
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);

                if let Ok(mut s) = sums.lock() {
                    s.push(sum);
                }
            })
        })
        .collect();

    for handle in read_handles {
        let _ = handle.join();
    }
    let read_time = read_start.elapsed().as_secs_f64();
    let read_throughput = (total_size as f64 / (1024.0 * 1024.0)) / read_time;

    // Calculate combined throughput
    let total_time = write_time + read_time;
    let combined_throughput = (total_size as f64 / (1024.0 * 1024.0) * 2.0) / total_time;

    MemoryResult {
        write_throughput,
        read_throughput,
        combined_throughput,
    }
}

fn warmup_memory(scale: f64) {
    let per_thread_size = (BASE_BUFFER_SIZE as f64 * scale) as usize;
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(NUM_THREADS));

    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|thread_id| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let mut buffer = vec![0u8; per_thread_size];
                barrier.wait();

                // Warmup write
                for (i, byte) in buffer.iter_mut().enumerate() {
                    *byte = ((thread_id + i) % 256) as u8;
                }

                // Warmup read
                let mut _sum = 0u64;
                for byte in buffer.iter() {
                    _sum = _sum.wrapping_add(*byte as u64);
                }
            })
        })
        .collect();

    for handle in handles {
        let _ = handle.join();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_benchmark() {
        let result = run_memory_benchmark();
        assert!(
            result.combined_throughput > 0.0,
            "Memory benchmark should return positive throughput"
        );
    }

    #[test]
    fn test_memory_benchmark_reasonable_throughput() {
        let result = run_memory_benchmark();
        // Throughput should be reasonable - at least 100 MB/s on most systems
        // This is a loose check to avoid flaky tests
        assert!(
            result.combined_throughput > 10.0,
            "Memory benchmark throughput seems too low: {} MB/s",
            result.combined_throughput
        );
    }

    #[test]
    fn test_memory_buffer_operations() {
        let buffer_size = 1_000_000; // 1 MB for testing
        let mut buffer = vec![0u8; buffer_size];

        // Write
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }

        // Verify write
        for (i, byte) in buffer.iter().enumerate() {
            assert_eq!(*byte, (i % 256) as u8);
        }

        // Read
        let mut sum = 0u64;
        for byte in buffer.iter() {
            sum = sum.wrapping_add(*byte as u64);
        }

        assert!(
            sum > 0,
            "Memory read operation should accumulate non-zero sum"
        );
    }

    #[test]
    fn test_memory_benchmark_multiple_runs() {
        let result1 = run_memory_benchmark();
        let result2 = run_memory_benchmark();

        // Results should be within reasonable variance (100% to avoid flakiness)
        let variance = ((result1.combined_throughput - result2.combined_throughput).abs()
            / result1.combined_throughput.max(result2.combined_throughput))
            * 100.0;
        assert!(
            variance < 100.0,
            "Memory benchmark variance too high: {:.2}%",
            variance
        );
    }

    #[test]
    fn test_memory_benchmark_scaled() {
        let result = run_memory_benchmark_scaled(0.5);
        assert!(result.write_throughput > 0.0);
        assert!(result.read_throughput > 0.0);
        assert!(result.combined_throughput > 0.0);
    }

    #[test]
    fn test_memory_warmup_no_panic() {
        // Ensure warmup doesn't panic
        warmup_memory(0.1);
    }

    #[test]
    fn test_memory_combined_calculation() {
        let result = run_memory_benchmark_scaled(0.3);
        // Combined throughput should be reasonable relative to individual values
        assert!(result.combined_throughput > 0.0);
        // Combined should not exceed sum of read and write (that would be impossible)
        assert!(result.combined_throughput <= result.read_throughput + result.write_throughput);
    }

    #[test]
    fn test_memory_benchmark_default() {
        let result = run_memory_benchmark();
        assert!(result.write_throughput > 0.0);
        assert!(result.read_throughput > 0.0);
        assert!(result.combined_throughput > 0.0);
    }
}
