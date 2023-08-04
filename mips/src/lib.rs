use core::panic;

macro_rules! define_i_signed_instruction_parse {
    ($parts:ident, $opcode:literal) => {
        match $parts[1..] {
            [rt, rs, immediate] => {
                let rt = parse_register(rt).map_err(|e| e.to_string())?;
                let rs = parse_register(rs).map_err(|e| e.to_string())?;
                let immediate = parse_immediate_signed(immediate).map_err(|e| e.to_string())?;
                Ok(Instruction::ISigned {
                    opcode: $opcode,
                    rs,
                    rt,
                    immediate,
                })
            }
            _ => panic!("Unknown structure for instruction \"{}\"", $parts[0]),
        }
    };
}
macro_rules! define_i_unsigned_instruction_parse {
    ($parts:ident, $opcode:literal) => {
        match $parts[1..] {
            [rt, rs, immediate] => {
                let rs = parse_register(rs).map_err(|e| e.to_string())?;
                let rt = parse_register(rt).map_err(|e| e.to_string())?;
                let immediate = parse_immediate_unsigned(immediate).map_err(|e| e.to_string())?;
                Ok(Instruction::IUnsigned {
                    opcode: $opcode,
                    rs,
                    rt,
                    immediate,
                })
            }
            _ => panic!("Unknown structure for instruction \"{}\"", $parts[0]),
        }
    };
}
macro_rules! define_r_instruction_parse {
    ($parts:ident, $funct:literal) => {
        match $parts[1..] {
            [rd, rs, rt] => {
                let rd = parse_register(rd).map_err(|e| e.to_string())?;
                let rs = parse_register(rs).map_err(|e| e.to_string())?;
                let rt = parse_register(rt).map_err(|e| e.to_string())?;
                Ok(Instruction::R {
                    opcode: 0,
                    rs,
                    rt,
                    rd,
                    shamt: 0,
                    funct: $funct,
                })
            }
            _ => panic!("Unknown structure for instruction \"{}\"", $parts[0]),
        }
    };
}

const REGISTERS: &[&'static str; 32] = &[
    "zero", // Constant 0
    "at",   // Assembler temporary (reserved for assembler)
    "v0",   // Return value, stores result of a function call
    "v1",   // Return value, stores result of a function call
    "a0",   // Function argument
    "a1",   // Function argument
    "a2",   // Function argument
    "a3",   // Function argument
    "t0",   // Temporary
    "t1",   // Temporary
    "t2",   // Temporary
    "t3",   // Temporary
    "t4",   // Temporary
    "t5",   // Temporary
    "t6",   // Temporary
    "t7",   // Temporary
    "s0",   // Saved temporary
    "s1",   // Saved temporary
    "s2",   // Saved temporary
    "s3",   // Saved temporary
    "s4",   // Saved temporary
    "s5",   // Saved temporary
    "s6",   // Saved temporary
    "s7",   // Saved temporary
    "t8",   // Temporary
    "t9",   // Temporary
    "k0",   // Kernel reserved
    "k1",   // Kernel reserved
    "gp",   // Global pointer
    "sp",   // Stack pointer
    "fp",   // Frame pointer
    "ra",   // Return address
];

#[derive(Debug, PartialEq)]
pub enum Instruction {
    IExceptional {
        opcode: u8,
        rt: u8,
        value: u8,
        immediate: i16,
    },
    ISigned {
        opcode: u8,
        rs: u8,
        rt: u8,
        immediate: i16,
    },
    IUnsigned {
        opcode: u8,
        rs: u8,
        rt: u8,
        immediate: u16,
    },
    R {
        opcode: u8,
        rs: u8,
        rt: u8,
        rd: u8,
        shamt: u8,
        funct: u8,
    },
}

