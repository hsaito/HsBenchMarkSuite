# Changelog

All notable changes to HsBenchMarkSuite are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-01-25

### Changed (Breaking)

- **Metric Naming**: Switched thread markers and JSON keys to intent-based labels
  - JSON keys: `cpu_matrix_mult_gflops_1t` → `cpu_matrix_mult_gflops_st`, `cpu_matrix_mult_gflops_{threads}t` → `cpu_matrix_mult_gflops_mt`, `cpu_parallel_speedup_{threads}t` → `cpu_parallel_speedup_mt`
  - CSV headers and console output now use `ST` (single-threaded) and `MT` (multi-threaded)
  - Speedup label updated to `Speedup (ST->MT)` to explicitly indicate the comparison
  - Rationale: Avoid confusion when configured threads are not 4 and standardize labels across outputs

### Documentation

- README updated with ST/MT definitions and the speedup formula (MT GFLOPS / ST GFLOPS)

## [0.2.4] - 2026-01-25

### Fixed

- **CRITICAL: Mandelbrot & FFT Benchmarks**: Fixed aggressive compiler optimization in release builds
  - Added `std::hint::black_box()` to prevent dead code elimination of benchmark calculations
  - Resolves issue where release builds showed absurdly high results (42+ quadrillion pixels/sec)
  - Compiler was optimizing away most/all calculations when results weren't "observably used"
  - Now properly accumulates computation results through black_box to force full execution
  - Changed `calculate_mandelbrot()` to return iteration sum instead of pixel count for meaningful checksums
  - Debug builds were unaffected; only release builds with `-O3` and LTO showed the problem
  - **Impact**: Release build results now accurately reflect actual computational work performed

### Changed

- Documentation improvements related to benchmark robustness and clarity
  - Clarified adaptive timing thresholds and use of `std::hint::black_box()` in CPU benchmarks
  - Ensures release builds produce accurate, variance-rich results

## [0.2.3] - 2026-01-25

### Fixed

- **Mandelbrot & FFT Benchmarks**: Fixed timing threshold issue causing artificially identical measurements
  - Increased minimum timing threshold from 1ms to 10ms for more accurate measurements
  - Increased max_rounds from 256 to 65,536 to allow proper scaling for fast operations
  - Resolves issue where all benchmark runs would produce identical results with 0% standard deviation
  - Ensures benchmarks can properly measure variance and produce statistically meaningful results
  - Affects both `benchmark_mandelbrot()` and `benchmark_fft()` functions

## [0.2.2] - 2026-01-25

### Added

- **Configurable Block Size for Disk Benchmarks**
  - New `run_disk_benchmark_scaled_with_block_size(scale, block_size)` function allows custom block sizes
  - New `--block-size <SIZE>` CLI argument for command-line configuration (default: 524288 = 512 KB)
  - Default block size: 512 KB (practical for most sequential I/O workloads)
  - Enables testing different I/O patterns (131072 for 128 KB random-access, 1048576 for 1 MB streaming, etc.)
  - Maintains platform equivalence with direct I/O flags across Windows, Linux, FreeBSD, and macOS

- **Parameter Mismatch Warnings in HTML Visualization Tool**
  - New automatic detection when comparing benchmark files with different parameters
  - Displays warning banner highlighting differences in: scale, runs (count), threads, and block_size
  - Shows configuration details for each loaded file (Scale, Runs, Threads, Block size)
  - Prevents misleading comparisons by alerting users to parameter inconsistencies

### Fixed

