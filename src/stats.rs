/// Statistical analysis utilities for benchmark results
use std::fmt;

/// Statistical summary of benchmark results across multiple runs
#[derive(Debug, Clone)]
pub struct Statistics {
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub p50: f64, // Median
    pub p95: f64,
    pub p99: f64,
    pub coefficient_of_variation: f64, // std_dev / mean, expressed as percentage
}

impl Statistics {
    /// Calculate statistics from a slice of values
    pub fn from_values(values: &[f64]) -> Option<Self> {
        if values.is_empty() {
            return None;
        }

        let n = values.len();

        // Calculate mean
        let mean = values.iter().sum::<f64>() / n as f64;

        // Calculate standard deviation
        let variance = values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n as f64;
        let std_dev = variance.sqrt();

        // Find min and max
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Calculate percentiles (requires sorted data)
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p50 = percentile(&sorted, 50.0);
        let p95 = percentile(&sorted, 95.0);
        let p99 = percentile(&sorted, 99.0);

        // Coefficient of variation (as percentage)
        let coefficient_of_variation = if mean.abs() > f64::EPSILON {
            (std_dev / mean) * 100.0
        } else {
            0.0
        };

        Some(Statistics {
            mean,
            std_dev,
            min,
            max,
            p50,
            p95,
            p99,
            coefficient_of_variation,
        })
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mean: {:.2}, StdDev: {:.2}, Min: {:.2}, Max: {:.2}, P50: {:.2}, P95: {:.2}, P99: {:.2}, CV: {:.2}%",
            self.mean, self.std_dev, self.min, self.max, self.p50, self.p95, self.p99, self.coefficient_of_variation
        )
    }
}

/// Calculate percentile from sorted data
/// Uses linear interpolation between closest ranks
fn percentile(sorted_data: &[f64], p: f64) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }

    if sorted_data.len() == 1 {
        return sorted_data[0];
    }

    let rank = (p / 100.0) * (sorted_data.len() - 1) as f64;
    let lower_index = rank.floor() as usize;
    let upper_index = rank.ceil() as usize;

    if lower_index == upper_index {
        sorted_data[lower_index]
    } else {
        // Linear interpolation
        let lower_value = sorted_data[lower_index];
        let upper_value = sorted_data[upper_index];
        let fraction = rank - lower_index as f64;
        lower_value + (upper_value - lower_value) * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_basic() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = Statistics::from_values(&values).unwrap();

        assert!((stats.mean - 3.0).abs() < 0.01);
        assert!((stats.min - 1.0).abs() < 0.01);
        assert!((stats.max - 5.0).abs() < 0.01);
        assert!((stats.p50 - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_statistics_empty() {
        let values: Vec<f64> = vec![];
        let stats = Statistics::from_values(&values);
        assert!(stats.is_none());
    }

    #[test]
    fn test_statistics_single_value() {
        let values = vec![42.0];
        let stats = Statistics::from_values(&values).unwrap();

        assert!((stats.mean - 42.0).abs() < 0.01);
        assert!((stats.std_dev - 0.0).abs() < 0.01);
        assert!((stats.min - 42.0).abs() < 0.01);
        assert!((stats.max - 42.0).abs() < 0.01);
    }

    #[test]
    fn test_percentile_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        assert!((percentile(&data, 50.0) - 5.5).abs() < 0.01);
        assert!((percentile(&data, 95.0) - 9.55).abs() < 0.01);
        assert!((percentile(&data, 0.0) - 1.0).abs() < 0.01);
        assert!((percentile(&data, 100.0) - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_coefficient_of_variation() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let stats = Statistics::from_values(&values).unwrap();

        // Mean is 30, std dev should be ~14.14, CV should be ~47.14%
        assert!(stats.coefficient_of_variation > 40.0 && stats.coefficient_of_variation < 50.0);
    }

    #[test]
    fn test_percentile_edge_cases() {
        // Single element
        let data = vec![5.0];
        assert_eq!(percentile(&data, 50.0), 5.0);
        assert_eq!(percentile(&data, 0.0), 5.0);
        assert_eq!(percentile(&data, 100.0), 5.0);

        // Empty data
        let data: Vec<f64> = vec![];
        assert_eq!(percentile(&data, 50.0), 0.0);

        // Two elements
        let data = vec![1.0, 3.0];
        assert!((percentile(&data, 50.0) - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_percentile_interpolation() {
        let data = vec![1.0, 2.0, 3.0];
        // P50 should be exactly 2.0 (middle element)
        assert!((percentile(&data, 50.0) - 2.0).abs() < 0.01);
        // P25 should be interpolated between 1.0 and 2.0
        assert!((percentile(&data, 25.0) - 1.5).abs() < 0.01);
    }

    #[test]
    fn test_statistics_with_duplicates() {
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let stats = Statistics::from_values(&values).unwrap();

        assert!((stats.mean - 5.0).abs() < 0.01);
        assert!((stats.std_dev - 0.0).abs() < 0.01);
        assert!((stats.min - 5.0).abs() < 0.01);
        assert!((stats.max - 5.0).abs() < 0.01);
        assert!((stats.coefficient_of_variation - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_statistics_with_negative_values() {
        let values = vec![-10.0, -5.0, 0.0, 5.0, 10.0];
        let stats = Statistics::from_values(&values).unwrap();

        assert!((stats.mean - 0.0).abs() < 0.01);
        assert!((stats.min - (-10.0)).abs() < 0.01);
        assert!((stats.max - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_statistics_large_dataset() {
        let values: Vec<f64> = (1..=100).map(|x| x as f64).collect();
        let stats = Statistics::from_values(&values).unwrap();

        assert!((stats.mean - 50.5).abs() < 0.1);
        assert!((stats.min - 1.0).abs() < 0.01);
        assert!((stats.max - 100.0).abs() < 0.01);
        assert!((stats.p50 - 50.5).abs() < 0.1);
    }

    #[test]
    fn test_statistics_display() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = Statistics::from_values(&values).unwrap();
        let display = format!("{}", stats);

        assert!(display.contains("Mean:"));
        assert!(display.contains("StdDev:"));
        assert!(display.contains("Min:"));
        assert!(display.contains("Max:"));
        assert!(display.contains("P50:"));
        assert!(display.contains("CV:"));
    }

    #[test]
    fn test_statistics_two_values() {
        let values = vec![10.0, 20.0];
        let stats = Statistics::from_values(&values).unwrap();

        assert!((stats.mean - 15.0).abs() < 0.01);
        assert!((stats.min - 10.0).abs() < 0.01);
        assert!((stats.max - 20.0).abs() < 0.01);
        assert!(stats.std_dev > 0.0);
    }

    #[test]
    fn test_coefficient_of_variation_zero_mean() {
        // CV should handle near-zero mean gracefully
        let values = vec![-0.001, 0.0, 0.001];
        let stats = Statistics::from_values(&values).unwrap();
        // Should not panic or produce NaN
        assert!(stats.coefficient_of_variation.is_finite());
    }
}
