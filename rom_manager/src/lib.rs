use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    ops::BitAnd,
};

use strum::IntoEnumIterator;
use strum::{EnumIter, FromRepr};

fn write_bytes_into(data: &mut [u8], begin: usize, value: &[u8]) {
    for (i, b) in data[begin..begin + value.len()].iter_mut().enumerate() {
        *b = value[i];
    }
}

pub trait FromBothEndian {
    fn from_both_endian(value: &[u8]) -> Self;
}
impl FromBothEndian for i16 {
    fn from_both_endian(value: &[u8]) -> Self {
        // The first half of bytes contains value in little endian format,
        // the second half of bytes contains value in big endian format.
        // One only needs to read either or.
        // This implementation handles bytes in the little endian format.
        let slice: [u8; 2] = value[0..2].try_into().unwrap();
        Self::from_le_bytes(slice)

        // In case someone wants to read bytes in the big endian format...
        /*let slice: [u8; 2] = buf[2..4].try_into().unwrap();
        Self::from_be_bytes(slice)*/
    }
}
impl FromBothEndian for i32 {
    fn from_both_endian(value: &[u8]) -> Self {
        // The first half of bytes contains value in little endian format,
        // the second half of bytes contains value in big endian format.
        // One only needs to read either or.
        // This implementation handles bytes in the little endian format.
        let slice: [u8; 4] = value[0..4].try_into().unwrap();
        Self::from_le_bytes(slice)

        // In case someone wants to read bytes in the big endian format...
        /*let slice: [u8; 4] = buf[4..8].try_into().unwrap();
        Self::from_be_bytes(slice)*/
    }
}

pub trait FromLittleEndian {
    fn from_little_endian(value: &[u8]) -> Self;
}
impl FromLittleEndian for i32 {
    fn from_little_endian(value: &[u8]) -> Self {
        let slice = value[0..4].try_into().unwrap();
        Self::from_le_bytes(slice)
    }
}

pub trait ByteCount {
    const BYTES: usize;
}
impl ByteCount for i16 {
    const BYTES: usize = Self::BITS as usize / 8;
}
impl ByteCount for i32 {
    const BYTES: usize = Self::BITS as usize / 8;
}

#[derive(Debug)]
pub struct ByteRange {
    pub begin: usize,
    pub end: usize,
}
impl ByteRange {
    pub const fn get_length_in_bytes(&self) -> usize {
        self.end - self.begin
    }
    pub const fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }
    pub fn read_as_bytes<const SIZE: usize>(&self, buf: &[u8]) -> [u8; SIZE] {
        buf[self.begin..self.end].try_into().unwrap()
    }
    /// Reads bytes including both endians, the little and the big endian (in that order).
    #[inline]
    pub fn read_as_both_endian<T>(&self, buf: &[u8]) -> T
    where
        T: FromBothEndian + ByteCount,
    {
        T::from_both_endian(&buf[self.begin..self.begin + T::BYTES])
    }
    #[inline]
    pub fn read_as_little_endian<T>(&self, buf: &[u8]) -> T
    where
        T: FromLittleEndian + ByteCount,
    {
        T::from_little_endian(&buf[self.begin..self.begin + T::BYTES])
    }
    #[inline]
    /**
     * Converts bytes into a string. Special empty characters with value 32 are also removed.
     */
    pub fn read_as_string(&self, buf: &[u8]) -> String {
        const SPECIAL_EMPTY_CHAR: u8 = 32;
        let slice = buf[self.begin..self.end]
            .iter()
            .filter(|b| **b != SPECIAL_EMPTY_CHAR)
            .map(|b| *b)
            .collect::<Vec<_>>();
        std::str::from_utf8(&slice[..]).unwrap().to_string()
    }
}

pub struct BothEndianI16Field<'a>(&'a ByteRange);
impl<'a> BothEndianI16Field<'a> {
    pub fn with_range(range: &'a ByteRange) -> Self {
        Self(&range)
    }
    pub fn write_into(&self, data: &mut [u8], begin_in_data: usize, value_to_write: i16) {
        for (i, b) in value_to_write.to_le_bytes().iter().enumerate() {
            data[begin_in_data + self.0.begin + i] = *b;
        }
        for (i, b) in value_to_write.to_be_bytes().iter().enumerate() {
            data[begin_in_data + self.0.begin + i16::BYTES + i] = *b;
        }
    }
}

#[test]
fn write_into_array_as_both_endian_i16() {
    let mut data = [0_u8; 4];

    let field = BothEndianI16Field::with_range(&ByteRange { begin: 0, end: 1 });
    field.write_into(&mut data, 0, 16);

    assert_eq!(data, [16, 0, 0, 16]);
}

