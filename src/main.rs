/// HsBenchMarkSuite - Rust Performance Benchmark Suite
///
/// DISCLAIMER:
/// This benchmark suite provides runtime metrics for synthetic test scenarios.
/// Results represent performance on specific algorithms (prime calculations, matrix operations, sequential I/O)
/// and do NOT necessarily equate to actual system capability for real-world workloads.
///
/// Key limitations:
/// - Tests controlled, synthetic scenarios that may not represent production usage
/// - Does not account for complex caching effects, kernel scheduling, or variable system load
/// - Results are single point-in-time measurements under specific conditions
/// - Should be used as ONE OF MANY DATA POINTS, not as the sole basis for system evaluation
///
/// Use these results to understand relative performance characteristics, but do NOT rely solely
/// on these benchmarks for critical system purchasing, deployment, or performance guarantees.
mod args;
mod board_game;
mod cpu;
mod disk;
mod memory;
mod stats;
mod sysinfo_capture;

use args::BenchmarkArgs;
use chrono::Local;
use cpu::CpuResult;
use disk::DiskResult;
use memory::MemoryResult;
use stats::Statistics;
use std::time::Instant;
use sysinfo_capture::SystemInfo;

struct BenchmarkResults {
    cpu: Vec<CpuResult>,
    memory: Vec<MemoryResult>,
    disk: Vec<DiskResult>,
}

