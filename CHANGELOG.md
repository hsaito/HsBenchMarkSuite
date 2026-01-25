# Changelog

All notable changes to HsBenchMarkSuite are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
