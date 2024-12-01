/// Processes and transforms column data
pub(crate) struct ColumnProcessor;

impl ColumnProcessor {
    pub(crate) fn sort_columns(left: &mut Vec<u32>, right: &mut Vec<u32>) {
        left.sort();
        right.sort();
    }
}