pub struct BothEndianI32Field<'a>(pub &'a ByteRange);
impl<'a> BothEndianI32Field<'a> {
    pub fn with_range(range: &'a ByteRange) -> Self {
        Self(&range)
    }
    pub fn write_into(&self, data: &mut [u8], begin_in_data: usize, value_to_write: i32) {
        for (i, b) in value_to_write.to_le_bytes().iter().enumerate() {
            data[begin_in_data + self.0.begin + i] = *b;
        }
        for (i, b) in value_to_write.to_be_bytes().iter().enumerate() {
            data[begin_in_data + self.0.begin + i32::BYTES + i] = *b;
        }
    }
}

pub struct DateAndTimeField<'a>(pub &'a ByteRange);
impl<'a> DateAndTimeField<'a> {
    pub fn format_by_bytes(value: &[u8; 7]) -> String {
        // The first byte has a number of years since 1900
        let year = 1900 + value[0] as usize;
        let month = value[1] as usize;
        let day = value[2] as usize;
        let hours = value[3] as usize;
        let minutes = value[4] as usize;
        let seconds = value[5] as usize;
        format!(
            "{}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}",
            year, month, day, hours, minutes, seconds
        )
    }
    pub fn with_range(range: &'a ByteRange) -> Self {
        Self(&range)
    }
    pub fn write_into(&self, data: &mut [u8], begin_in_data: usize, value: &[u8; 7]) {
        let begin = self.0.begin + begin_in_data;
        for (i, b) in data[begin..begin + value.len()].iter_mut().enumerate() {
            *b = value[i];
        }
    }
}

#[test]
fn write_into_array_as_both_endian_i32() {
    let mut data = [0_u8; 8];

    let field = BothEndianI32Field::with_range(&ByteRange { begin: 0, end: 1 });
    field.write_into(&mut data, 0, 16);

    assert_eq!(data, [16, 0, 0, 0, 0, 0, 0, 16]);
}

pub struct LittleEndianField<'a>(pub &'a ByteRange);
impl<'a> LittleEndianField<'a> {
    pub fn with_range(range: &'a ByteRange) -> Self {
        Self(&range)
    }
    pub fn write_into(&self, data: &mut [u8], begin_in_data: usize, value_to_write: i32) {
        for (i, b) in value_to_write.to_le_bytes().iter().enumerate() {
            data[begin_in_data + self.0.begin + i] = *b;
        }
    }
}

pub struct StringField<'a>(pub &'a ByteRange);
impl<'a> StringField<'a> {
    pub fn with_range(range: &'a ByteRange) -> Self {
        Self(&range)
    }
    pub fn write_into(&self, data: &mut [u8], begin_in_data: usize, value_to_write: &String) {
        const IDENTIFIER_ZEROFILL_CHARACTER: u8 = 32;

        let len = value_to_write.len();

        // Copy bytes from a value to a target
        for (i, b) in value_to_write.bytes().enumerate() {
            data[begin_in_data + self.0.begin + i] = b;
        }

        // Zerofill all the rest of the range with a zerofill character
        for i in self.0.begin + len..self.0.end - 1 {
            data[begin_in_data + i] = IDENTIFIER_ZEROFILL_CHARACTER;
        }
    }
}

