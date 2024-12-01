use std::fs::File;
use std::io::{BufRead, BufReader};

/// Handles reading columns from the input file
pub(crate) struct ColumnReader {
    file_path: String,
}

impl ColumnReader {
    pub(crate) fn new(file_path: String) -> Self {
        Self { file_path }
    }

    fn get_reader(&self) -> std::io::Result<BufReader<File>> {
        let file = File::open(self.file_path.trim())?;
        Ok(BufReader::new(file))
    }

    pub(crate) fn read_columns(&self) -> std::io::Result<(Vec<u32>, Vec<u32>)> {
        let reader = self.get_reader()?;
        let (mut left, mut right) = (Vec::new(), Vec::new());

        for line in reader.lines() {
            let line = line?;
            let mut columns = line
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok());

            if let (Some(l), Some(r)) = (columns.next(), columns.next()) {
                left.push(l);
                right.push(r);
            }
        }

        Ok((left, right))
    }
}
