use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::{fs::File, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub description: String,
    pub folder: String,
    pub tags: Vec<String>,
}

pub fn read_file(path: &Path) -> Result<HashMap<String, Note>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)
        .or_else(|_| create_if_not_exist(path))
        .expect("Cannot open file");

    let metadata = file.metadata()?;
    println!("Metadata length: {:?}", metadata.len());
    if metadata.len() == 1 || metadata.len() == 0 {
        let data: HashMap<String, Note> = HashMap::new();
        return Ok(data);
    }

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: HashMap<String, Note> = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn update_file(data: &mut HashMap<String, Note>, update: Note) {
    data.insert(update.id.clone(), update);
}

pub fn write_file(
    path: &Path,
    data: &HashMap<String, Note>,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = serde_json::to_string_pretty(data)?;
    let mut file = File::create(path)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}

fn create_if_not_exist(path: &Path) -> io::Result<File> {
    File::create(path)
}
