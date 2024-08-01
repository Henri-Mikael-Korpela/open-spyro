use std::{fs::File, io::{Error, Read, Seek, SeekFrom}};

pub fn read_bytes_from_file(file_path: &str, offset: usize, count: usize) -> Result<Vec<u8>, Error> {
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(offset as u64))?;
    let mut buffer = vec![0; count];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}