use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

mod errors;
use errors::DfsError;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Map {
    file_name: String,
    chunk_size: usize, 
    files_names: Vec<String>,
}

impl Map {
    fn new(file_name: String, chunk_size: usize, files_names: Vec<String>) -> Map {
        Map {
            file_name,
            files_names,
            chunk_size,
        }
    }

    fn create(&self) -> Result<(), DfsError> {
        let json = serde_json::to_string(self)?;
        let path = Path::new(&self.file_name)
            .file_name()
            .ok_or_else(|| DfsError::InvalidFileName(self.file_name.clone()))?;
        write(
            format!("./maps/{}.map.json", path.display()),
            &json.into_bytes(),
        )?;
        Ok(())
    }
}

pub fn chunk_file(
    file_name: &str,
    chunk_size: usize,
) -> Result<Vec<String>, DfsError> {
    std::fs::create_dir_all("./chunks")?;
    std::fs::create_dir_all("./maps")?;
    let data = std::fs::read(file_name)?;
    let chunks = data.chunks(chunk_size);
    let mut files_names = Vec::new();

    for chunk in chunks {
        let hex_id = hex::encode(Sha256::digest(chunk));
        write(format!("./chunks/{}", hex_id), chunk)?;
        files_names.push(hex_id);
    }

    let map = Map::new(file_name.to_string(), chunk_size, files_names.clone());
    map.create()?;

    Ok(files_names)
}

pub fn reassmble(map_name: &str) -> Result<(), DfsError> {
    std::fs::create_dir_all("./restored")?;
    let content = read(format!("./maps/{}.map.json", map_name))?;
    let map: Map = serde_json::from_str(&String::from_utf8_lossy(&content))?;
    let mut reassembled_file = Vec::new();

    for f in map.files_names {
        let content = read(format!("./chunks/{}", f))?;
        let hex_id = hex::encode(Sha256::digest(&content));
        assert_eq!(hex_id, f);
        reassembled_file.extend_from_slice(&content);
    }
    write(
        format!("./restored/{}", map.file_name),
        reassembled_file.as_slice(),
    )?;

    Ok(())
}

fn write(to: String, content: &[u8]) -> Result<(), DfsError> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(to)?;
    file.write_all(content)?;

    Ok(())
}

fn read(from: String) -> Result<Vec<u8>, DfsError> {
    let mut file = File::open(from)?;
    let mut vec_buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut vec_buf)?;
    Ok(vec_buf)
}
