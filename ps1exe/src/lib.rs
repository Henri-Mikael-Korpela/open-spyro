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
    const PS1_EXE_NAME: &'static str = "Playstation executable file";
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

pub struct PS1ExeReader<'a> {
    exe: &'a PS1Exe,
}
impl<'a> PS1ExeReader<'a> {
    pub fn disassemble_at_adress_by_count(&self, address_in_memory: u64, instruction_count: usize) {
        let address = self.exe.get_address_by_address_in_memory(address_in_memory);
        const INSTRUCTION_LEN_IN_BYTES: usize = 4;
        for i in 0..instruction_count {
            let instruction_bytes = &self.exe.data[address + i * INSTRUCTION_LEN_IN_BYTES
                ..address + (i + 1) * INSTRUCTION_LEN_IN_BYTES];
            let instruction_bytes: &[u8; 4] = instruction_bytes.try_into().unwrap();
            let instruction = mips::Instruction::parse_from_le_bytes(instruction_bytes);
            println!("{}", instruction.to_instruction());
        }
    }
    pub fn disassemble_str_at_address_until_byte(
        &'a self,
        address_in_memory: u64,
        end_byte: u8,
    ) -> Result<&'a str, String> {
        const MAX_STR_LENGTH: usize = 256;

        let address = self.exe.get_address_by_address_in_memory(address_in_memory);
        let bytes_buffer = &self.exe.data[address..address + MAX_STR_LENGTH];

        for i in 0..MAX_STR_LENGTH {
            if bytes_buffer[i] == end_byte {
                let str_bytes = &bytes_buffer[0..i];
                let str = str::from_utf8(str_bytes).map_err(|err| {
                    format!(
                        "Failed to UTF-8 encode a string at address 0x{:X}: {}. Bytes found: {:?}",
                        address_in_memory,
                        err.to_string(),
                        str_bytes
                    )
                })?;
                return Ok(str);
            }
        }

        Err(format!("Maximum length for requested string reaching while disassemblying: no end byte {} found", end_byte))
    }
    pub fn new(exe: &'a PS1Exe) -> Self {
        Self { exe }
    }
}

pub struct PS1ExeWriter<'a> {
    /// Stores information about how much of the executable
    /// bytes has been written into via this writer.
    data_written: Vec<u8>,
    exe: &'a mut PS1Exe,
}
impl<'a> PS1ExeWriter<'a> {
    pub fn get_percentage_of_written_bytes(&self) -> f32 {
        let mut written_bytes_count = 0;
        for i in 0..self.data_written.len() {
            if self.data_written[i] != 0 {
                written_bytes_count += 1;
            }
        }
        written_bytes_count as f32 / self.data_written.len() as f32
    }
    pub fn new(exe: &'a mut PS1Exe) -> Self {
        Self {
            data_written: Vec::new(),
            exe,
        }
    }
    pub fn write_code(&mut self, address_in_memory: u64, code: &[u8]) -> PS1ExeWriteResult {
        // By default, data_written vector has nothing in it, it is uninitialized.
        // So, initialize data_written vector if it hasn't been initialized yet.
        if self.data_written.len() == 0 {
            let mut data_len = self.exe.data.len();
            self.data_written = Vec::with_capacity(data_len);
            while data_len > 0 {
                self.data_written.push(0); // 0 means that the byte hasn't been written into yet.
                data_len -= 1;
            }
        }

        let address = self.exe.get_address_by_address_in_memory(address_in_memory);

        // Mark bytes as written into based on the given address and code length
        for i in address..address + code.len() {
            if self.data_written[i] == 0 {
                self.data_written[i] = 1;
            }
        }

        let bytes_to_write_into = &mut self.exe.data[address..address + code.len()];

        let result = if *code == *bytes_to_write_into {
            PS1ExeWriteResult::Unchanged
        } else {
            PS1ExeWriteResult::Changed {
                original_code: bytes_to_write_into.to_vec(),
            }
        };

        // Overwrite bytes in the executable file with given code
        bytes_to_write_into.copy_from_slice(code);

        result
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

pub enum PS1ExeWriteResult {
    Changed { original_code: Vec<u8> },
    Unchanged,
}
