use std::str;

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
        str::from_utf8(&slice[..]).unwrap().to_string()
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
