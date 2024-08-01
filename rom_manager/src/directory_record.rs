use std::ops::BitAnd;

use crate::{byte_range::ByteRange, fields, Serialize, Unserialize};

#[derive(Debug)]
pub enum DirectoryRecordError {
    NotEnoughBytes,
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
        let mut result: Vec<u8> = vec![0u8; self.length as usize];

        result[DirectoryRecord::LENGTH_POSITION] = self.length;
        result[DirectoryRecord::EXTENDED_ATTRIBUTE_RECORD_POSITION] =
            self.extended_attribute_record;

        fields::BothEndianI32::with_range(&DirectoryRecord::LOCATION_OF_EXTENT_RANGE).write_into(
            &mut result,
            0,
            self.location_of_extent,
        );
        fields::BothEndianI32::with_range(&DirectoryRecord::SIZE_OF_EXTENT_RANGE).write_into(
            &mut result,
            0,
            self.data_length,
        );
        fields::DateAndTime::with_range(&DirectoryRecord::RECORDING_DATE_AND_TIME_RANGE)
            .write_into(&mut result, 0, &self.recording_date_and_time);
        result[DirectoryRecord::FILE_FLAGS_POSITION] = self.file_flags;
        result[DirectoryRecord::FILE_UNIT_SIZE_POSITION] = self.file_unit_size;
        result[DirectoryRecord::INTERLEAVE_GAP_SIZE_POSITION] = self.interleave_gap_size;
        fields::BothEndianI16::with_range(&DirectoryRecord::VOLUME_SEQUENCE_NUMBER_RANGE)
            .write_into(&mut result, 0, self.volume_sequence_number);
        result[DirectoryRecord::FILE_IDENTIFIER_LENGTH_POSITION] = self.file_identifier_length;

        result
    }
}
impl Unserialize<DirectoryRecordError> for DirectoryRecord {
    fn unserialize(data: &[u8]) -> Result<Self, DirectoryRecordError> {
        let len = data.len();
        if len < 34 {
            return Err(DirectoryRecordError::NotEnoughBytes);
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
        fields::DateAndTime::format_by_bytes(&self.recording_date_and_time)
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
    #[allow(dead_code)]
    Existance = 1 << 0, // Unused, but defined here, because it exists in ECMA-119 and ISO-9660 specs.
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
    #[allow(dead_code)]
    Protection = 1 << 4, // Unused, but defined here, because it exists in ECMA-119 and ISO-9660 specs.
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