- **Mandelbrot Benchmark**: Fixed pixel throughput calculation bug where multiply factor was incorrectly applied
  - Previously: `pixel_count * rounds` (where `pixel_count` was last iteration's value)
  - Now: `(width * height) * rounds` (actual pixels processed)
  - Resolves 1000× variance in Run 3 measurements reported in earlier benchmarks
- **HTML Visualization Tool**: Fixed issue where metric bars could fail to render when metric values were missing or malformed in loaded JSON files
  - Improved null/undefined value handling in metric extraction
  - Ensures all metric values are valid numbers with fallback to 0

## [0.2.1] - 2026-01-25

### Added

- **Enhanced Documentation**
  - Reorganized README.md with separate "Data Export and Visualization" section
  - Added dedicated "Using the HTML Visualization Tool" section with detailed usage instructions
  - Comprehensive example workflows for trend analysis, cross-platform comparison, and configuration testing
  - Technical details about Chart.js integration and responsive design

### Changed

- **Release Artifact Naming**: ZIP files now use version suffix format (e.g., `hs-benchmark-suite-linux-x86_64-v0.2.1.zip`)
- **Release Contents**: All release ZIPs now include README.md, LICENSE, and visualize.html alongside binaries

## [0.2.0] - 2026-01-25

### Added

- **Interactive HTML Visualization Tool** (`visualize.html`)
  - Drag-and-drop JSON file loading for easy data import
  - Multi-file comparison with side-by-side charts
  - Automatic hostname and timestamp extraction from JSON metadata
  - Responsive grid layout for displaying multiple metrics
  - Interactive metric selection and filtering
  - Chart.js integration for professional data visualization
  - File management (add/remove files without clearing others)

- **Timestamped Output Files**
  - CSV and JSON files now use `output_YYYYMMDD_HHMMSS` naming format
  - Prevents accidental file overwrites
  - Enables easy historical tracking and comparison
  - JSON metadata includes RFC3339 timestamp for precise temporal tracking

- **Enhanced JSON Output**
  - Metadata section with RFC3339 timestamp and hostname
  - Machine identification enables cross-machine performance comparison
  - Prefixed metric names for easier identification:
    - CPU metrics: `cpu_primes_per_sec`, `cpu_matrix_mult_gflops_*`, etc.
    - Memory metrics: `memory_write_throughput_mbs`, `memory_read_throughput_mbs`, etc.
    - Disk metrics: `disk_write_throughput_mbs`, `disk_read_throughput_mbs`, etc.

- **Improved CSV Output**
  - Removed comment header lines for direct data import
  - Direct compatibility with Excel, Python, R, and other data analysis tools
  - Clean structure: immediate header row followed by data rows

- **Comprehensive Documentation Updates**
  - "Data Export and Visualization" section in README
  - Explanation of JSON metadata and timestamping
  - Usage examples for visualization tool
  - Warning about statistical metrics in single-run mode

- **Release Workflow Enhancement**
  - README.md, LICENSE, and visualize.html now included in release ZIP files
  - Guardrail to ensure releases are only created for commits on main branch

### Changed

- **Default Benchmark Count**: Changed from 1 to 3 runs
  - Provides better statistical data by default
  - More reliable performance measurements
  - Reduced variance in reported metrics

- **Default Arguments Help Text**
  - Updated to reflect new default count of 3
  - Clarified behavior of `--count` parameter

- **Statistical Analysis Documentation**
  - Added note explaining that statistical metrics are only meaningful with `--count > 1`
  - Recommendation to use at least 3-5 runs for reliable statistical analysis
  - Clarification about single-run limitations

### Technical Improvements

- Integrated `chrono` 0.4 dependency for robust datetime handling
- Hostname capture via `System::host_name()` for system identification
- Improved JSON structure for better data organization and consumption
- Fixed file input state management in HTML visualization tool
- Responsive CSS grid layout for adaptive chart display

### Bug Fixes

- Fixed file input not accepting files after clearing in HTML visualizer
- Fixed status message display to hide when empty
- Fixed file removal to properly clear display when last file is removed
- Fixed append behavior for multiple file selections
- Fixed chart display to properly group data by metric instead of by file

## Project Highlights

### Benchmark Categories

The suite includes comprehensive benchmarks for:

- **CPU**: Prime calculation, matrix multiplication, Mandelbrot set, FFT
- **Memory**: Sequential read/write throughput
- **Disk**: File I/O read/write throughput

### Statistical Analysis

With multiple runs, comprehensive statistics are provided:
- Mean, standard deviation, min/max values
- Percentiles (P50, P95, P99)
- Coefficient of variation for normalized variability

### System Information

Every run captures:
- CPU model and core count
- Memory capacity
- Operating system and version
- Hostname for cross-machine identification

## [0.1.0] - 2026-01-24

### Added

- **Initial Release**: Foundation of HsBenchMarkSuite
- CPU benchmarks: Prime calculation, matrix multiplication, Mandelbrot set, FFT
- Memory benchmarks: Sequential read/write throughput
- Disk benchmarks: File I/O read/write throughput
- Command-line argument parsing for customizable benchmark runs
- Statistical analysis: Mean, standard deviation, min/max, percentiles
- CSV and JSON output formats
- System information capture (CPU, memory, OS, hostname)
- Disclaimer about benchmark limitations
