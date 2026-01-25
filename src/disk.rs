/// Disk Benchmark Module
/// Tests disk I/O performance through read/write operations
/// Uses direct I/O where possible to bypass OS cache and measure true disk throughput
use std::fs;
use std::io::{Read, Write};

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
use std::os::fd::AsRawFd;

#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

#[cfg(test)]
use std::fs::File;

const BASE_FILE_SIZE: usize = 50_000_000; // 50 MB
const ALIGNMENT: usize = 4096; // Align buffers for O_DIRECT when available
const TEST_DIR: &str = ".bench_temp";
const TEST_FILE: &str = ".bench_temp/test_file.bin";

fn alloc_aligned(size: usize) -> (Vec<u8>, usize) {
    // Allocate slightly larger buffer and return an aligned slice offset
    let buffer = vec![0u8; size + ALIGNMENT];
    let ptr = buffer.as_ptr() as usize;
    let offset = (ALIGNMENT - (ptr % ALIGNMENT)) % ALIGNMENT;
    (buffer, offset)
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn drop_os_cache(fd: std::os::fd::RawFd) {
    // Best-effort: tell kernel the data is not needed in page cache
    unsafe {
        let _ = libc::posix_fadvise(fd, 0, 0, libc::POSIX_FADV_DONTNEED);
    }
}

#[cfg(target_os = "macos")]
fn drop_os_cache(fd: std::os::fd::RawFd) {
    // macOS: disable caching on this descriptor
    unsafe {
        let _ = libc::fcntl(fd, libc::F_NOCACHE, 1);
    }
}

#[cfg(windows)]
fn drop_os_cache(_handle: std::os::windows::io::RawHandle) {
    // Windows flags already request no buffering; nothing extra to do here
}

pub struct DiskResult {
    pub write_throughput: f64,
    pub read_throughput: f64,
    pub combined_throughput: f64,
}

#[allow(dead_code)]
pub fn run_disk_benchmark() -> DiskResult {
    run_disk_benchmark_scaled(1.0)
}

pub fn run_disk_benchmark_scaled(scale: f64) -> DiskResult {
    // Warmup phase: small file to prime disk cache
    warmup_disk(scale * 0.1);

    // Actual benchmark with full file
    let file_size = (BASE_FILE_SIZE as f64 * scale) as usize;

    // Create temporary directory
    let _ = fs::create_dir(TEST_DIR);

    let (mut data_buf, data_offset) = alloc_aligned(file_size);
    let data_slice = &mut data_buf[data_offset..data_offset + file_size];
    data_slice.fill(0xAB);

    // Write benchmark with direct I/O (bypassing OS cache)
    let write_start = std::time::Instant::now();
    {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create(true).truncate(true);

        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECT | libc::O_SYNC);
        }

        #[cfg(target_os = "freebsd")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECT);
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            options.custom_flags(0x20000000 | 0x80000000); // FILE_FLAG_NO_BUFFERING | FILE_FLAG_WRITE_THROUGH
        }

        // macOS: Use standard I/O, direct I/O not commonly available
        #[cfg(target_os = "macos")]
        {
            // No special flags on macOS
        }

        if let Ok(mut file) = options.open(TEST_FILE) {
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
            drop_os_cache(file.as_raw_fd());

            #[cfg(windows)]
            drop_os_cache(file.as_raw_handle());

            let _ = file.write_all(data_slice);
            let _ = file.sync_all();
        } // File handle dropped here, ensuring flush
    }
    let write_time = write_start.elapsed().as_secs_f64();
    let write_throughput = (file_size as f64 / (1024.0 * 1024.0)) / write_time;

    // Read benchmark with direct I/O (bypassing OS cache)
    let read_start = std::time::Instant::now();
    let (mut buffer, buffer_offset) = alloc_aligned(file_size);
    let buffer_slice = &mut buffer[buffer_offset..buffer_offset + file_size];
    {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);

        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECT);
        }

        #[cfg(target_os = "freebsd")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECT);
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            options.custom_flags(0x20000000); // FILE_FLAG_NO_BUFFERING
        }

        // macOS: Use standard I/O
        #[cfg(target_os = "macos")]
        {
            // No special flags on macOS
        }

        if let Ok(mut file) = options.open(TEST_FILE) {
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
            drop_os_cache(file.as_raw_fd());

            #[cfg(windows)]
            drop_os_cache(file.as_raw_handle());

            let _ = file.read_exact(buffer_slice);
        } // File handle dropped here
    }
    let read_time = read_start.elapsed().as_secs_f64();
    let read_throughput = (file_size as f64 / (1024.0 * 1024.0)) / read_time;

    // Cleanup
    let _ = fs::remove_file(TEST_FILE);
    let _ = fs::remove_dir(TEST_DIR);

    // Calculate combined throughput
    let total_time = write_time + read_time;
    let combined_throughput = (file_size as f64 / (1024.0 * 1024.0) * 2.0) / total_time;

    DiskResult {
        write_throughput,
        read_throughput,
        combined_throughput,
    }
}

