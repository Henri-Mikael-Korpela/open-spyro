use crate::byte_range::{ByteCount, ByteRange};

pub(crate) struct BothEndianI16<'a>(&'a ByteRange);
impl<'a> BothEndianI16<'a> {
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

pub(crate) struct BothEndianI32<'a>(pub &'a ByteRange);
impl<'a> BothEndianI32<'a> {
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

pub(crate) struct DateAndTime<'a>(pub &'a ByteRange);
impl<'a> DateAndTime<'a> {
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

pub(crate) struct LittleEndian<'a>(pub &'a ByteRange);
impl<'a> LittleEndian<'a> {
    #[allow(dead_code)]
    pub fn with_range(range: &'a ByteRange) -> Self {
        Self(&range)
    }
    pub fn write_into(&self, data: &mut [u8], begin_in_data: usize, value_to_write: i32) {
        for (i, b) in value_to_write.to_le_bytes().iter().enumerate() {
            data[begin_in_data + self.0.begin + i] = *b;
        }
    }
}

pub(crate) struct StringField<'a>(pub &'a ByteRange);
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

#[test]
fn write_into_array_as_both_endian_i16() {
    let mut data = [0_u8; 4];

    let field = BothEndianI16::with_range(&ByteRange { begin: 0, end: 1 });
    field.write_into(&mut data, 0, 16);

    assert_eq!(data, [16, 0, 0, 16]);
}
#[test]
fn write_into_array_as_both_endian_i32() {
    let mut data = [0_u8; 8];

    let field = BothEndianI32::with_range(&ByteRange { begin: 0, end: 1 });
    field.write_into(&mut data, 0, 16);

    assert_eq!(data, [16, 0, 0, 0, 0, 0, 0, 16]);
}