#[derive(Debug)]
pub struct DirectoryRecord {
    /// Length of Directory Record (LEN_DR) (BP 1)
    ///
    /// This field shall specify as an 8-bit number the length in bytes of the Directory Record.
    /// (ECMA 119, 9.1.1)
    pub length: u8,
    /// Extended Attribute Record Length (BP 2)
    ///
    /// This field shall contain an 8-bit number. This number shall specify the assigned
    /// Extended Attribute Record length if an Extended Attribute Record is recorded.
    /// Otherwise this number shall be zero. (ECMA 119, 9.1.2)
    pub extended_attribute_record: u8,
    /// Location of Extent (BP 3 to 10)
    ///
    /// This field shall specify as a 32-bit number the Logical Block Number of
    /// the first Logical Block allocated to the Extent (ECMA-119, 9.1.3).
    pub location_of_extent: i32,
    /// Means the same as size of extent.
    pub data_length: i32,
    pub recording_date_and_time:
        [u8; DirectoryRecord::RECORDING_DATE_AND_TIME_RANGE.get_length_in_bytes()],
    pub file_flags: u8,
    /// File Unit Size (BP 27)
    ///
    /// This field shall contain an 8-bit number. This number shall specify the assigned
    /// File Unit size for the File Section if the File Section is recorded in interleaved
    /// mode. Otherwise this number shall be zero.
    pub file_unit_size: u8,
    /// Interleave Gap Size (BP 28)
    ///
    /// This field shall contain an 8-bit number. This number shall specify the assigned
    /// Interleave Gap size for the File Section if the File Section is recorded in
    /// interleaved mode. Otherwise this number shall be zero.
    pub interleave_gap_size: u8,
    /// Volume Sequence Number (BP 29 to 32)
    ///
    /// This field shall specify as a 16-bit number the ordinal number of the volume
    /// in the Volume Set on which the Extent described by this Directory Record is recorded.
    pub volume_sequence_number: i16,
    pub file_identifier_length: u8,
    pub file_identifier: [u8; 255],
}
impl Serialize for DirectoryRecord {
    fn serialize(&self) -> Vec<u8>
    where
        Self: Sized,
    {
        let mut result = vec![0u8; self.length as usize];

        result[DirectoryRecord::LENGTH_POSITION] = self.length;
        result[DirectoryRecord::EXTENDED_ATTRIBUTE_RECORD_POSITION] =
            self.extended_attribute_record;

        BothEndianI32Field::with_range(&DirectoryRecord::LOCATION_OF_EXTENT_RANGE).write_into(
            &mut result,
            0,
            self.location_of_extent,
        );
        BothEndianI32Field::with_range(&DirectoryRecord::SIZE_OF_EXTENT_RANGE).write_into(
            &mut result,
            0,
            self.data_length,
        );
        DateAndTimeField::with_range(&DirectoryRecord::RECORDING_DATE_AND_TIME_RANGE).write_into(
            &mut result,
            0,
            &self.recording_date_and_time,
        );
        result[DirectoryRecord::FILE_FLAGS_POSITION] = self.file_flags;
        result[DirectoryRecord::FILE_UNIT_SIZE_POSITION] = self.file_unit_size;
        result[DirectoryRecord::INTERLEAVE_GAP_SIZE_POSITION] = self.interleave_gap_size;
        BothEndianI16Field::with_range(&DirectoryRecord::VOLUME_SEQUENCE_NUMBER_RANGE).write_into(
            &mut result,
            0,
            self.volume_sequence_number,
        );
        result[DirectoryRecord::FILE_IDENTIFIER_LENGTH_POSITION] = self.file_identifier_length;

        result
    }
}
impl Unserialize for DirectoryRecord {
    fn unserialize(data: &[u8]) -> Result<Self, String> {
        let len = data.len();
        if len < 34 {
            return Err(format!(
                "There are not enough bytes given to read a directory record (value given was {})",
                len
            ));
        }

        let length = data[DirectoryRecord::LENGTH_POSITION];
        let extended_attribute_record = data[DirectoryRecord::EXTENDED_ATTRIBUTE_RECORD_POSITION];
        let location_of_extent = Self::LOCATION_OF_EXTENT_RANGE.read_as_both_endian::<i32>(data);
        let data_length = Self::SIZE_OF_EXTENT_RANGE.read_as_both_endian::<i32>(data);
        let recording_date_and_time = Self::RECORDING_DATE_AND_TIME_RANGE.read_as_bytes(data);
        let file_flags = data[DirectoryRecord::FILE_FLAGS_POSITION];
        let file_unit_size = data[DirectoryRecord::FILE_UNIT_SIZE_POSITION];
        let interleave_gap_size = data[DirectoryRecord::INTERLEAVE_GAP_SIZE_POSITION];
        let file_identifier_length = data[Self::FILE_IDENTIFIER_LENGTH_POSITION];
        let volume_sequence_number = Self::VOLUME_SEQUENCE_NUMBER_RANGE.read_as_both_endian(data);

        let mut file_identifier = [0u8; 255];

        for i in 0..file_identifier_length as usize {
            file_identifier[i] = data[33 + i];
        }

        Ok(Self {
            length,
            extended_attribute_record,
            location_of_extent,
            data_length,
            recording_date_and_time,
            file_flags,
            file_unit_size,
            interleave_gap_size,
            volume_sequence_number,
            file_identifier_length,
            file_identifier,
        })
    }
}
impl DirectoryRecord {
    const LENGTH_POSITION: usize = 0;
    const EXTENDED_ATTRIBUTE_RECORD_POSITION: usize = 1;
    const LOCATION_OF_EXTENT_RANGE: ByteRange = ByteRange::new(2, 10);
    const SIZE_OF_EXTENT_RANGE: ByteRange = ByteRange::new(10, 18);
    const RECORDING_DATE_AND_TIME_RANGE: ByteRange = ByteRange::new(18, 25);
    const FILE_FLAGS_POSITION: usize = 25;
    const FILE_UNIT_SIZE_POSITION: usize = 26;
    const INTERLEAVE_GAP_SIZE_POSITION: usize = 27;
    const VOLUME_SEQUENCE_NUMBER_RANGE: ByteRange = ByteRange::new(28, 32);
    const FILE_IDENTIFIER_LENGTH_POSITION: usize = 32;