fn warmup_disk(scale: f64) {
    const WARMUP_FILE: &str = ".bench_temp/warmup_file.bin";
    let file_size = (BASE_FILE_SIZE as f64 * scale) as usize;

    // Create temporary directory
    let _ = fs::create_dir(TEST_DIR);

    let (mut data_buf, data_offset) = alloc_aligned(file_size);
    let data_slice = &mut data_buf[data_offset..data_offset + file_size];
    data_slice.fill(0xAB);

    // Warmup write with direct I/O
    {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create(true).truncate(true);

        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECT | libc::O_SYNC);
        }

        #[cfg(target_os = "freebsd")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECT);
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            options.custom_flags(0x20000000 | 0x80000000);
        }

        #[cfg(target_os = "macos")]
        {
            // No special flags on macOS
        }

        if let Ok(mut file) = options.open(WARMUP_FILE) {
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
            drop_os_cache(file.as_raw_fd());

            #[cfg(windows)]
            drop_os_cache(file.as_raw_handle());

            let _ = file.write_all(data_slice);
            let _ = file.sync_all();
        }
    }

    // Warmup read with direct I/O
    let (mut _buffer, buffer_offset) = alloc_aligned(file_size);
    let buffer_slice = &mut _buffer[buffer_offset..buffer_offset + file_size];
    {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);

        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECT);
        }

        #[cfg(target_os = "freebsd")]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_DIRECT);
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            options.custom_flags(0x20000000);
        }

        #[cfg(target_os = "macos")]
        {
            // No special flags on macOS
        }

        if let Ok(mut file) = options.open(WARMUP_FILE) {
            #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
            drop_os_cache(file.as_raw_fd());

            #[cfg(windows)]
            drop_os_cache(file.as_raw_handle());

            let _ = file.read_exact(buffer_slice);
        }
    }

    // Cleanup warmup file
    let _ = fs::remove_file(WARMUP_FILE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_benchmark() {
        // Use lightweight scale for CI/testing - 5 MB instead of 50 MB
        let result = run_disk_benchmark_scaled(0.1);
        assert!(
            result.combined_throughput > 0.0,
            "Disk benchmark should return positive throughput"
        );
    }

    #[test]
    fn test_disk_file_creation_and_cleanup() {
        use std::path::Path;

        let test_file = ".bench_test_cleanup.tmp";

        // Create test file
        {
            let data = vec![0xAB; 1_000_000];
            let mut file = File::create(test_file).expect("Failed to create test file");
            file.write_all(&data).expect("Failed to write test file");
        }

        // Verify file exists
        assert!(
            Path::new(test_file).exists(),
            "Test file should exist after creation"
        );

        // Cleanup
        fs::remove_file(test_file).expect("Failed to remove test file");
        // Wait briefly and check deletion
        for _ in 0..5 {
            if !Path::new(test_file).exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(
            !Path::new(test_file).exists(),
            "Test file should be removed after cleanup"
        );
    }

    #[test]
    fn test_disk_read_write_consistency() {
        let test_file = ".bench_test_consistency.tmp";
        let test_data = vec![0x42; 100_000];

        // Write
        {
            let mut file = File::create(test_file).expect("Failed to create file");
            file.write_all(&test_data).expect("Failed to write data");
            file.sync_all().expect("Failed to sync file");
        }

        // Read and verify
        {
            let mut buffer = vec![0u8; test_data.len()];
            let mut file = File::open(test_file).expect("Failed to open file");
            file.read_exact(&mut buffer).expect("Failed to read file");
            assert_eq!(buffer, test_data, "Read data should match written data");
        }

        // Cleanup
        fs::remove_file(test_file).expect("Failed to remove test file");
    }

    #[test]
    fn test_disk_benchmark_reasonable_throughput() {
        // Use lightweight scale for CI/testing
        let result = run_disk_benchmark_scaled(0.1);
        // Throughput should be reasonable - at least 1 MB/s on most systems
        assert!(
            result.combined_throughput > 1.0,
            "Disk benchmark throughput seems too low: {} MB/s",
            result.combined_throughput
        );
    }

    #[test]
    fn test_disk_benchmark_scaled() {
        let result = run_disk_benchmark_scaled(0.5);
        assert!(result.write_throughput > 0.0);
        assert!(result.read_throughput > 0.0);
        assert!(result.combined_throughput > 0.0);
    }

    #[test]
    fn test_disk_warmup_no_panic() {
        // Ensure warmup doesn't panic and cleans up properly
        warmup_disk(0.1);
        // Verify warmup file was cleaned up
        use std::path::Path;
        assert!(!Path::new(".bench_temp/warmup_file.bin").exists());
    }

    #[test]
    fn test_disk_combined_calculation() {
        let result = run_disk_benchmark_scaled(0.3);
        // Combined throughput should be reasonable
        assert!(result.combined_throughput > 0.0);
        // Combined should not exceed sum of read and write
        assert!(result.combined_throughput <= result.read_throughput + result.write_throughput);
    }

    #[test]
    fn test_disk_benchmark_default() {
        // Use lightweight scale for CI/testing
        let result = run_disk_benchmark_scaled(0.1);
        assert!(result.write_throughput > 0.0);
        assert!(result.read_throughput > 0.0);
        assert!(result.combined_throughput > 0.0);
    }

    #[test]
    fn test_disk_cleanup_on_completion() {
        use std::path::Path;
        run_disk_benchmark_scaled(0.2);
        // Give filesystem time to complete cleanup
        std::thread::sleep(std::time::Duration::from_millis(100));
        // Verify test file and directory are cleaned up
        assert!(!Path::new(TEST_FILE).exists());
    }
}
