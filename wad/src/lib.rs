use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

/// WAD file format is a custom file format used for Spyro the Dragon.
/// It is a container format that contains multiple files.
///
/// # File layout
/// ## Header
///
/// # Notes
/// Because Insomniac Games worked on Disruptor, which is a Doom clone, before Spyro the Dragon,
/// it is likely that the WAD file format is based on the WAD file format used in Doom.
/// At least some of the developers were probably familiar with the WAD file format.
///
/// WAD probably stands for "Where's All the Data" as in Doom.
///
/// Huge thanks to KlyMenCompany for providing a reference implementation in SpyroWorldViewer.
/// It was tremendously helpful in putting this WAD file reading logic together.
pub struct WAD<'a> {
    file: File,
    file_path: &'a str,
    pub file_size: u64,
}
impl<'a> WAD<'a> {
    pub fn from_file_path(file_path: &'a str) -> Result<Self, String> {
        let file = File::open(file_path).map_err(|err| {
            format!(
                "Failed to open WAD file \"{}\": {}",
                file_path,
                err.to_string()
            )
        })?;
        let file_size = get_file_size(&file, file_path)?;
        Ok(Self {
            file,
            file_path,
            file_size,
        })
    }
}

#[derive(Debug)]
pub struct WADFileMetadata {
    pub offset: u32,
    pub size: u32,
}

pub struct WADReader<'a> {
    wad: &'a WAD<'a>,
}
impl<'a> WADReader<'a> {
    const WAD_FILE_METADATA_LEN: usize = 8;

    pub fn new(wad: &'a WAD) -> Self {
        Self { wad }
    }
    pub fn read_subfiles_by_file_metadata(
        &self,
        file_metadata: &WADFileMetadata,
    ) -> Result<Vec<WADFileMetadata>, String> {
        let mut reader = BufReader::new(&self.wad.file);

        // Seek to the beginning of the file by its offset.
        reader
            .seek(SeekFrom::Start(file_metadata.offset as u64))
            .map_err(|err| {
                format!(
                    "Failed to seek to the beginning of the file in WAD file in path \"{}\": {}",
                    self.wad.file_path,
                    err.to_string()
                )
            })?;

        let mut file_metadatum = Vec::<WADFileMetadata>::new();
        let mut wad_file_subfile_metadata_buffer = [0u8; Self::WAD_FILE_METADATA_LEN];

        loop {
            // Try to read the next 8 bytes from the WAD file.
            reader
                .read(&mut wad_file_subfile_metadata_buffer)
                .map_err(|err| {
                    format!(
                        "Failed to read bytes from WAD file header in file in path \"{}\": {}",
                        self.wad.file_path,
                        err.to_string()
                    )
                })?;

            // If the buffer is empty (contains only bytes whose value is 0), stop reading.
            if wad_file_subfile_metadata_buffer.iter().all(|b| *b == 0) {
                break;
            }

            let file = get_file_metadata_from_bytes(&wad_file_subfile_metadata_buffer);
            file_metadatum.push(file);

            // Get the current position in the WAD file. If it is at the end of the file, stop reading.
            let position = reader.seek(SeekFrom::Current(0)).map_err(|err| {
                format!(
                    "Failed to get current position in WAD file in path \"{}\": {}",
                    self.wad.file_path,
                    err.to_string()
                )
            })?;

            // Add WAD file metadata length to this check, because in the next iteration
            // of this loop, the next WAD file metadata will be read and it is necessary
            // to check if there enough bytes to read.
            if (position + Self::WAD_FILE_METADATA_LEN as u64) >= self.wad.file_size {
                break;
            }
        }

        Ok(file_metadatum)
    }
    pub fn read_file_metadatum_from_header(&self) -> Result<Vec<WADFileMetadata>, String> {
        let mut reader = BufReader::new(&self.wad.file);

        let mut file_metadatum = Vec::<WADFileMetadata>::new();
        let mut wad_file_file_metadata_buffer = [0u8; Self::WAD_FILE_METADATA_LEN];

        // Read information about files from the header section.
        loop {
            // Try to read the next 8 bytes from the WAD file.
            reader
                .read(&mut wad_file_file_metadata_buffer)
                .map_err(|err| {
                    format!(
                        "Failed to read bytes from WAD file header in file in path \"{}\": {}",
                        self.wad.file_path,
                        err.to_string()
                    )
                })?;

            // If the buffer is empty (contains only bytes whose value is 0), stop reading.
            if wad_file_file_metadata_buffer.iter().all(|b| *b == 0) {
                break;
            }

            let file = get_file_metadata_from_bytes(&wad_file_file_metadata_buffer);
            file_metadatum.push(file);

            // Get the current position in the WAD file. If it is at the end of the file, stop reading.
            let position = reader.seek(SeekFrom::Current(0)).map_err(|err| {
                format!(
                    "Failed to get current position in WAD file in path \"{}\": {}",
                    self.wad.file_path,
                    err.to_string()
                )
            })?;

            // Add WAD file metadata length to this check, because in the next iteration
            // of this loop, the next WAD file metadata will be read and it is necessary
            // to check if there enough bytes to read.
            if (position + Self::WAD_FILE_METADATA_LEN as u64) >= self.wad.file_size {
                break;
            }
        }

        Ok(file_metadatum)
    }
}

fn get_file_metadata_from_bytes(value: &[u8; WADReader::WAD_FILE_METADATA_LEN]) -> WADFileMetadata {
    // Because value is always 8 bytes long, it is safe to use try_into() and then unwrap.
    // A panic should never occur.

    // Read file offset (4 bytes).
    let offset = u32::from_le_bytes(value[0..4].try_into().unwrap());
    // Read file size (4 bytes).
    let size = u32::from_le_bytes(
        value[4..WADReader::WAD_FILE_METADATA_LEN]
            .try_into()
            .unwrap(),
    );
    WADFileMetadata { offset, size }
}
fn get_file_size(file: &File, file_path: &str) -> Result<u64, String> {
    let metadata = file.metadata().map_err(|err| {
        format!(
            "Failed to get metadata (in order to determine file size in bytes) of given WAD file in path \"{}\": {}",
            file_path, err.to_string()
        )
    })?;
    Ok(metadata.len())
}
