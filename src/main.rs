use chrono::Local;
use genbank_parser::{parse_sequence_record, Protein, Sequence};
use rayon::prelude::*;
use serde_json;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: genbank-parser <file_path>");
        return;
    }

    let file_path = &args[1];
    if !Path::new(file_path).exists() {
        println!("File not found: {}", file_path);
        return;
    }

    println!(
        "{} - reading sequences from file",
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
    );
    let (sequences, proteins) = read_and_process_genbank_file(&file_path).unwrap();
    println!(
        "{} - finished parsing (with {} sequences and {} proteins), writing to disk",
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        sequences.len(),
        proteins.len()
    );

    let sequences_json = serde_json::to_string(&sequences).unwrap();
    let proteins_json = serde_json::to_string(&proteins).unwrap();
    std::fs::write("sequences.json", sequences_json).expect("Failed to dump sequences.");
    std::fs::write("proteins.json", proteins_json).expect("Failed to dump proteins.");

    println!(
        "{} - finished",
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
    );
}

pub fn read_and_process_genbank_file(
    file_path: &str,
) -> std::io::Result<(Vec<Sequence>, Vec<Protein>)> {
    let contents = fs::read(file_path)?;

    let mut start_index = 0;
    let mut records = Vec::new();
    for (i, window) in contents.windows(6).enumerate() {
        if window.starts_with(b"\nLOCUS") {
            records.push(&contents[start_index..i + 1]);
            start_index = i + 1;
        }
    }

    if start_index < contents.len() {
        records.push(&contents[start_index..]);
    }

    println!(
        "{} - processing records",
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
    );

    let (sequences, proteins): (Vec<Sequence>, Vec<Vec<Protein>>) = records
        .par_iter()
        .skip(1) // first element of records is file header
        .map(|record| parse_sequence_record(record))
        .unzip();

    let proteins: Vec<Protein> = proteins.into_iter().flatten().collect();

    Ok((sequences, proteins))
}
