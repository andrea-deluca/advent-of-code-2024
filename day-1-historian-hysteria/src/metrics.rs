/// Calculates metrics for column data
pub(crate) struct Metrics;

impl Metrics {
    pub fn total_distance(left: &[u32], right: &[u32]) -> u32 {
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    pub(crate) fn similarity(left: &[u32], right: &[u32]) -> u32 {
        left.iter()
            .map(|l| l * right.iter().filter(|r| *r == l).count() as u32)
            .sum()
    }
}
