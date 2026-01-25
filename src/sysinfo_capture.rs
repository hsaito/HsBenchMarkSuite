/// System information capture for benchmark context
use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub cpu_brand: String,
    pub cpu_physical_cores: usize,
    pub cpu_logical_cores: usize,
    pub total_memory_mb: u64,
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
}

impl SystemInfo {
    /// Capture current system information
    pub fn capture() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_brand = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        let cpu_physical_cores = System::physical_core_count().unwrap_or(0);
        let cpu_logical_cores = sys.cpus().len();
        let total_memory_mb = sys.total_memory() / (1024 * 1024);

        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

        SystemInfo {
            cpu_brand,
            cpu_physical_cores,
            cpu_logical_cores,
            total_memory_mb,
            os_name,
            os_version,
            hostname,
        }
    }

    /// Display formatted system information
    pub fn display(&self) {
        println!("=== System Information ===");
        println!("CPU: {}", self.cpu_brand);
        println!(
            "Cores: {} physical, {} logical",
            self.cpu_physical_cores, self.cpu_logical_cores
        );
        println!("Memory: {} MB", self.total_memory_mb);
        println!("OS: {} {}", self.os_name, self.os_version);
        println!("Hostname: {}\n", self.hostname);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_capture() {
        let info = SystemInfo::capture();

        // Basic sanity checks
        assert!(!info.cpu_brand.is_empty());
        assert!(info.cpu_logical_cores > 0);
        assert!(info.total_memory_mb > 0);
        assert!(!info.os_name.is_empty());
    }

    #[test]
    fn test_system_info_fields() {
        let info = SystemInfo::capture();

        // Verify all fields are populated
        assert!(!info.cpu_brand.is_empty());
        assert!(info.cpu_physical_cores <= info.cpu_logical_cores);
        assert!(info.total_memory_mb > 0);
        assert!(!info.os_name.is_empty());
        assert!(!info.os_version.is_empty());
        assert!(!info.hostname.is_empty());
    }

    #[test]
    fn test_system_info_clone() {
        let info = SystemInfo::capture();
        let cloned = info.clone();

        assert_eq!(info.cpu_brand, cloned.cpu_brand);
        assert_eq!(info.cpu_physical_cores, cloned.cpu_physical_cores);
        assert_eq!(info.cpu_logical_cores, cloned.cpu_logical_cores);
        assert_eq!(info.total_memory_mb, cloned.total_memory_mb);
    }

    #[test]
    fn test_system_info_debug() {
        let info = SystemInfo::capture();
        let debug_str = format!("{:?}", info);

        assert!(debug_str.contains("SystemInfo"));
        assert!(debug_str.contains("cpu_brand"));
    }
}
