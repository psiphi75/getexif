use std::{error::Error, time::SystemTime};

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

fn get_ascii(exif: &exif::Exif, tag: exif::Tag) -> Option<String> {
    let f = exif.get_field(tag, exif::In::PRIMARY);
    if f.is_none() {
        return None;
    }
    let ascii = &f.unwrap().value;

    if let exif::Value::Ascii(value) = ascii {
        // FIXME: We've hardcoded the "value[0]", works for this little program
        //        but should review this for production.
        let buf = value[0].to_owned();

        match String::from_utf8(buf) {
            Ok(v) => return Some(v),
            Err(_) => return None,
        };
    }

    return None;
}

fn get_short(exif: &exif::Exif, tag: exif::Tag) -> Option<u16> {
    let f = exif.get_field(tag, exif::In::PRIMARY);
    if f.is_none() {
        return None;
    }
    let short = &f.unwrap().value;

    if let exif::Value::Short(value) = short {
        // FIXME: We've hardcoded the "value[0]", works for this little program
        //        but should review this for production.
        let v = value[0];

        return Some(v);
    }

    return None;
}

fn jpeg_to_metadata(path: &std::path::Path) -> Result<String, Box<dyn Error>> {
    let data = std::fs::metadata(path)?;

    println!("filename={:?}", path.file_stem());
    println!("size={:?}", data.len());
    println!("created_time={:?}", to_utc(data.created()?));
    println!("modified_time={:?}", to_utc(data.modified()?));

    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;

    if let Some(v) = get_short(&exif, exif::Tag::Orientation) {
        println!("orientation={}", v);
    }

    if let Some(v) = get_ascii(&exif, exif::Tag::DateTimeOriginal) {
        println!("capture_time={}", v);
    }

    if let Some(v) = get_ascii(&exif, exif::Tag::Model) {
        println!("camera_model={}", v);
    }

    if let Some(v) = get_ascii(&exif, exif::Tag::BodySerialNumber) {
        println!("camera_serial={}", v);
    }

    Ok("".to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    if args.files.len() == 0 {
        eprintln!("Error: No JPG files provided");
        std::process::exit(0x01);
    }

    for file in &args.files {
        println!("{}", file);
        jpeg_to_metadata(std::path::Path::new(file))?;
    }

    Ok(())
}
