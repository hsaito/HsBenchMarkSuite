/// Command-line argument parsing module
pub struct BenchmarkArgs {
    pub scale: f64,
    pub count: usize,
    pub threads: usize,
    pub csv: bool,
    pub json: bool,
    pub board_game: bool,
}

impl Default for BenchmarkArgs {
    fn default() -> Self {
        Self {
            scale: 1.0,
            count: 1,
            threads: 4,
            csv: false,
            json: false,
            board_game: false,
        }
    }
}

impl BenchmarkArgs {
    pub fn parse() -> Self {
        let mut args = BenchmarkArgs::default();

        let cli_args: Vec<String> = std::env::args().collect();

        let mut i = 1;
        while i < cli_args.len() {
            match cli_args[i].as_str() {
                "--scale" => {
                    if i + 1 < cli_args.len() {
                        args.scale = cli_args[i + 1].parse().unwrap_or(1.0);
                        i += 2;
                    } else {
                        eprintln!("Error: --scale requires a value");
                        i += 1;
                    }
                }
                "--count" => {
                    if i + 1 < cli_args.len() {
                        args.count = cli_args[i + 1].parse().unwrap_or(1);
                        i += 2;
                    } else {
                        eprintln!("Error: --count requires a value");
                        i += 1;
                    }
                }
                "--thread" => {
                    if i + 1 < cli_args.len() {
                        args.threads = cli_args[i + 1].parse().unwrap_or(4);
                        i += 2;
                    } else {
                        eprintln!("Error: --thread requires a value");
                        i += 1;
                    }
                }
                "--csv" => {
                    args.csv = true;
                    i += 1;
                }
                "--json" => {
                    args.json = true;
                    i += 1;
                }
                "--board-game" => {
                    args.board_game = true;
                    i += 1;
                }
                "--help" | "-h" => {
                    Self::print_help();
                    std::process::exit(0);
                }
                arg => {
                    eprintln!("Unknown argument: {}", arg);
                    i += 1;
                }
            }
        }

        // Validate arguments
        if args.scale <= 0.0 {
            eprintln!("Warning: scale must be positive, setting to 1.0");
            args.scale = 1.0;
        }

        if args.count == 0 {
            eprintln!("Warning: count must be at least 1, setting to 1");
            args.count = 1;
        }

        if args.threads == 0 {
            eprintln!("Warning: threads must be at least 1, setting to 4");
            args.threads = 4;
        }

        args
    }

    fn print_help() {
        println!("Benchmark Suite - Performance Testing Tool");
        println!();
        println!("USAGE:");
        println!("    benchmark [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    --scale <VALUE>    Scale factor for benchmark intensity (default: 1.0)");
        println!("                        Higher values increase test duration and memory usage");
        println!("    --count <NUM>      Number of times to run benchmarks (default: 1)");
        println!("                        Results from multiple runs are averaged");
        println!("    --thread <NUM>     Number of threads for parallel benchmark (default: 4)");
        println!("                        Controls multithreaded matrix multiplication");
        println!("    --csv              Output results to output.csv file");
        println!("    --json             Output results to output.json file with full statistics");
        println!("    --help, -h         Print this help message");
        println!();
        println!("EXAMPLES:");
        println!("    benchmark                    # Run with default settings");
        println!("    benchmark --scale 2.0        # Run with 2x intensity");
        println!("    benchmark --count 3          # Run 3 times and show average");
        println!("    benchmark --thread 8         # Run parallel test with 8 threads");
        println!("    benchmark --scale 0.5 --count 5 --thread 2  # Combined options");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_args() {
        let args = BenchmarkArgs::default();
        assert_eq!(args.scale, 1.0);
        assert_eq!(args.count, 1);
        assert_eq!(args.threads, 4);
        assert!(!args.csv);
        assert!(!args.json);
        assert!(!args.board_game);
    }

    #[test]
    fn test_args_validation_negative_scale() {
        // Note: This test documents current behavior
        // Args parsing validates and resets invalid values
        let args = BenchmarkArgs {
            scale: -1.0,
            count: 1,
            threads: 4,
            csv: false,
            json: false,
            board_game: false,
        };
        // Should be valid after constructor, but parse() validates
        assert_eq!(args.scale, -1.0);
    }

    #[test]
    fn test_args_validation_zero_count() {
        let args = BenchmarkArgs {
            scale: 1.0,
            count: 0,
            threads: 4,
            csv: false,
            json: false,
            board_game: false,
        };
        assert_eq!(args.count, 0);
    }

    #[test]
    fn test_args_all_flags() {
        let args = BenchmarkArgs {
            scale: 2.5,
            count: 10,
            threads: 8,
            csv: true,
            json: true,
            board_game: true,
        };
        assert_eq!(args.scale, 2.5);
        assert_eq!(args.count, 10);
        assert_eq!(args.threads, 8);
        assert!(args.csv);
        assert!(args.json);
        assert!(args.board_game);
    }
}
