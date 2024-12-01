mod column_reader;
mod column_processor;
mod metrics;

use std::io::Error;
use clap::Parser;
use column_reader::ColumnReader;
use column_processor::ColumnProcessor;
use metrics::Metrics;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input file
    input_path: String,
}

/// Main entry point
fn main() {
    let args = Args::parse();
    match process_file(args.input_path) {
        Ok((distance, similarity)) => {
            println!("Distance: {}", distance);
            println!("Similarity: {}", similarity);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Orchestrates reading, processing, and computing metrics
fn process_file(file_path: String) -> Result<(u32, u32), Error> {
    let (mut left_column, mut right_column) = ColumnReader::new(file_path).read_columns()?;
    ColumnProcessor::sort_columns(&mut left_column, &mut right_column);
    let distance = Metrics::total_distance(&left_column, &right_column);
    let similarity = Metrics::similarity(&left_column, &right_column);
    Ok((distance, similarity))
}