fn main() {
    let cli_args = BenchmarkArgs::parse();

    // Easter egg: board_game
    if cli_args.board_game {
        board_game::run_board_game();
        return;
    }

    // Display disclaimer
    println!("DISCLAIMER: Benchmark Results vs Actual System Capability");
    println!("These results reflect runtime metrics for synthetic test scenarios");
    println!("and do NOT necessarily equate to actual system capability for");
    println!("real-world workloads. Use these results as one of many data points,");
    println!("not as the sole basis for system evaluation.\n");

    // Capture system information
    let system_info = SystemInfo::capture();
    system_info.display();

    println!("=== Benchmark Configuration ===");
    println!("Scale: {}", cli_args.scale);
    println!("Runs: {}", cli_args.count);
    println!("Threads: {}\n", cli_args.threads);

    let mut results = BenchmarkResults {
        cpu: Vec::new(),
        memory: Vec::new(),
        disk: Vec::new(),
    };

    // Run benchmarks multiple times
    for run in 1..=cli_args.count {
        println!("--- Run {} ---", run);

        // CPU Benchmark
        println!("Running CPU Benchmark...");
        let cpu_start = Instant::now();
        let cpu_result = cpu::run_cpu_benchmark_scaled(cli_args.scale, cli_args.threads);
        let cpu_duration = cpu_start.elapsed();
        println!(
            "CPU Primes:              {:.0} primes/sec",
            cpu_result.primes_per_sec
        );
        println!(
            "CPU Matrix Mult (ST):    {:.2} GFLOPS",
            cpu_result.matrix_mult_gflops
        );
        println!(
            "CPU Matrix Mult (MT):    {:.2} GFLOPS",
            cpu_result.parallel_matrix_gflops
        );
        println!(
            "CPU Speedup (MT):        {:.2}x",
            cpu_result.parallel_speedup
        );
        println!(
            "CPU Mandelbrot:          {:.0} pixels/sec",
            cpu_result.mandelbrot_pixels_per_sec
        );
        println!(
            "CPU FFT:                 {:.0} Msamples/sec",
            cpu_result.fft_msamples_per_sec
        );
        results.cpu.push(cpu_result);
        println!("Duration: {:?}\n", cpu_duration);

        // Memory Benchmark
        println!("Running Memory Benchmark...");
        let mem_start = Instant::now();
        let mem_result = memory::run_memory_benchmark_scaled(cli_args.scale);
        let mem_duration = mem_start.elapsed();
        println!("Memory Write: {:.2} MB/s", mem_result.write_throughput);
        println!("Memory Read:  {:.2} MB/s", mem_result.read_throughput);
        println!("Memory Avg:   {:.2} MB/s", mem_result.combined_throughput);
        results.memory.push(mem_result);
        println!("Duration: {:?}\n", mem_duration);

        // Disk Benchmark
        println!("Running Disk Benchmark...");
        let disk_start = Instant::now();
        let disk_result =
            disk::run_disk_benchmark_scaled_with_block_size(cli_args.scale, cli_args.block_size);
        let disk_duration = disk_start.elapsed();
        println!("Disk Write: {:.2} MB/s", disk_result.write_throughput);
        println!("Disk Read:  {:.2} MB/s", disk_result.read_throughput);
        println!("Disk Avg:   {:.2} MB/s", disk_result.combined_throughput);
        results.disk.push(disk_result);
        println!("Duration: {:?}\n", disk_duration);
    }

    // Display summary with averages if multiple runs
    if cli_args.count > 1 {
        println!("=== Summary ===\n");

        println!("CPU Benchmark:");
        for (i, result) in results.cpu.iter().enumerate() {
            println!("  Run {}:", i + 1);
            println!(
                "    Primes:              {:.0} primes/sec",
                result.primes_per_sec
            );
            println!(
                "    Matrix Mult (ST):    {:.2} GFLOPS",
                result.matrix_mult_gflops
            );
            println!(
                "    Matrix Mult (MT):    {:.2} GFLOPS",
                result.parallel_matrix_gflops
            );
            println!("    Speedup (MT):        {:.2}x", result.parallel_speedup);
            println!(
                "    Mandelbrot:          {:.0} pixels/sec",
                result.mandelbrot_pixels_per_sec
            );
            println!(
                "    FFT:                 {:.0} Msamples/sec",
                result.fft_msamples_per_sec
            );
        }
        let cpu_primes_avg =
            results.cpu.iter().map(|r| r.primes_per_sec).sum::<f64>() / results.cpu.len() as f64;
        let cpu_matrix_avg = results
            .cpu
            .iter()
            .map(|r| r.matrix_mult_gflops)
            .sum::<f64>()
            / results.cpu.len() as f64;
        let cpu_parallel_avg = results
            .cpu
            .iter()
            .map(|r| r.parallel_matrix_gflops)
            .sum::<f64>()
            / results.cpu.len() as f64;
        let cpu_speedup_avg =
            results.cpu.iter().map(|r| r.parallel_speedup).sum::<f64>() / results.cpu.len() as f64;
        let cpu_mandelbrot_avg = results
            .cpu
            .iter()
            .map(|r| r.mandelbrot_pixels_per_sec)
            .sum::<f64>()
            / results.cpu.len() as f64;
        let cpu_fft_avg = results
            .cpu
            .iter()
            .map(|r| r.fft_msamples_per_sec)
            .sum::<f64>()
            / results.cpu.len() as f64;
        println!("  Average:");
        println!("    Primes:              {:.0} primes/sec", cpu_primes_avg);
        println!("    Matrix Mult (ST):    {:.2} GFLOPS", cpu_matrix_avg);
        println!("    Matrix Mult (MT):    {:.2} GFLOPS", cpu_parallel_avg);
        println!("    Speedup (MT):        {:.2}x", cpu_speedup_avg);
        println!(
            "    Mandelbrot:          {:.0} pixels/sec",
            cpu_mandelbrot_avg
        );
        println!("    FFT:                 {:.0} Msamples/sec\n", cpu_fft_avg);

        println!("Memory Benchmark:");
        for (i, result) in results.memory.iter().enumerate() {
            println!("  Run {}:", i + 1);
            println!("    Write: {:.2} MB/s", result.write_throughput);
            println!("    Read:  {:.2} MB/s", result.read_throughput);
            println!("    Avg:   {:.2} MB/s", result.combined_throughput);
        }
        let mem_write_avg = results
            .memory
            .iter()
            .map(|r| r.write_throughput)
            .sum::<f64>()
            / results.memory.len() as f64;
        let mem_read_avg = results
            .memory
            .iter()
            .map(|r| r.read_throughput)
            .sum::<f64>()
            / results.memory.len() as f64;
        let mem_combined_avg = results
            .memory
            .iter()
            .map(|r| r.combined_throughput)
            .sum::<f64>()
            / results.memory.len() as f64;
        println!("  Average:");
        println!("    Write: {:.2} MB/s", mem_write_avg);
        println!("    Read:  {:.2} MB/s", mem_read_avg);
        println!("    Avg:   {:.2} MB/s\n", mem_combined_avg);

        println!("Disk Benchmark:");
        for (i, result) in results.disk.iter().enumerate() {
            println!("  Run {}:", i + 1);
            println!("    Write: {:.2} MB/s", result.write_throughput);
            println!("    Read:  {:.2} MB/s", result.read_throughput);
            println!("    Avg:   {:.2} MB/s", result.combined_throughput);
        }
        let disk_write_avg = results.disk.iter().map(|r| r.write_throughput).sum::<f64>()
            / results.disk.len() as f64;
        let disk_read_avg =
            results.disk.iter().map(|r| r.read_throughput).sum::<f64>() / results.disk.len() as f64;
        let disk_combined_avg = results
            .disk
            .iter()
            .map(|r| r.combined_throughput)
            .sum::<f64>()
            / results.disk.len() as f64;
        println!("  Average:");
        println!("    Write: {:.2} MB/s", disk_write_avg);
        println!("    Read:  {:.2} MB/s", disk_read_avg);
        println!("    Avg:   {:.2} MB/s\n", disk_combined_avg);
    }

    // Write CSV output if requested
    if cli_args.csv {
        if let Err(e) = write_csv_report(&cli_args, &results, &system_info) {
            eprintln!("Error writing CSV report: {}", e);
        } else {
            let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
            println!("CSV report written to output_{}.csv", timestamp);
        }
    }

    // Write JSON output if requested
    if cli_args.json {
        if let Err(e) = write_json_report(&cli_args, &results, &system_info) {
            eprintln!("Error writing JSON report: {}", e);
        } else {
            let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
            println!("JSON report written to output_{}.json", timestamp);
        }
    }

    println!("=== Benchmark Complete ===");
}

