use crate::level::{LevelObject, SerializeLevelObject};
use bevy::log::Level;
use std::fmt::format;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn write_level(objects: Vec<LevelObject>) -> String {
    let mut content = String::new();
    for object in objects {
        let (statement, args) = object.serialize();
        let args = args
            .iter()
            .map(|a| {
                let url_encoded = url::form_urlencoded::byte_serialize(a.as_bytes());
                String::from_iter(url_encoded)
            })
            .collect::<Vec<_>>()
            .join(" ");
        content += format!("{statement} {args}\n").as_str();
    }

    content
}

pub fn write_level_to_data(name: String, objects: Vec<LevelObject>) {
    let directory = "./levels";
    let _ = fs::create_dir_all(directory);

    let file_path = format!("./levels/{}.tdsave", name);
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path.clone())
    {
        let canonical_file_path = fs::canonicalize(PathBuf::from(file_path.clone())).unwrap();
        println!("Writing to {:?}", canonical_file_path);
        let data_to_write = write_level(objects);
        if let Err(_) = file.write_all(data_to_write.as_bytes()) {
            eprintln!("Failed to write")
        }
    } else {
        eprintln!("Failed to open file")
    }
}