    /// TODO: Optimize with get_file_version
    pub fn file_identifier_as_string(&self) -> String {
        if self.file_identifier[0] == 0 {
            String::new()
        } else {
            let value =
                std::str::from_utf8(&self.file_identifier[0..self.file_identifier_length as usize])
                    .unwrap()
                    .to_string();
            value.split(';').nth(0).unwrap().to_string()
        }
    }
    /// Extracts file version from a file identifier.
    /// If there is no version, 0 will be returned.
    ///
    /// TODO: Optimize with get_file_identifier_as_string
    pub fn file_version(&self) -> u8 {
        if self.file_identifier[0] == 0 {
            0
        } else {
            let value =
                std::str::from_utf8(&self.file_identifier[0..self.file_identifier_length as usize])
                    .unwrap()
                    .to_string();
            if let Some(version) = value.split(';').nth(1) {
                version.to_string().parse::<u8>().unwrap()
            } else {
                0
            }
        }
    }
    pub fn is_dir(&self) -> bool {
        // "If set to ONE, shall mean that the Directory Record identifies a directory."
        // ECMA-119, Table 10, BP 1
        (self.file_flags & DirectoryRecordFileFlag::Directory != 0)
            // "If this Directory Record identifies a directory then bit positions 2, 3 and 7 shall be set to ZERO."
            // ECMA-119, 9.1.6
            && (self.file_flags & DirectoryRecordFileFlag::AssociatedFile == 0)
            && (self.file_flags & DirectoryRecordFileFlag::Record == 0)
            && (self.file_flags & DirectoryRecordFileFlag::MultiExtent == 0)
    }
    pub fn recording_date_and_time_formatted(&self) -> String {
        DateAndTimeField::format_by_bytes(&self.recording_date_and_time)
    }
}

enum DirectoryRecordFileFlag {
    /// If set to ZERO, shall mean that the existence of the file shall be made known to
    /// the user upon an inquiry by the user.
    ///
    /// If set to ONE, shall mean that the existence of the file need not be made known to
    /// the user.
    ///
    /// ([ECMA-119](https://www.ecma-international.org/wp-content/uploads/ECMA-119_4th_edition_june_2019.pdf), 9.1.6)
    Existance = 1 << 0,
    /// If set to ZERO, shall mean that the Directory Record does not identify a directory.
    ///
    /// If set to ONE, shall mean that the Directory Record identifies a directory.
    ///
    /// ([ECMA-119](https://www.ecma-international.org/wp-content/uploads/ECMA-119_4th_edition_june_2019.pdf), 9.1.6)
    Directory = 1 << 1,
    /// If set to ZERO, shall mean that the file is not an Associated File.
    ///
    /// If set to ONE, shall mean that the file is an Associated File.
    ///
    /// ([ECMA-119](https://www.ecma-international.org/wp-content/uploads/ECMA-119_4th_edition_june_2019.pdf), 9.1.6)
    AssociatedFile = 1 << 2,
    /// ([ECMA-119](https://www.ecma-international.org/wp-content/uploads/ECMA-119_4th_edition_june_2019.pdf), 9.1.6)
    Record = 1 << 3,
    /// ([ECMA-119](https://www.ecma-international.org/wp-content/uploads/ECMA-119_4th_edition_june_2019.pdf), 9.1.6)
    Protection = 1 << 4,
    /// ([ECMA-119](https://www.ecma-international.org/wp-content/uploads/ECMA-119_4th_edition_june_2019.pdf), 9.1.6)
    MultiExtent = 1 << 7,
}
// There is a Rust crate for dealing with bitflags, but I wanted to implement
// only the trait that I need explicitly
impl BitAnd<DirectoryRecordFileFlag> for u8 {
    type Output = Self;

    fn bitand(self, rhs: DirectoryRecordFileFlag) -> Self::Output {
        self & rhs as Self
    }
}

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

/// Represents a volume on a CD, also called ROM.
/// Playstation follows the CD-ROM XA standard, which is a variation
/// of the ISO 9660 standard used for CD-ROMs.
pub struct CDROMXAVolume {
    file: File,
}
impl CDROMXAVolume {
    const SYSTEM_AREA_SECTOR_COUNT: u64 = 16;

