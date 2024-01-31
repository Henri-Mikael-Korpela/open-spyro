use crate::{byte_range::ByteRange, directory_record::DirectoryRecord, Unserialize};

#[derive(Debug)]
pub struct PrimaryVolumeDescriptor {
    pub volume_identifier: String,
    /// This field shall specify as a 32-bit number the number of Logical Blocks
    /// in which the Volume Space of the volume is recorded.
    /// ([ECMA-119](https://www.ecma-international.org/wp-content/uploads/ECMA-119_4th_edition_june_2019.pdf), p. 35, 8.4.8)
    pub volume_space_size: i32,
    // The size in bytes of a logical block, because a logical block on a CD could be something other than 2 KB.
    pub logical_block_size: i16,
    /// LBA (Location of extent) location of the path table. The path table pointed to contains only little endian values.
    pub location_of_type_l_path_table: i32,
    pub directory_record_for_root_directory: DirectoryRecord,
    pub publisher_identifier: String,
    pub application_identifier: String,
}
impl PrimaryVolumeDescriptor {
    pub const VOLUME_IDENTIFIER_RANGE: ByteRange = ByteRange::new(40, 72);
    pub const VOLUME_SPACE_SIZE_RANGE: ByteRange = ByteRange::new(80, 88);
    pub const LOGICAL_BLOCK_SIZE_RANGE: ByteRange = ByteRange::new(128, 132);
    pub const LOCATION_OF_TYPE_L_PATH_TABLE_RANGE: ByteRange = ByteRange::new(140, 144);
    pub const DIRECTORY_RECORD_FOR_ROOT_DIRECTORY_RANGE: ByteRange = ByteRange::new(156, 190);
    pub const PUBLISHER_IDENTIFIER_RANGE: ByteRange = ByteRange::new(318, 446);
    pub const APPLICATION_IDENTIFIER_RANGE: ByteRange = ByteRange::new(574, 702);

    pub fn try_from_buffer(buf: &[u8], offset_in_file: u64) -> Result<Self, String> {
        const CD001: &'static str = "CD001";
        const CD001_BYTES: &[u8] = CD001.as_bytes();
        const OFFSET_FROM_DESCRIPTOR_TYPE: usize = 1;

        let descriptor_buf_cd001 =
            &buf[OFFSET_FROM_DESCRIPTOR_TYPE..OFFSET_FROM_DESCRIPTOR_TYPE + CD001_BYTES.len()];

        if *CD001_BYTES != *descriptor_buf_cd001 {
            return Err(format!(
                "Descriptor is missing its \"{}\" in file at offset {}",
                CD001, offset_in_file
            ));
        }

        let volume_identifier = Self::VOLUME_IDENTIFIER_RANGE.read_as_string(buf);

        let volume_space_size = Self::VOLUME_SPACE_SIZE_RANGE.read_as_both_endian::<i32>(buf);

        let logical_block_size = Self::LOGICAL_BLOCK_SIZE_RANGE.read_as_both_endian::<i16>(buf);

        let location_of_type_l_path_table =
            Self::LOCATION_OF_TYPE_L_PATH_TABLE_RANGE.read_as_little_endian::<i32>(buf);

        let directory_record_for_root_directory =
            Self::DIRECTORY_RECORD_FOR_ROOT_DIRECTORY_RANGE.read_as_bytes::<34>(buf);
        let directory_record_for_root_directory =
            match DirectoryRecord::unserialize(&directory_record_for_root_directory) {
                Ok(directory) => directory,
                Err(message) => {
                    return Err(message);
                }
            };

        let publisher_identifier = Self::PUBLISHER_IDENTIFIER_RANGE.read_as_string(buf);

        let application_identifier = Self::APPLICATION_IDENTIFIER_RANGE.read_as_string(buf);

        Ok(Self {
            volume_identifier,
            volume_space_size,
            logical_block_size,
            location_of_type_l_path_table,
            directory_record_for_root_directory,
            publisher_identifier,
            application_identifier,
        })
    }
}
