use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use serde_json::{Map, Number, Value};
use std::{error::Error, path::Path, time::SystemTime};

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

fn get_date_from_ascii(exif: &exif::Exif, tag: exif::Tag) -> Option<String> {
    let f = get_ascii(exif, tag);
    if f.is_none() {
        return None;
    }

    let date_str = &f.unwrap();
    let dt = NaiveDateTime::parse_from_str(date_str, "%Y:%m:%d %H:%M:%S");
    if dt.is_err() {
        eprintln!("Error parsing date: {:?}, date input={:?}", dt, date_str);
        return None;
    }

    // FIXME: This does not give the date we expect
    let result = dt.unwrap().to_string();

    Some(result)
}

fn read_file_stat(path: &Path, map: &mut Map<String, Value>) -> Result<(), Box<dyn Error>> {
    let data = std::fs::metadata(path)?;

    let filename = match path.file_name() {
        Some(f) => f.to_string_lossy().to_string(),
        None => return Err(String::from("Invalid file provided").into()),
    };
    map.insert("filename".to_string(), Value::String(filename));
    map.insert(
        "size".to_string(),
        Value::Number(serde_json::Number::from(data.len())),
    );
    map.insert(
        "created_time".to_string(),
        Value::String(to_utc(data.created()?)),
    );
    map.insert(
        "modified_time".to_string(),
        Value::String(to_utc(data.modified()?)),
    );

    Ok(())
}

fn read_exif_data(path: &Path, map: &mut Map<String, Value>) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;

    if let Some(v) = get_short(&exif, exif::Tag::Orientation) {
        map.insert("orientation".to_string(), Value::Number(Number::from(v)));
    }

    if let Some(v) = get_date_from_ascii(&exif, exif::Tag::DateTimeOriginal) {
        map.insert("capture_time".to_string(), Value::String(v));
    }

    if let Some(v) = get_ascii(&exif, exif::Tag::Model) {
        map.insert("camera_model".to_string(), Value::String(v));
    }

    if let Some(v) = get_ascii(&exif, exif::Tag::BodySerialNumber) {
        map.insert("camera_serial".to_string(), Value::String(v));
    }

    Ok(())
}

pub fn jpeg_to_metadata(path: &Path) -> Result<Map<String, Value>, Box<dyn Error>> {
    let mut map = Map::new();

    // Read the filesystem details
    read_file_stat(path, &mut map)?;

    // Read the exif data from the image
    read_exif_data(path, &mut map)?;

    return Ok(map);
}