    pub fn new(file: File) -> Self {
        Self { file }
    }
    /// Reads directory record data into a byte buffer.
    pub fn read_directory_record_data(
        &mut self,
        directory_record: &DirectoryRecord,
        logical_block_size: i16,
    ) -> Result<Vec<u8>, String> {
        // If the directory record is not a file, then it does not have data and cannot be read
        if directory_record.is_dir() {
            return Err(String::from(
                "Failed to read directory record data into a byte buffer: directory record is not a file.",
            ));
        }

        let sector_size = Sector::get_size_by_logical_block_size(logical_block_size);

        // Set seek to the beginning of the data for the given directory record
        let sector_base_offset = directory_record.location_of_extent as u64 * sector_size as u64;

        let mut reader = BufReader::new(&self.file);

        if let Err(_) = reader.seek(SeekFrom::Start(sector_base_offset)) {
            return Err(format!("Failed to set seek for directory record data begin in file by offset {}, because it does not exist.", sector_base_offset));
        }

        // Allocate all memory as bytes necessary for storing the entire directory record data
        let mut file_data_buf = Vec::<u8>::with_capacity(directory_record.data_length as usize);
        let mut file_data_bytes_left = directory_record.data_length as usize;
        let mut sector_data_buf = vec![0_u8; logical_block_size as usize];

        let sector_count =
            f32::ceil(directory_record.data_length as f32 / sector_size as f32) as usize;

        for i in 0..sector_count {
            // If reading any other sector except the last one
            if i < sector_count - 1 {
                reader.seek_relative(Sector::XA_HEADER_BYTE_COUNT).unwrap();

                reader.read_exact(&mut sector_data_buf).unwrap();

                for b in sector_data_buf.iter() {
                    file_data_buf.push(*b);
                }

                reader
                    .seek_relative(Sector::XA_DATA_LAST_BYTES_COUNT)
                    .unwrap();

                file_data_bytes_left -= logical_block_size as usize;
            }
            // If reading the last sector
            else {
                reader.seek_relative(Sector::XA_HEADER_BYTE_COUNT).unwrap();

                let mut buf = vec![0_u8; file_data_bytes_left as usize];
                reader.read_exact(&mut buf).unwrap();

                for b in buf.iter() {
                    file_data_buf.push(*b);
                }
            }
        }

        Ok(file_data_buf)
    }
    pub fn read_directory_records(
        &mut self,
        directory_record: &DirectoryRecord,
        logical_block_size: i16,
    ) -> Result<Vec<DirectoryRecord>, String> {
        let mut result = Vec::<DirectoryRecord>::new();

        if !directory_record.is_dir() {
            return Err(format!("Directory record in file is not a directory."));
        }

        let logical_block_size_usize = logical_block_size as usize;
        let sector_size = Sector::get_size_by_logical_block_size(logical_block_size);

        let sector_base_offset = directory_record.location_of_extent as u64 * sector_size as u64;

        let mut reader = BufReader::new(&self.file);

        if let Err(_) = reader.seek(SeekFrom::Start(sector_base_offset)) {
            return Err(format!("Failed to set seek for descriptor begin in file by offset {}, because it does not exist.", sector_base_offset));
        }

        let mut data_bytes_read = 0_usize;
        let mut sector_buf = vec![0_u8; sector_size];

        while data_bytes_read < directory_record.data_length as usize {
            if let Err(err) = reader.read_exact(&mut sector_buf) {
                return Err(format!("Failed to read sector: {}", err));
            }

            let mut sector_index = 0_usize;
            'read_records_in_sector: while sector_index < logical_block_size_usize {
                let record_begin_index = sector_index + Sector::XA_HEADER_BYTE_COUNT as usize;
                let record_length = sector_buf[record_begin_index] as usize;

                if record_length == 0 {
                    break 'read_records_in_sector;
                }

                match DirectoryRecord::unserialize(
                    &sector_buf[record_begin_index..record_begin_index + record_length],
                ) {
                    Ok(directory_record) => {
                        result.push(directory_record);
                    }
                    Err(err) => {
                        return Err(format!(
                            "Error occured while reading directory records: {}",
                            err
                        ));
                    }
                }

                sector_index += record_length;
            }

            data_bytes_read += logical_block_size_usize;
        }

