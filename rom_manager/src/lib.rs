use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

use primary_volume_descriptor::PrimaryVolumeDescriptor;
use strum::IntoEnumIterator;
use strum::{EnumIter, FromRepr};

mod byte_range;
mod directory_record;
mod fields;
mod primary_volume_descriptor;

use directory_record::DirectoryRecord;

fn write_bytes_into(data: &mut [u8], begin: usize, value: &[u8]) {
    for (i, b) in data[begin..begin + value.len()].iter_mut().enumerate() {
        *b = value[i];
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
            .find(|l| l.descriptor_type == VolumeDescriptorType::Primary)
        else {
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

        fields::StringField::with_range(&PrimaryVolumeDescriptor::VOLUME_IDENTIFIER_RANGE)
            .write_into(&mut data, HEADER_LEN, &descriptor.volume_identifier);
        fields::BothEndianI32::with_range(&PrimaryVolumeDescriptor::VOLUME_SPACE_SIZE_RANGE)
            .write_into(&mut data, HEADER_LEN, descriptor.volume_space_size);
        fields::BothEndianI16::with_range(&PrimaryVolumeDescriptor::LOGICAL_BLOCK_SIZE_RANGE)
            .write_into(&mut data, HEADER_LEN, descriptor.logical_block_size);
        fields::LittleEndian(&PrimaryVolumeDescriptor::LOCATION_OF_TYPE_L_PATH_TABLE_RANGE)
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
        fields::StringField::with_range(&PrimaryVolumeDescriptor::PUBLISHER_IDENTIFIER_RANGE)
            .write_into(&mut data, HEADER_LEN, &descriptor.publisher_identifier);
        fields::StringField::with_range(&PrimaryVolumeDescriptor::APPLICATION_IDENTIFIER_RANGE)
            .write_into(&mut data, HEADER_LEN, &descriptor.application_identifier);

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
    use crate::byte_range::ByteRange;

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