fn write_csv_report(
    _args: &BenchmarkArgs,
    results: &BenchmarkResults,
    _system_info: &SystemInfo,
) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("output_{}.csv", timestamp);
    let mut file = File::create(&filename)?;

    // Write header with individual runs and statistics
    let mut header = vec!["Metric".to_string()];
    for i in 1..=results.cpu.len() {
        header.push(format!("Run {}", i));
    }
    header.extend_from_slice(&[
        "Mean".to_string(),
        "StdDev".to_string(),
        "Min".to_string(),
        "Max".to_string(),
        "P50".to_string(),
        "P95".to_string(),
        "P99".to_string(),
        "CV%".to_string(),
    ]);
    writeln!(file, "{}", header.join(","))?;

    // Helper function to write metric with stats
    let write_metric = |file: &mut File, name: &str, values: Vec<f64>| -> std::io::Result<()> {
        let mut row = vec![name.to_string()];
        for val in &values {
            row.push(format!("{:.2}", val));
        }

        // Calculate and append statistics
        if let Some(stats) = Statistics::from_values(&values) {
            row.push(format!("{:.2}", stats.mean));
            row.push(format!("{:.2}", stats.std_dev));
            row.push(format!("{:.2}", stats.min));
            row.push(format!("{:.2}", stats.max));
            row.push(format!("{:.2}", stats.p50));
            row.push(format!("{:.2}", stats.p95));
            row.push(format!("{:.2}", stats.p99));
            row.push(format!("{:.2}", stats.coefficient_of_variation));
        }

        writeln!(file, "{}", row.join(","))
    };

    // CPU metrics
    write_metric(
        &mut file,
        "CPU Primes (primes/sec)",
        results.cpu.iter().map(|r| r.primes_per_sec).collect(),
    )?;

    write_metric(
        &mut file,
        "CPU Matrix ST (GFLOPS)",
        results.cpu.iter().map(|r| r.matrix_mult_gflops).collect(),
    )?;

    write_metric(
        &mut file,
        "CPU Matrix MT (GFLOPS)",
        results
            .cpu
            .iter()
            .map(|r| r.parallel_matrix_gflops)
            .collect(),
    )?;

    write_metric(
        &mut file,
        "CPU Speedup (MT)",
        results.cpu.iter().map(|r| r.parallel_speedup).collect(),
    )?;

    write_metric(
        &mut file,
        "CPU Mandelbrot (pixels/sec)",
        results
            .cpu
            .iter()
            .map(|r| r.mandelbrot_pixels_per_sec)
            .collect(),
    )?;

    write_metric(
        &mut file,
        "CPU FFT (Msamples/sec)",
        results.cpu.iter().map(|r| r.fft_msamples_per_sec).collect(),
    )?;

    // Memory metrics
    write_metric(
        &mut file,
        "Memory Write (MB/s)",
        results.memory.iter().map(|r| r.write_throughput).collect(),
    )?;

    write_metric(
        &mut file,
        "Memory Read (MB/s)",
        results.memory.iter().map(|r| r.read_throughput).collect(),
    )?;

    write_metric(
        &mut file,
        "Memory Combined (MB/s)",
        results
            .memory
            .iter()
            .map(|r| r.combined_throughput)
            .collect(),
    )?;

    // Disk metrics
    write_metric(
        &mut file,
        "Disk Write (MB/s)",
        results.disk.iter().map(|r| r.write_throughput).collect(),
    )?;

    write_metric(
        &mut file,
        "Disk Read (MB/s)",
        results.disk.iter().map(|r| r.read_throughput).collect(),
    )?;

    write_metric(
        &mut file,
        "Disk Combined (MB/s)",
        results.disk.iter().map(|r| r.combined_throughput).collect(),
    )?;

    Ok(())
}

