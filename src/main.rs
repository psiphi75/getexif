use clap::Parser;
use rayon::prelude::*;
use serde_json::{Map, Value};
use std::{error::Error, fs::File};

mod exif_reader;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,
}

fn save_metadata(
    path: &std::path::Path,
    json_data: &Map<String, Value>,
) -> Result<(), Box<dyn Error>> {
    // Create the .JSON path
    let mut json_path = std::path::PathBuf::new();
    json_path.push(path);
    json_path.set_extension("json");

    // Save the JSON to the file
    serde_json::to_writer_pretty(&File::create(json_path)?, json_data).unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    if args.files.len() == 0 {
        eprintln!("Error: No JPG files provided");
        std::process::exit(0x01);
    }

    // TODO: Should use the number of CPUs available, or find the a good
    //       balance.  IO is likely to be the bottleneck, not CPU, so want to
    //       test this a bit.
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()?;

    args.files.par_iter().for_each(|file| {
        println!("{}", file);
        let path = Path::new(file);

        // FIXME: Need to handle these results, and not use `unwrap()`.
        let metadata = exif_reader::jpeg_to_metadata(path).unwrap();
        save_metadata(path, &metadata).unwrap();
    });

    Ok(())
}