        Ok(result)
    }

    pub fn read_primary_volume_descriptor(
        &mut self,
        desciptor_locations: &Vec<VolumeDescriptorLocation>,
    ) -> Result<PrimaryVolumeDescriptor, String> {
        // Find the primary volume descriptor location
        let Some(descriptor_location) = desciptor_locations
            .iter()
            .find(|l| l.descriptor_type == VolumeDescriptorType::Primary) else {
                return Err(String::from("No primary volume descriptor found."));
            };

        // Seek offset in file for the primary volume descriptor
        let mut reader = BufReader::new(&self.file);

        if let Err(_) = reader.seek(SeekFrom::Start(descriptor_location.descriptor_offset)) {
            return Err(format!("Failed to set seek for primary volume descriptor CD001 offset in file by offset {}, because it does not exist.", descriptor_location.descriptor_offset));
        }

        // Read the entire sector data into memory
        let mut descriptor_buf = [0_u8; Sector::LOGICAL_SIZE as usize];
        if let Err(_) = reader.read_exact(&mut descriptor_buf) {
            return Err(format!(
                "Failed to read a sector from file by offset {}, because it does not exist.",
                descriptor_location.descriptor_offset
            ));
        };

        Ok(PrimaryVolumeDescriptor::try_from_buffer(
            &descriptor_buf,
            descriptor_location.descriptor_offset,
        )?)
    }
    pub fn read_volume_descriptor_locations(
        &mut self,
    ) -> Result<Vec<VolumeDescriptorLocation>, String> {
        let mut result = Vec::<VolumeDescriptorLocation>::new();

        let mut reader = BufReader::new(&self.file);

        // Skip system area sectors by starting the iteration after system area sectors
        for sector_index in Self::SYSTEM_AREA_SECTOR_COUNT.. {
            let sector_offset = sector_index * Sector::LOGICAL_SIZE;
            let descriptor_begin_offset = sector_offset + 24;

            if let Err(_) = reader.seek(SeekFrom::Start(descriptor_begin_offset)) {
                return Err(format!("Failed to set seek for descriptor begin in file by offset {}, because it does not exist.", descriptor_begin_offset));
            }

            let mut buf = [0_u8; 1];
            if let Err(_) = reader.read_exact(&mut buf) {
                return Err(format!("Failed to read descriptor type from file by offset {}, because it does not exist.", descriptor_begin_offset));
            }

            let descriptor_type = match buf[0] {
                001 => VolumeDescriptorType::Primary,
                255 => VolumeDescriptorType::SetTerminator,
                value => {
                    let supported_values = VolumeDescriptorType::iter()
                        .map(|t| format!("{}", t as u8))
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(format!("Encountered unsupported descriptor type {}. Supported descriptor types include {}", value, supported_values));
                }
            };
            let desciptor_location = VolumeDescriptorLocation {
                descriptor_offset: descriptor_begin_offset,
                descriptor_type,
            };

            result.push(desciptor_location);

            if descriptor_type == VolumeDescriptorType::SetTerminator {
                break;
            }
        }

        Ok(result)
    }
    /// Writes to an already existing file on the volume.
    pub fn replace_file(
        &mut self,
        directory_record: &DirectoryRecord,
        logical_block_size: i16,
        content: &Vec<u8>,
    ) -> Result<(), String> {
        let mut writer = BufWriter::new(&self.file);

        let sector_size = Sector::get_size_by_logical_block_size(logical_block_size);
        let sector_base_offset = directory_record.location_of_extent as u64 * sector_size as u64;

        // Go to the beginning of the sector containing the file content with XA header bytes.
        // One could also seek the beginning of the sector + XA header bytes length (bytes to
        // skip), but this would make iteration through the sectors below more difficult to formulate.
        writer
            .seek(SeekFrom::Start(sector_base_offset))
            .map_err(|err| {
                format!(
                    "Failed to set seek for file on ROM by offset {}: {}",
                    sector_base_offset, err
                )
            })?;

        let sector_count =
            f32::ceil(directory_record.data_length as f32 / sector_size as f32) as usize;

        let data_len = directory_record.data_length as usize;

        let mut data_bytes_written = 0_usize;

        for i in 0..sector_count {
            // Skip the XA header bytes so that data is not written to them.
            let sector_data_offset = sector_base_offset
                + Sector::XA_HEADER_BYTE_COUNT as u64
                + i as u64 * sector_size as u64;
            writer
                .seek(SeekFrom::Start(sector_data_offset))
                .map_err(|err| {
                    format!(
                        "Failed to set seek for file on ROM by offset {}: {}",
                        sector_data_offset, err
                    )
                })?;

            // A sector size contains XA header and end bytes.
            // The sector size for writing should not includes length of those bytes,
            // so both of those lengths need to be subtracted.
            let sector_size_for_write = sector_size - Sector::XA_DATA_BYTE_COUNT as usize;

            let data_bytes_to_write = if data_bytes_written + sector_size_for_write > data_len {
                data_len - data_bytes_written
            } else {
                sector_size_for_write
            };

            let bytes_from_content =
                &content[data_bytes_written..data_bytes_written + data_bytes_to_write];

            writer.write_all(bytes_from_content).map_err(|err| {
                format!(
                    "Failed to write file on ROM by offset {}: {}",
                    sector_data_offset, err
                )
            })?;

            data_bytes_written += data_bytes_to_write;
        }

        Ok(())
    }
}

