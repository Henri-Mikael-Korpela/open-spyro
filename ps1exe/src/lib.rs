use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::str;

pub struct PS1Exe {
    data: Vec<u8>,
    /// This value will determine where code and data from 0x800 onwards
    /// will be inserted into in RAM.
    ///
    /// Usually 0x80010000 or higher in the case Playstation executable file.
    pub destination_address_in_ram: u32,
    /// 0x800 byte header is ignored in file size.
    pub file_size: u32,
    pub initial_gp_r28: u32,
    /// Program counter contains the address (location) of the instruction being executed at the current time.
    /// Initial PC value is usually 0x80010000 or higher in the case of Playstation executable file.
    pub initial_pc: u32,
    pub ascii_marker: String,
}
impl PS1Exe {
    const CODE_AND_DATA_BEGIN_OFFSET: u32 = 0x800; // 0x800 = 2048
    const PS1_EXE_NAME: &str = "Playstation executable file";
    const VALID_MULTIPLIER: usize = 2048;

    fn from_bytes(value: Vec<u8>) -> Result<Self, String> {
        // Ensure file size in a multiple of 2048
        if value.len() % Self::VALID_MULTIPLIER != 0 {
            return Err(format!(
                "{} size is not in a multiple of {} bytes.",
                Self::PS1_EXE_NAME,
                Self::VALID_MULTIPLIER
            ));
        }

        // Ensure ASCII ID, a magic marker for Playstation executable files, is present
        const PS1_EXE_HEADER_ASCII_ID: &str = "PS-X EXE";
        const PS1_EXE_HEADER_ASCII_ID_LENGTH: usize = PS1_EXE_HEADER_ASCII_ID.len();

        let ascii_id = &value[0..PS1_EXE_HEADER_ASCII_ID_LENGTH];

        let ascii_id = str::from_utf8(&ascii_id).map_err(|_| {
            format!(
                "Failed to UTF-8 encode ASCII ID \"{}\" for {}.",
                PS1_EXE_HEADER_ASCII_ID,
                Self::PS1_EXE_NAME
            )
        })?;

        if ascii_id.ne(PS1_EXE_HEADER_ASCII_ID) {
            return Err(format!(
                "{} does not contain ASCII ID \"{}\".",
                Self::PS1_EXE_NAME,
                PS1_EXE_HEADER_ASCII_ID
            ));
        }

        /// Converts LE bytes to an unsigned integer.
        #[inline]
        fn from_le_bytes_u32(value: &[u8]) -> u32 {
            u32::from_le_bytes(value.try_into().unwrap())
        }
        /// Gets a range of bytes from a given data slice.
        #[inline]
        fn get_range(data: &[u8], begin_index: usize, length_in_bytes: usize) -> &[u8] {
            &data[begin_index..begin_index + length_in_bytes]
        }

        // Read destination address in RAM
        let destination_address_in_ram = get_range(&value, 0x018, 4);
        let destination_address_in_ram = from_le_bytes_u32(destination_address_in_ram);

        // Read file size
        let file_size = get_range(&value, 0x01C, 4);
        let file_size = from_le_bytes_u32(file_size);

        // Read the initial GP (global pointer)
        let initial_gp_r28 = get_range(&value, 0x014, 4);
        let initial_gp_r28 = from_le_bytes_u32(initial_gp_r28);

        // Read the initial PC (program counter)
        let initial_pc = get_range(&value, 0x010, 4);
        let initial_pc = from_le_bytes_u32(initial_pc);

        // Read the ASCII marker. Example values:
        //
        // * "Sony Computer Entertainment Inc. for Europe area" -> PAL
        // * "Sony Computer Entertainment Inc. for Japan area" -> NTSC
        // * "Sony Computer Entertainment Inc. for North America area" -> NTSC
        //
        // May be zerofilled in some homebrew files.
        // The BIOS doesn't verify this string and boots fine without it.
        let begin_index = 0x04C;
        let max_length = 0x7FF - begin_index;
        let ascii_marker_bytes = get_range(&value, begin_index, max_length);

        let ascii_marker: String;
        'ascii_marker_search: {
            for i in 0..max_length {
                // Value 0 stands for null termination character in this case
                if ascii_marker_bytes[i] == 0 {
                    match str::from_utf8(&ascii_marker_bytes[0..i]) {
                        Ok(ascii_marker_as_str) => {
                            ascii_marker = ascii_marker_as_str.to_string();
                            break 'ascii_marker_search;
                        }
                        _ => {
                            return Err(String::from(
                                "Failed to convert bytes ASCII marker with UTF-8 encoding.",
                            ));
                        }
                    }
                }
            }

            return Err(String::from(
                "No null termination character found when reading ASCII marker.",
            ));
        }

        Ok(Self {
            data: value,
            destination_address_in_ram,
            file_size,
            initial_gp_r28,
            initial_pc,
            ascii_marker,
        })
    }
    pub fn from_file_path(file_path: &str) -> Result<Self, String> {
        let ps1_exe_file = File::open(file_path).map_err(|_| {
            format!(
                "Failed to open given Playstation executable file in path \"{}\".",
                file_path
            )
        })?;

        let mut reader = BufReader::new(ps1_exe_file);

        let mut data = Vec::new();
        reader.read_to_end(&mut data).map_err(|err| {
            format!(
                "Failed to read given Playstation executable file in path \"{}\": {}",
                file_path,
                err.to_string()
            )
        })?;

        Self::from_bytes(data)
    }
    fn get_address_by_address_in_memory(&self, address: u64) -> usize {
        address as usize - self.destination_address_in_ram as usize
            + Self::CODE_AND_DATA_BEGIN_OFFSET as usize
    }
}

pub struct PS1ExeWriter<'a> {
    exe: &'a mut PS1Exe,
}
impl<'a> PS1ExeWriter<'a> {
    pub fn new(exe: &'a mut PS1Exe) -> Self {
        Self { exe }
    }
    pub fn write_code(&mut self, address_in_memory: u64, code: &[u8]) {
        let address = self.exe.get_address_by_address_in_memory(address_in_memory);

        // Debug printing to ensure the correct address is being written to
        println!(
            "Writing code {:?} to {:?}...",
            code,
            &self.exe.data[address..address + code.len()]
        );

        // Overwrite bytes in the executable file with given code
        self.exe.data[address..address + code.len()].copy_from_slice(code);
    }
    pub fn write_into_file(&self, file_path: &str) -> Result<(), String> {
        let mut file = File::create(file_path).map_err(|err| {
            format!(
                "Failed to make a Playstation executable file to path \"{}\": {}",
                file_path,
                err.to_string()
            )
        })?;
        file.write_all(&self.exe.data).map_err(|err| {
            format!(
                "Failed to write to Playstation executable file to path \"{}\": {}",
                file_path,
                err.to_string()
            )
        })?;
        Ok(())
    }
}
