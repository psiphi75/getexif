use std::time::SystemTime;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,
}

use chrono::{DateTime, SecondsFormat, Utc};

fn to_utc(sys_time: SystemTime) -> String {
    let dt = DateTime::<Utc>::from(sys_time);
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn jpeg_to_metadata(path: &std::path::Path) -> std::io::Result<String> {
    let data = std::fs::metadata(path)?;

    println!("filename={:?}", path.file_stem());
    println!("size={:?}", data.len());
    println!("created_time={:?}", to_utc(data.created()?));
    println!("modified_time={:?}", to_utc(data.modified()?));

    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    let f = exif
        .get_field(exif::Tag::Orientation, exif::In::PRIMARY)
        .unwrap();
    println!("orientation={:?}", f.value);

    let f = exif
        .get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
        .unwrap();
    println!("capture_time={:?}", f.value);

    let f = exif.get_field(exif::Tag::Model, exif::In::PRIMARY).unwrap();
    println!("camera_model={:?}", f.value);

    let f = exif
        .get_field(exif::Tag::BodySerialNumber, exif::In::PRIMARY)
        .unwrap();
    println!("camera_serial={:?}", f.value);
    Ok("".to_string())
}

fn main() {
    let args = Cli::parse();

    if args.files.len() == 0 {
        eprintln!("Error: No JPG files provided");
        std::process::exit(0x01);
    }

    for file in &args.files {
        println!("{}", file);
        jpeg_to_metadata(std::path::Path::new(file)).unwrap();
    }
}