fn write_json_report(
    args: &BenchmarkArgs,
    results: &BenchmarkResults,
    system_info: &SystemInfo,
) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let iso_timestamp = Local::now().to_rfc3339();
    let filename = format!("output_{}.json", timestamp);
    let mut file = File::create(&filename)?;

    // Helper to create stats JSON
    let stats_json = |values: &[f64]| -> String {
        if let Some(stats) = Statistics::from_values(values) {
            format!(
                r#"{{"mean":{:.2},"std_dev":{:.2},"min":{:.2},"max":{:.2},"p50":{:.2},"p95":{:.2},"p99":{:.2},"cv_percent":{:.2}}}"#,
                stats.mean,
                stats.std_dev,
                stats.min,
                stats.max,
                stats.p50,
                stats.p95,
                stats.p99,
                stats.coefficient_of_variation
            )
        } else {
            "null".to_string()
        }
    };

    writeln!(file, "{{")?;

    // Metadata (timestamp and hostname for easy comparison)
    writeln!(file, r#"  "metadata": {{"#)?;
    writeln!(file, r#"    "timestamp": "{}","#, iso_timestamp)?;
    writeln!(
        file,
        r#"    "hostname": "{}""#,
        system_info.hostname.replace("\"", "\\\"")
    )?;
    writeln!(file, "  }},")?;

    // System information
    writeln!(file, r#"  "system_info": {{"#)?;
    writeln!(
        file,
        r#"    "cpu_brand": "{}","#,
        system_info.cpu_brand.replace("\"", "\\\"")
    )?;
    writeln!(
        file,
        r#"    "cpu_physical_cores": {},"#,
        system_info.cpu_physical_cores
    )?;
    writeln!(
        file,
        r#"    "cpu_logical_cores": {},"#,
        system_info.cpu_logical_cores
    )?;
    writeln!(
        file,
        r#"    "total_memory_mb": {},"#,
        system_info.total_memory_mb
    )?;
    writeln!(
        file,
        r#"    "os_name": "{}","#,
        system_info.os_name.replace("\"", "\\\"")
    )?;
    writeln!(
        file,
        r#"    "os_version": "{}","#,
        system_info.os_version.replace("\"", "\\\"")
    )?;
    writeln!(
        file,
        r#"    "hostname": "{}""#,
        system_info.hostname.replace("\"", "\\\"")
    )?;
    writeln!(file, "  }},")?;

    // Benchmark configuration
    writeln!(file, r#"  "configuration": {{"#)?;
    writeln!(file, r#"    "scale": {},"#, args.scale)?;
    writeln!(file, r#"    "runs": {},"#, args.count)?;
    writeln!(file, r#"    "threads": {},"#, args.threads)?;
    writeln!(file, r#"    "block_size": {}"#, args.block_size)?;
    writeln!(file, "  }},")?;

    // Results
    writeln!(file, r#"  "results": {{"#)?;

    // CPU results
    writeln!(file, r#"    "cpu": {{"#)?;
    let cpu_primes: Vec<f64> = results.cpu.iter().map(|r| r.primes_per_sec).collect();
    writeln!(file, r#"      "cpu_primes_per_sec": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        cpu_primes
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(file, r#"        "statistics": {}"#, stats_json(&cpu_primes))?;
    writeln!(file, "      }},")?;

    let cpu_matrix: Vec<f64> = results.cpu.iter().map(|r| r.matrix_mult_gflops).collect();
    writeln!(file, r#"      "cpu_matrix_mult_gflops_st": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        cpu_matrix
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(file, r#"        "statistics": {}"#, stats_json(&cpu_matrix))?;
    writeln!(file, "      }},")?;

    let cpu_parallel: Vec<f64> = results
        .cpu
        .iter()
        .map(|r| r.parallel_matrix_gflops)
        .collect();
    writeln!(file, r#"      "cpu_matrix_mult_gflops_mt": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        cpu_parallel
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(
        file,
        r#"        "statistics": {}"#,
        stats_json(&cpu_parallel)
    )?;
    writeln!(file, "      }},")?;

    let cpu_speedup: Vec<f64> = results.cpu.iter().map(|r| r.parallel_speedup).collect();
    writeln!(file, r#"      "cpu_parallel_speedup_mt": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        cpu_speedup
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(
        file,
        r#"        "statistics": {}"#,
        stats_json(&cpu_speedup)
    )?;
    writeln!(file, "      }},")?;

    let cpu_mandelbrot: Vec<f64> = results
        .cpu
        .iter()
        .map(|r| r.mandelbrot_pixels_per_sec)
        .collect();
    writeln!(file, r#"      "cpu_mandelbrot_pixels_per_sec": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        cpu_mandelbrot
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(
        file,
        r#"        "statistics": {}"#,
        stats_json(&cpu_mandelbrot)
    )?;
    writeln!(file, "      }},")?;

    let cpu_fft: Vec<f64> = results.cpu.iter().map(|r| r.fft_msamples_per_sec).collect();
    writeln!(file, r#"      "cpu_fft_msamples_per_sec": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        cpu_fft
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(file, r#"        "statistics": {}"#, stats_json(&cpu_fft))?;
    writeln!(file, "      }}")?;
    writeln!(file, "    }},")?;

    // Memory results
    writeln!(file, r#"    "memory": {{"#)?;
    let mem_write: Vec<f64> = results.memory.iter().map(|r| r.write_throughput).collect();
    writeln!(file, r#"      "memory_write_throughput_mbs": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        mem_write
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(file, r#"        "statistics": {}"#, stats_json(&mem_write))?;
    writeln!(file, "      }},")?;

    let mem_read: Vec<f64> = results.memory.iter().map(|r| r.read_throughput).collect();
    writeln!(file, r#"      "memory_read_throughput_mbs": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        mem_read
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(file, r#"        "statistics": {}"#, stats_json(&mem_read))?;
    writeln!(file, "      }},")?;

    let mem_combined: Vec<f64> = results
        .memory
        .iter()
        .map(|r| r.combined_throughput)
        .collect();
    writeln!(file, r#"      "memory_combined_throughput_mbs": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        mem_combined
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(
        file,
        r#"        "statistics": {}"#,
        stats_json(&mem_combined)
    )?;
    writeln!(file, "      }}")?;
    writeln!(file, "    }},")?;

    // Disk results
    writeln!(file, r#"    "disk": {{"#)?;
    let disk_write: Vec<f64> = results.disk.iter().map(|r| r.write_throughput).collect();
    writeln!(file, r#"      "disk_write_throughput_mbs": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        disk_write
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(file, r#"        "statistics": {}"#, stats_json(&disk_write))?;
    writeln!(file, "      }},")?;

    let disk_read: Vec<f64> = results.disk.iter().map(|r| r.read_throughput).collect();
    writeln!(file, r#"      "disk_read_throughput_mbs": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        disk_read
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(file, r#"        "statistics": {}"#, stats_json(&disk_read))?;
    writeln!(file, "      }},")?;

    let disk_combined: Vec<f64> = results.disk.iter().map(|r| r.combined_throughput).collect();
    writeln!(file, r#"      "disk_combined_throughput_mbs": {{"#)?;
    writeln!(
        file,
        r#"        "runs": [{}],"#,
        disk_combined
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(",")
    )?;
    writeln!(
        file,
        r#"        "statistics": {}"#,
        stats_json(&disk_combined)
    )?;
    writeln!(file, "      }}")?;
    writeln!(file, "    }}")?;

    writeln!(file, "  }}")?;
    writeln!(file, "}}")?;

    Ok(())
}