impl Instruction {
    pub fn parse_from_be_bytes(content: &[u8; 4]) -> Self {
        let machine_code = u32::from_be_bytes(*content);
        let opcode = (machine_code >> 26) as u8;
        match opcode {
            0 => {
                // R-type
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rd = ((machine_code >> 11) & 0b11111) as u8;
                let shamt = ((machine_code >> 6) & 0b11111) as u8;
                let funct = (machine_code & 0b111111) as u8;
                Instruction::R {
                    opcode,
                    rs,
                    rt,
                    rd,
                    shamt,
                    funct,
                }
            }
            0b001000 => {
                // addi
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let immediate2 = (machine_code & 0xFFFF) as u32; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                let immediate = immediate2 as u16 as i16;
                Instruction::ISigned {
                    opcode,
                    rs,
                    rt,
                    immediate,
                }
            }
            0b001001 => {
                // addiu
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let immediate = (machine_code & 0xFFFF) as u16; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                Instruction::IUnsigned {
                    opcode,
                    rs,
                    rt,
                    immediate,
                }
            }
            0b00101000 => {
                // slti
                let value = ((machine_code >> 21) & 0b11111) as u8;
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let immediate = (machine_code & 0xFFFF) as i16; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                Instruction::IExceptional {
                    opcode,
                    rt,
                    value,
                    immediate,
                }
            }
            _ => panic!("Unknown opcode \"{:b}\" ({})", opcode, opcode),
        }
    }
    #[inline]
    pub fn parse_from_machine_code(machine_code: u32) -> Self {
        Self::parse_from_be_bytes(&machine_code.to_be_bytes())
    }
    pub fn parse_from_le_bytes(content: &[u8; 4]) -> Self {
        let mut content_reversed = content.clone();
        content_reversed.reverse();
        Self::parse_from_be_bytes(&content_reversed)
    }
    /// Parses an instruction from a string.
    pub fn parse_from_str(content: &str) -> Result<Self, String> {
        let content_sanitized = content.replace(",", " ").replace("  ", " ");
        let parts = content_sanitized.split(" ").collect::<Vec<_>>();
        match parts[0] {
            "add" => define_r_instruction_parse!(parts, 0b100000), // Funct is 32
            "addi" => define_i_signed_instruction_parse!(parts, 0b001000), // Opcode is 8
            "addiu" => define_i_unsigned_instruction_parse!(parts, 0b001001), // Opcode is 9
            "addu" => define_r_instruction_parse!(parts, 0b100001), // Funct is 33
            "and" => define_r_instruction_parse!(parts, 0b100100), // Funct is 36
            "andi" => define_i_signed_instruction_parse!(parts, 0b001100), // Opcode is 12
            "lui" => match parts[1..] {
                [rt, immediate] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));
                    let immediate =
                        parse_immediate_unsigned(immediate).unwrap_or_else(|e| panic!("{}", e));
                    Ok(Instruction::IUnsigned {
                        opcode: 0b001111, // Opcode is 15
                        rs: 0,
                        rt,
                        immediate,
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
            "lw" => match parts[1..] {
                [rt, immediate, rs] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));
                    let rs = parse_register(rs).unwrap_or_else(|e| panic!("{}", e));
                    let immediate =
                        parse_immediate_signed(immediate).unwrap_or_else(|e| panic!("{}", e));
                    Ok(Instruction::ISigned {
                        opcode: 0b100011, // Opcode is 35
                        rs,
                        rt,
                        immediate,
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
            "nor" => define_r_instruction_parse!(parts, 0b100111), // Funct is 39
            "or" => define_r_instruction_parse!(parts, 0b100101),  // Funct is 37
            "sb" => {
                let rt = parse_register(parts[1]).unwrap_or_else(|e| panic!("{}", e));
                let value = parts[2].parse::<u8>().unwrap_or_else(|e| panic!("{}", e));
                let immediate =
                    parse_immediate_signed(parts[3]).unwrap_or_else(|e| panic!("{}", e));
                Ok(Instruction::IExceptional {
                    opcode: 0b101000, // Opcode is 40
                    rt,
                    value,
                    immediate,
                })
            }
            "slt" => define_r_instruction_parse!(parts, 0b101010), // Funct is 42
            "slti" => define_i_signed_instruction_parse!(parts, 0b001010), // Opcode is 10
            "sltu" => define_r_instruction_parse!(parts, 0b101011), // Funct is 43
            "sub" => define_r_instruction_parse!(parts, 0b100010), // Funct is 34
            "subu" => define_r_instruction_parse!(parts, 0b100011), // Funct is 35
            "sw" => match parts[1..] {
                [rt, immediate, rs] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));
                    let rs = parse_register(rs).unwrap_or_else(|e| panic!("{}", e));
                    let immediate =
                        parse_immediate_signed(immediate).unwrap_or_else(|e| panic!("{}", e));
                    Ok(Instruction::ISigned {
                        opcode: 0b101011, // Opcode is 43
                        rs,
                        rt,
                        immediate,
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
            _ => panic!("Unknown instruction \"{}\"", parts[0]),
        }
    }
    pub fn to_instruction(&self) -> String {
        match self {
            Instruction::IExceptional {
                opcode,
                rt,
                value,
                immediate,
            } => match opcode {
                0b101000 => format!("sb ${}, {}(${})", rt, value, immediate),
                _ => panic!("Unknown opcode \"{}\" for IExceptional instruction", opcode),
            },
            Instruction::ISigned {
                opcode,
                rs,
                rt,
                immediate,
            } => {
                let rt = REGISTERS[*rt as usize];
                let rs = REGISTERS[*rs as usize];

                match opcode {
                    0b001000 => format!("addi {}, {}, {}", rt, rs, immediate),
                    0b001100 => format!("andi {}, {}, {}", rt, rs, immediate),
                    0b001111 => format!("lui {}, {}", rt, immediate),
                    0b100011 => format!("lw {}, {}({})", rt, immediate, rs),
                    0b001010 => format!("slti {}, {}, {}", rt, rs, immediate),
                    0b101011 => format!("sw {}, {}({})", rt, immediate, rs),
                    _ => panic!("Unknown opcode \"{}\" for I instruction", opcode),
                }
            }
            Instruction::IUnsigned {
                opcode,
                rs,
                rt,
                immediate,
            } => {
                let rt = REGISTERS[*rt as usize];
                let rs = REGISTERS[*rs as usize];

                match opcode {
                    0b001001 => format!("addiu {}, {}, {}", rt, rs, immediate),
                    0b001111 => format!("lui {}, {}", rt, immediate),
                    _ => panic!("Unknown opcode \"{}\" for I instruction", opcode),
                }
            }
            Instruction::R {
                rs, rt, rd, funct, ..
            } => {
                let rd = REGISTERS[*rd as usize];
                let rs = REGISTERS[*rs as usize];
                let rt = REGISTERS[*rt as usize];

                match funct {
                    0b100000 => format!("add {}, {}, {}", rd, rs, rt),
                    0b100001 => format!("addu {}, {}, {}", rd, rs, rt),
                    0b100100 => format!("and {}, {}, {}", rd, rs, rt),
                    0b100111 => format!("nor {}, {}, {}", rd, rs, rt),
                    0b100101 => format!("or {}, {}, {}", rd, rs, rt),
                    0b101010 => format!("slt {}, {}, {}", rd, rs, rt),
                    0b101011 => format!("sltu {}, {}, {}", rd, rs, rt),
                    0b100010 => format!("sub {}, {}, {}", rd, rs, rt),
                    0b100011 => format!("subu {}, {}, {}", rd, rs, rt),
                    _ => panic!("Unknown funct \"{:b}\" ({})", funct, funct),
                }
            }
        }
    }
    #[inline]
    pub fn to_be_bytes(&self) -> [u8; 4] {
        self.to_machine_code().to_be_bytes()
    }
    /// Converts the instruction to machine code.
    /// The machine code is returned as a 32-bit unsigned integer.
    /// The machine code is in big-endian format.
    #[inline]
    pub fn to_machine_code(&self) -> u32 {
        match self {
            Instruction::IExceptional {
                opcode,
                rt,
                value,
                immediate,
            } => {
                let mut machine_code = 0u32;
                machine_code |= (*opcode as u32) << 26;
                machine_code |= (*rt as u32) << 21;
                machine_code |= (*value as u32) << 16;
                machine_code |= *immediate as u16 as u32;
                machine_code
            }
            Instruction::ISigned {
                opcode,
                rs,
                rt,
                immediate,
            } => {
                let mut machine_code = 0u32;
                machine_code |= (*opcode as u32) << 26;
                machine_code |= (*rs as u32) << 21;
                machine_code |= (*rt as u32) << 16;
                machine_code |= *immediate as u16 as u32;
                machine_code
            }
            Instruction::IUnsigned {
                opcode,
                rs,
                rt,
                immediate,
            } => {
                let mut machine_code = 0u32;
                machine_code |= (*opcode as u32) << 26;
                machine_code |= (*rs as u32) << 21;
                machine_code |= (*rt as u32) << 16;
                machine_code |= *immediate as u32;
                machine_code
            }
            Instruction::R {
                opcode,
                rs,
                rt,
                rd,
                shamt,
                funct,
            } => {
                let mut machine_code = 0u32;
                machine_code |= (*opcode as u32) << 26;
                machine_code |= (*rs as u32) << 21;
                machine_code |= (*rt as u32) << 16;
                machine_code |= (*rd as u32) << 11;
                machine_code |= (*shamt as u32) << 6;
                machine_code |= *funct as u32;
                machine_code
            }
        }
    }
    #[inline]
    pub fn to_le_bytes(&self) -> [u8; 4] {
        self.to_machine_code().to_le_bytes()
    }
}

/// Parses an immediate value from a string.
/// The string must be in the format "0x<hexadecimal number>" or "<decimal number>".
/// If the string is not in the correct format, an error is returned.
fn parse_immediate_signed(content: &str) -> Result<i16, String> {
    if content.starts_with("0x") || content.starts_with("0X") {
        i16::from_str_radix(&content[2..], 16)
            .map_err(|e| format!("Could not parse immediate \"{}\": {}", content, e))
    } else if content.starts_with("-0x") || content.starts_with("-0X") {
        let content_without_0x = String::from("-") + &content[3..];
        i16::from_str_radix(&content_without_0x, 16)
            .map_err(|e| format!("Could not parse immediate \"{}\": {}", content, e))
    } else {
        content
            .parse::<i16>()
            .map_err(|e| format!("Could not parse immediate \"{}\": {}", content, e))
    }
}

/// Parses an immediate value from a string.
/// The string must be in the format "0x<hexadecimal number>" or "<decimal number>".
/// If the string is not in the correct format, an error is returned.
fn parse_immediate_unsigned(content: &str) -> Result<u16, String> {
    if content.starts_with("0x") || content.starts_with("0X") {
        u16::from_str_radix(&content[2..], 16).map_err(|e| {
            format!(
                "Could not parse immediate \"{}\": {}",
                content,
                e.to_string()
            )
        })
    } else if content.starts_with("-") {
        Err(format!(
            "Could not parse immediate \"{}\": {}",
            content, "Negative number given when only unsigned immediate is expected"
        ))
    } else {
        content.parse::<u16>().map_err(|e| {
            format!(
                "Could not parse immediate \"{}\": {}",
                content,
                e.to_string()
            )
        })
    }
}
/// Parses an immediate value from a string.
/// The string must be in the format "0x<hexadecimal number>" or "<decimal number>".
/// If the string is not in the correct format, an error is returned.
fn parse_immediate_unsigned_u64(content: &str) -> Result<u64, String> {
    if content.starts_with("0x") || content.starts_with("0X") {
        u64::from_str_radix(&content[2..], 16).map_err(|e| {
            format!(
                "Could not parse immediate \"{}\": {}",
                content,
                e.to_string()
            )
        })
    } else if content.starts_with("-") {
        Err(format!(
            "Could not parse immediate \"{}\": {}",
            content, "Negative number given when only unsigned immediate is expected"
        ))
    } else {
        content.parse::<u64>().map_err(|e| {
            format!(
                "Could not parse immediate \"{}\": {}",
                content,
                e.to_string()
            )
        })
    }
}
pub fn parse_nodes(content: &str) -> Result<Vec<Node>, String> {
    let content_lines = content.split("\n");
    let mut nodes = Vec::new();

    for line in content_lines {
        let line = line.trim();

        // If the line is empty, skip the line.
        if line.is_empty() {
            continue;
        }
        // If the line contains a comment, skip the line.
        else if line.starts_with("#") {
            continue;
        }
        // If the line contains a label, store a label node.
        else if line.ends_with(":") {
            let label = line[..line.len() - 1].to_string();
            nodes.push(Node {
                kind: NodeKind::Label(label),
            });
        }
        // If the line contains a custom command (starting with '@'), store a custom command node.
        else if line.starts_with("@") {
            let custom_command_parts = line[1..].split(" ").collect::<Vec<&str>>();

            match &custom_command_parts[..] {
                ["at", address] => {
                    let address = parse_immediate_unsigned_u64(&address)?;
                    nodes.push(Node {
                        kind: NodeKind::CustomCommand(CustomCommand::At(address)),
                    });
                }
                _ => {
                    return Err(format!(
                        "Could not parse custom command \"{}\": {}",
                        line, "Unknown custom command"
                    ));
                }
            }
        }
        // If the line (should) contain an instruction, store an instruction node.
        else {
            let instruction = Instruction::parse_from_str(line)?;
            nodes.push(Node {
                kind: NodeKind::Instruction(instruction),
            });
        }
    }

    Ok(nodes)
}
/// Parses a register from a string.
///
/// The string must be in the format "$<register_number>".
/// The register number must be between 0 and 31.
/// If the register number is out of range, an error is returned.
/// If the string is not in the correct format, an error is returned.
/// If the string is in the correct format and the register number is in range, the register number is returned.
fn parse_register(content: &str) -> Result<u8, String> {
    if content.starts_with("$") {
        let register_number = content[1..].parse::<u8>().unwrap();
        if register_number > 31 {
            Err(format!(
                "Register number given ({}) is out of range (0-31)",
                register_number
            ))
        } else {
            Ok(register_number)
        }
    } else {
        for (i, register) in REGISTERS.iter().enumerate() {
            if content == *register {
                return Ok(i as u8);
            }
        }

        let content_as_number = content.parse::<u8>();
        match content_as_number {
            Ok(valid_number) => {
                if valid_number > 31 {
                    Err(format!(
                        "Unknown register \"{}\". Maybe you meant \"${}\" (also make sure the register in range 0-31)?",
                        valid_number,
                        valid_number
                    ))
                } else {
                    Err(format!(
                        "Unknown register \"{}\". . Maybe you meant \"${}\"?",
                        content, content
                    ))
                }
            }
            Err(_) => Err(format!("Unknown register \"{}\".", content)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_negative_immediate_from_signed_value() {
        let result = parse_immediate_signed("-32");
        assert_eq!(result, Ok(-32));

        let result = parse_immediate_signed("-0x20"); // -32 = 0xFFE0
        assert_eq!(result, Ok(-32));
    }
    #[test]
    fn parse_positive_immediate_from_signed_value() {
        let result = parse_immediate_signed("32");
        assert_eq!(result, Ok(32));

        let result = parse_immediate_signed("0x20"); // 32 = 0x20
        assert_eq!(result, Ok(32));
    }

    #[test]
    fn disassemble_add_instruction_from_machine_code() {
        let instruction = Instruction::parse_from_machine_code(0x012A4020);
        println!("{}", instruction.to_instruction());
        assert_eq!(instruction.to_instruction(), "add t0, t1, t2");
    }
    #[test]
    fn parse_add_instruction_from_bytes() {
        let result_bin = 0b00000001001010100100000000100000;
        let result_hex = 0x012A4020;
        assert_eq!(result_bin, result_hex);

        let instruction = Instruction::parse_from_be_bytes(&[0x01, 0x2A, 0x40, 0x20]);
        assert_eq!(instruction.to_machine_code(), result_bin);

        let instruction = Instruction::parse_from_le_bytes(&[0x20, 0x40, 0x2A, 0x01]);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
    #[test]
    fn parse_add_instruction_from_machine_code() {
        // add t0, t1, t2
        let instruction = Instruction::parse_from_machine_code(0x012A4020);
        let result_bin = 0b00000001001010100100000000100000;
        let result_hex = 0x012A4020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
    #[test]
    fn parse_add_instruction_from_string() {
        let instruction = Instruction::parse_from_str("add t0, t1, t2").unwrap();
        let result_bin = 0b00000001001010100100000000100000;
        let result_hex = 0x012A4020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }

    #[test]
    fn disassemble_addi_instruction_from_machine_code_with_positive_immediate() {
        let instruction = Instruction::parse_from_machine_code(0x21280020);
        assert_eq!(instruction.to_instruction(), "addi t0, t1, 32"); // 32 = 0x20
    }
    #[test]
    fn parse_addi_instruction_from_bytes() {
        // addi t0, t1, 0x20
        let result_bin = 0b00100001001010000000000000100000;
        let result_hex = 0x21280020;
        assert_eq!(result_bin, result_hex);

        let instruction = Instruction::parse_from_be_bytes(&[0x21, 0x28, 0x00, 0x20]);
        assert_eq!(instruction.to_machine_code(), result_bin);

        let instruction = Instruction::parse_from_le_bytes(&[0x20, 0x00, 0x28, 0x21]);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
    #[test]
    fn parse_addi_instruction_from_machine_code() {
        // addi t0, t1, 0x20
        let instruction = Instruction::parse_from_machine_code(0x21280020);
        let result_bin = 0b00100001001010000000000000100000;
        let result_hex = 0x21280020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
    #[test]
    fn parse_addi_instruction_from_string_with_negative_immediate() {
        // addi t0, t1, -0x20
        let instruction = Instruction::parse_from_str("addi t0, t1, -0x20").unwrap();
        let result_bin = 0b00100001001010001111111111100000;
        let result_hex = 0x2128FFE0;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
    #[test]
    fn parse_addi_instruction_from_string_with_positive_immediate() {
        // addi t0, t1, 0x20
        let instruction = Instruction::parse_from_str("addi t0, t1, 0x20").unwrap();
        let result_bin = 0b00100001001010000000000000100000;
        let result_hex = 0x21280020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }

    #[test]
    fn disassemble_addiu_instruction_from_machine_code_with_positive_immediate() {
        let instruction = Instruction::parse_from_machine_code(0x25280020);
        assert_eq!(instruction.to_instruction(), "addiu t0, t1, 32"); // 32 = 0x20
    }
    #[test]
    fn parse_addiu_instruction_from_bytes() {
        // addiu t0, t1, 0x20
        let result_bin = 0b00100101001010000000000000100000;
        let result_hex = 0x25280020;
        assert_eq!(result_bin, result_hex);

        let instruction = Instruction::parse_from_be_bytes(&[0x25, 0x28, 0x00, 0x20]);
        assert_eq!(instruction.to_machine_code(), result_bin);

        let instruction = Instruction::parse_from_le_bytes(&[0x20, 0x00, 0x28, 0x25]);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
    #[test]
    fn parse_addiu_instruction_from_machine_code() {
        // addiu t0, t1, 0x20
        let instruction = Instruction::parse_from_machine_code(0x25280020);
        let result_bin = 0b00100101001010000000000000100000;
        let result_hex = 0x25280020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
    #[test]
    fn parse_addiu_instruction_from_string_with_positive_immediate() {
        // addiu t0, t1, 0x20
        let instruction = Instruction::parse_from_str("addiu t0, t1, 0x20").unwrap();
        let result_bin = 0b00100101001010000000000000100000;
        let result_hex = 0x25280020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
    #[test]
    fn fail_parse_addiu_instruction_from_string_with_negative_immediate() {
        // addiu t0, t1, -0x20
        let instruction = Instruction::parse_from_str("addiu t0, t1, -0x20");
        assert!(instruction.is_err());
    }

    #[test]
    fn parse_lui_instruction_from_machine_code() {
        // lui t0, 0x20
        let instruction = Instruction::parse_from_machine_code(0x3C080020);
        let result_bin = 0b00111100000100000000000000100000;
        let result_hex = 0x3C080020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);

        // lui v0, 0x8007
        let instruction = Instruction::parse_from_machine_code(0x3C028007);
        let result_bin = 0b00111100000000101000000000000111;
        let result_hex = 0x3C028007;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }
}

#[derive(Debug, PartialEq)]
pub enum CustomCommand {
    At(u64),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
}
#[derive(Debug, PartialEq)]
pub enum NodeKind {
    CustomCommand(CustomCommand),
    Instruction(Instruction),
    Label(String),
}