pub struct Sector {
    data: [u8; Self::LOGICAL_SIZE as usize],
}
impl Sector {
    const XA_HEADER_BYTE_COUNT: i64 = 24;
    /// Includes length of XA header bytes ([XA_HEADER_BYTE_COUNT]) and XA last bytes ([XA_DATA_LAST_BYTES_COUNT]) together.
    const XA_DATA_BYTE_COUNT: i64 = 304;
    const XA_DATA_LAST_BYTES_COUNT: i64 = Self::XA_DATA_BYTE_COUNT - Self::XA_HEADER_BYTE_COUNT;
    /// Logical sector size shall not be any larger than a logical block size.
    pub const LOGICAL_SIZE: u64 = 2352;

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    pub fn from_primary_volume_descriptor(
        descriptor: &PrimaryVolumeDescriptor,
        old_data: &[u8; Self::LOGICAL_SIZE as usize],
    ) -> Result<Sector, String> {
        let mut data = old_data.clone();

        const HEADER_LEN: usize = Sector::XA_HEADER_BYTE_COUNT as usize;

        // TODO: Figure out what header data contains exactly
        let header: [u8; HEADER_LEN] = [
            00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 00, 00, 02, 0x16, 02,
            00, 00, 09, 00, 00, 00, 09, 00,
        ];
        write_bytes_into(&mut data, 0, &header);

        StringField::with_range(&PrimaryVolumeDescriptor::VOLUME_IDENTIFIER_RANGE).write_into(
            &mut data,
            HEADER_LEN,
            &descriptor.volume_identifier,
        );
        BothEndianI32Field::with_range(&PrimaryVolumeDescriptor::VOLUME_SPACE_SIZE_RANGE)
            .write_into(&mut data, HEADER_LEN, descriptor.volume_space_size);
        BothEndianI16Field::with_range(&PrimaryVolumeDescriptor::LOGICAL_BLOCK_SIZE_RANGE)
            .write_into(&mut data, HEADER_LEN, descriptor.logical_block_size);
        LittleEndianField(&PrimaryVolumeDescriptor::LOCATION_OF_TYPE_L_PATH_TABLE_RANGE)
            .write_into(
                &mut data,
                HEADER_LEN,
                descriptor.location_of_type_l_path_table,
            );
        write_bytes_into(
            &mut data,
            HEADER_LEN + PrimaryVolumeDescriptor::DIRECTORY_RECORD_FOR_ROOT_DIRECTORY_RANGE.begin,
            &descriptor.directory_record_for_root_directory.serialize(),
        );
        StringField::with_range(&PrimaryVolumeDescriptor::PUBLISHER_IDENTIFIER_RANGE).write_into(
            &mut data,
            HEADER_LEN,
            &descriptor.publisher_identifier,
        );
        StringField::with_range(&PrimaryVolumeDescriptor::APPLICATION_IDENTIFIER_RANGE).write_into(
            &mut data,
            HEADER_LEN,
            &descriptor.application_identifier,
        );

        assert_eq!(data, *old_data);
        Ok(Sector { data })
    }
    #[inline]
    pub fn get_size_by_logical_block_size(logical_block_size: i16) -> usize {
        logical_block_size as usize + Self::XA_DATA_BYTE_COUNT as usize
    }
}

pub struct VolumeDescriptorLocation {
    pub descriptor_offset: u64,
    pub descriptor_type: VolumeDescriptorType,
}
#[derive(Clone, Copy, Debug, EnumIter, FromRepr, PartialEq)]
#[repr(u8)]
pub enum VolumeDescriptorType {
    Primary = 1,
    SetTerminator = 255,
}

#[cfg(test)]
mod byte_position_tests {
    use super::ByteRange;

    #[test]
    fn read_as_string() {
        // Spells out "SPYRO" + extra characters
        let buf: [u8; 8] = [0x53, 0x50, 0x59, 0x52, 0x4F, 0x20, 0x20, 0x20];
        let s = ByteRange::new(0, 5).read_as_string(&buf);
        assert_eq!(s, "SPYRO");
        assert_eq!(s.len(), 5);
    }
}

#[test]
fn serialize_root_directory_record() {
    let directory_record_for_root_directory = DirectoryRecord {
        length: 34,
        extended_attribute_record: 0,
        location_of_extent: 22,
        data_length: 2048,
        recording_date_and_time: [0x62, 0x08, 0x0D, 0x10, 0x38, 0x05, 0x24],
        file_flags: 2,
        file_unit_size: 0,
        interleave_gap_size: 0,
        volume_sequence_number: 1,
        file_identifier_length: 1,
        file_identifier: [0_u8; 255],
    };

    // Ensure correct bytes are written
    let bytes = directory_record_for_root_directory.serialize();
    assert_eq!(
        bytes,
        [
            0x22, 0x00, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x00, 0x08, 0x00, 0x00,
            0x00, 0x00, 0x08, 0x00, 0x62, 0x08, 0x0d, 0x10, 0x38, 0x05, 0x24, 0x02, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x01, 0x01, 0x00
        ]
    );
}

pub trait Identifiable {
    fn get_name(&self) -> &'static str;
}

#[derive(Debug, PartialEq)]
pub enum NumberType {
    Decimal = 10,
    Hexadecimal = 16,
}

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>
    where
        Self: Sized;
}
pub trait Unserialize {
    fn unserialize(data: &[u8]) -> Result<Self, String>
    where
        Self: Sized;
}

