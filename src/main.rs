use chrono::Local;
use csv::WriterBuilder;
use genbank_parser;
use genbank_parser::{
    // faster::parse_new_sequence_record,
    // faster::NewProtein,
    // faster::NewSequence,
    parse_sequence_record,
    Protein,
    Sequence, // write_arrow_file
};
use memchr::memmem;
use rayon::prelude::*;
use serde_json::to_writer_pretty;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cli-tool <path>");
        return;
    }

    let path_input = &args[1];
    let path = Path::new(path_input);

    if !path.exists() {
        println!("Path not found: {}", path_input);
        return;
    }

    let mut file_paths = Vec::new();
    let extension = "seq";

    if path.is_file() {
        if path.extension().and_then(std::ffi::OsStr::to_str) == Some(extension) {
            file_paths.push(path.to_path_buf());
        } else {
            println!("The file does not have the .{} extension", extension);
            return;
        }
    } else if path.is_dir() {
        file_paths = find_files_with_extension(path_input, extension);
    } else {
        println!("The path is neither a file nor a directory");
        return;
    }

    if file_paths.is_empty() {
        println!("No .{} files found at the specified path", extension);
    } else {
        println!(
            "Found {} file(s) with .{} extension:",
            file_paths.len(),
            extension
        );
        for file_path in file_paths {
            // println!("{}", file_path.display());
            if let Some(file_name) = file_path.file_name().and_then(|name| name.to_str()) {
                println!(
                    "{} - reading sequences from {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    file_name
                );

                let (nucleotides, proteins) = read_and_process_genbank_file(&file_path).unwrap();

                println!(
                    "{} - finished parsing (with {} nucleotides and {} proteins), writing to disk",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    nucleotides.len(),
                    proteins.len()
                );

                serialize_nucleotides(&nucleotides, file_name).expect("Failed to save nucleotide sequences");
                serialize_proteins(&proteins, file_name).expect("Failed to save protein sequences");

                // write_arrow_file(&sequences, "sequences_2.arrow")
                //     .expect("Faild to write viral sequences to arrow");
                // write_arrow_file(&proteins, "proteins_2.arrow")
                //     .expect("Faild to write protein sequences to arrow");

                println!(
                    "{} - finished writing to disk",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
                );
            }
        }
    }
}

fn find_files_with_extension(dir: &str, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some(ext) {
                files.push(path);
            }
        }
    }

    files
}

fn serialize_nucleotides(
    sequences: &[Sequence],
    source_file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let csv_path = format!(
        "nucleotides_{}.csv",
        source_file_name.trim_end_matches(".seq")
    );
    let json_path = format!(
        "nucleotides_{}.json",
        source_file_name.trim_end_matches(".seq")
    );

    let json_data: Vec<(String, String)> = sequences
        .par_iter()
        .map(|seq| {
            (
                String::from_utf8_lossy(&seq.version).into_owned(),
                String::from_utf8_lossy(&seq.sequence).into_owned(),
            )
        })
        .collect();
    let json_file = File::create(json_path)?;
    let mut writer = BufWriter::new(json_file);
    to_writer_pretty(&mut writer, &json_data)?;
    writer.flush()?;

    let csv_file = File::create(csv_path)?;
    let mut writer = BufWriter::new(csv_file);
    let mut csv_writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(&mut writer);

    csv_writer.write_record(&[
        "accession",
        "definition",
        "organism",
        "taxonomy",
        "host",
        "mol_type",
    ])?;
    let csv_records: Vec<_> = sequences
        .par_iter()
        .map(|seq| {
            vec![
                String::from_utf8_lossy(&seq.version).into_owned(),
                String::from_utf8_lossy(&seq.definition).into_owned(),
                String::from_utf8_lossy(&seq.organism).into_owned(),
                String::from_utf8_lossy(&seq.taxonomy).into_owned(),
                String::from_utf8_lossy(&seq.host).into_owned(),
                String::from_utf8_lossy(&seq.mol_type).into_owned(),
            ]
        })
        .collect();

    for record in csv_records {
        csv_writer.write_record(&record)?;
    }

    csv_writer.flush()?;

    Ok(())
}

fn serialize_proteins(
    proteins: &[Protein],
    source_file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let csv_path = format!("proteins_{}.csv", source_file_name.trim_end_matches(".seq"));
    let json_path = format!(
        "proteins_{}.json",
        source_file_name.trim_end_matches(".seq")
    );

    let json_data: Vec<(String, String)> = proteins
        .par_iter()
        .map(|prot| {
            (
                String::from_utf8_lossy(&prot.protein_id).into_owned(),
                String::from_utf8_lossy(&prot.sequence).into_owned(),
            )
        })
        .collect();
    let json_file = File::create(json_path)?;
    let mut writer = BufWriter::new(json_file);
    to_writer_pretty(&mut writer, &json_data)?;
    writer.flush()?;

    let csv_file = File::create(csv_path)?;
    let mut writer = BufWriter::new(csv_file);
    let mut csv_writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(&mut writer);

    csv_writer.write_record(&["accession", "location", "source"])?;
    let csv_records: Vec<_> = proteins
        .par_iter()
        .map(|prot| {
            vec![
                String::from_utf8_lossy(&prot.protein_id).into_owned(),
                String::from_utf8_lossy(&prot.location).into_owned(),
                String::from_utf8_lossy(&prot.source_id).into_owned(),
            ]
        })
        .collect();

    for record in csv_records {
        csv_writer.write_record(&record)?;
    }

    csv_writer.flush()?;

    Ok(())
}

fn chunk_contents_with_memchr<'a>(contents: &'a Vec<u8>) -> Vec<&'a [u8]> {
    let mut records: Vec<&[u8]> = Vec::new();
    let needle = b"\nLOCUS";

    let mut last_pos = 0;
    while let Some(pos) = memmem::find(&contents[last_pos..], needle) {
        let chunk = &contents[last_pos..last_pos + pos];
        records.push(chunk);
        last_pos += pos + needle.len();
    }

    if last_pos < contents.len() {
        let chunk = &contents[last_pos..];
        records.push(chunk);
    }

    records
}

pub fn read_and_process_genbank_file(
    file_path: &PathBuf,
) -> std::io::Result<(Vec<Sequence>, Vec<Protein>)> {
    let contents = fs::read(file_path)?;
    let records = chunk_contents_with_memchr(&contents);

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

// pub fn new_read_and_process_genbank_file<'a>(
//     contents: &'a Vec<u8>,
// ) -> std::io::Result<(Vec<NewSequence<'a>>, Vec<NewProtein<'a>>)> {
//     let records = chunk_contents_with_memchr(contents);

//     println!(
//         "{} - processing records",
//         Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
//     );

//     let (sequences, proteins): (Vec<NewSequence>, Vec<Vec<NewProtein>>) = records
//         .par_iter()
//         .skip(1) // first element of records is file header
//         .map(|record| parse_new_sequence_record(record))
//         .unzip();

//     let proteins: Vec<NewProtein> = proteins.into_iter().flatten().collect();

//     Ok((sequences, proteins))
// }
