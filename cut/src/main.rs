use std::io::{self, BufRead};

use anyhow::bail;
use clap::Parser;
use cli::Cli;
use csv::{ReaderBuilder, WriterBuilder};
use extract::{extract_bytes, extract_chars, extract_fields, Extract};
use file::open;
use parser::parse_position;

mod cli;
mod extract;
mod file;
mod parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let delimiter_bytes = cli.delimiter.as_bytes();
    if delimiter_bytes.len() != 1 {
        bail!("--delimiter \"{}\" must be a single byte", cli.delimiter);
    }
    let delimiter = *delimiter_bytes.first().unwrap();

    let extract_arg = if let Some(fields) = cli.extract.fields.map(parse_position).transpose()? {
        Extract::Fields(fields)
    } else if let Some(bytes) = cli.extract.bytes.map(parse_position).transpose()? {
        Extract::Bytes(bytes)
    } else if let Some(chars) = cli.extract.chars.map(parse_position).transpose()? {
        Extract::Chars(chars)
    } else {
        unreachable!("Must have --fields, --bytes, or --chars")
    };

    for filename in &cli.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(f) => match &extract_arg {
                Extract::Fields(field_pos) => {
                    let mut reader = ReaderBuilder::new()
                        .delimiter(delimiter)
                        .has_headers(false)
                        .from_reader(f);
                    let mut writer = WriterBuilder::new()
                        .delimiter(delimiter)
                        .from_writer(io::stdout());

                    for record in reader.records() {
                        writer.write_record(extract_fields(&record?, field_pos))?;
                    }
                }
                Extract::Bytes(byte_pos) => {
                    for line in f.lines() {
                        println!("{}", extract_bytes(&line?, byte_pos));
                    }
                }
                Extract::Chars(char_pos) => {
                    for line in f.lines() {
                        println!("{}", extract_chars(&line?, char_pos));
                    }
                }
            },
        }
    }

    Ok(())
}