pub fn parse_numeric_value_as_i16(value: &str) -> Result<(NumberType, i16), String> {
    let number_type: NumberType;
    let number_value: i16;

    'init_value: {
        // If value given is a negative hexadecimal value
        if value.len() > 3 && value[0..3].eq("-0x") {
            // Make a copy of the value and remove 0x part,
            // resulting in a negative hexadecimal value
            let mut value_to_remove_chars_from = value.to_string();
            value_to_remove_chars_from.drain(1..3);

            if let Ok(value) = i16::from_str_radix(&value_to_remove_chars_from[..], 16) {
                number_type = NumberType::Hexadecimal;
                number_value = value;
                break 'init_value;
            }
        }
        // If value given is a positive hexadecimal value
        if value.len() > 2 && value[0..2].eq("0x") {
            if let Ok(value) = i16::from_str_radix(&value[2..], 16) {
                number_type = NumberType::Hexadecimal;
                number_value = value;
                break 'init_value;
            }
        }
        // If a value given is a positive decimal value
        else if value.len() > 0 {
            if let Ok(value) = i16::from_str_radix(&value[..], 10) {
                number_type = NumberType::Decimal;
                number_value = value;
                break 'init_value;
            }
        }

        return Err(String::from("Failed to parse numeric value."));
    }

    Ok((number_type, number_value))
}
pub fn parse_numeric_value_as_i32(value: &str) -> Result<(NumberType, i32), String> {
    let number_type: NumberType;
    let number_value: i32;

    let s: String = value.into();
    let len = s.len();

    'init_value: {
        // If value given is a negative hexadecimal value
        if len > 3 && s[0..3].eq("-0x") {
            // Make a copy of the value and remove 0x part,
            // resulting in a negative hexadecimal value
            let mut value_to_remove_chars_from = s.clone();
            value_to_remove_chars_from.drain(1..3);

            if let Ok(value) = i32::from_str_radix(&value_to_remove_chars_from[..], 16) {
                number_type = NumberType::Hexadecimal;
                number_value = value;
                break 'init_value;
            }
        }
        // If value given is a positive hexadecimal value
        if len > 2 && s[0..2].eq("0x") {
            if let Ok(value) = i32::from_str_radix(&s[2..], 16) {
                number_type = NumberType::Hexadecimal;
                number_value = value;
                break 'init_value;
            }
        }
        // If a value given is a positive decimal value
        else if len > 0 {
            if let Ok(value) = i32::from_str_radix(&s[..], 10) {
                number_type = NumberType::Decimal;
                number_value = value;
                break 'init_value;
            }
        }

        return Err(String::from("Failed to parse numeric value."));
    }

    Ok((number_type, number_value))
}

#[cfg(test)]
mod parse_numeric_value_tests {
    use super::{parse_numeric_value_as_i16, NumberType};

    #[test]
    fn parse_numeric_value_from_decimal_value() {
        {
            let value_raw = &String::from("9");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Decimal);
            assert_eq!(value, 9);
        }
        {
            let value_raw = &String::from("31");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Decimal);
            assert_eq!(value, 31);
        }
        {
            let value_raw = &String::from("250");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Decimal);
            assert_eq!(value, 250);
        }
        {
            let value_raw = &String::from("-250");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Decimal);
            assert_eq!(value, -250);
        }
    }
    #[test]
    fn parse_numeric_value_from_hexadecimal_value() {
        {
            let value_raw = &String::from("0x2");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Hexadecimal);
            assert_eq!(value, 2);
        }
        {
            let value_raw = &String::from("0xF");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Hexadecimal);
            assert_eq!(value, 15);
        }
        {
            let value_raw = &String::from("0x10");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Hexadecimal);
            assert_eq!(value, 16);
        }
        {
            let value_raw = &String::from("0xFF");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Hexadecimal);
            assert_eq!(value, 255);
        }
        {
            let value_raw = &String::from("0xff");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Hexadecimal);
            assert_eq!(value, 255);
        }
        {
            let value_raw = &String::from("-0x10");
            let (number_type, value) = parse_numeric_value_as_i16(&value_raw).unwrap();
            assert_eq!(number_type, NumberType::Hexadecimal);
            assert_eq!(value, -16);
        }
    }
    #[test]
    fn parse_numeric_value_from_hexadecimal_value_with_failure_when_value_is_too_big() {
        {
            let value_raw = &String::from("0xAAAA");
            assert!(parse_numeric_value_as_i16(&value_raw).is_err());
        }
        {
            let value_raw = &String::from("-0xAAAA");
            assert!(parse_numeric_value_as_i16(&value_raw).is_err());
        }
    }
}
