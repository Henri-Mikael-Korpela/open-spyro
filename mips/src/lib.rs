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
    J {
        opcode: u8,
        address: u32,
    },
    Nop,
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
        if *content == [0, 0, 0, 0] {
            return Instruction::Nop;
        }

        let machine_code = u32::from_be_bytes(*content);
        let opcode = (machine_code >> 26) as u8;
        match opcode {
            // R-type instruction where opcode is always 0
            0 => {
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
            // addi, opcode 8
            0b001000 => {
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
            // addiu, opcode 9
            0b001001 => {
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
            // beq, opcode 4
            0b000100 => {
                // Parse as an exceptional I instruction
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let immediate2 = (machine_code & 0xFFFF) as u32; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                let immediate = immediate2 as u16 as i16;
                Instruction::IExceptional {
                    opcode,
                    rt,
                    value: rs,
                    immediate,
                }
            }
            // bne, opcode 5
            0b000101 => {
                // Parse as an exceptional I instruction
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let immediate2 = (machine_code & 0xFFFF) as u32; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                let immediate = immediate2 as u16 as i16;
                Instruction::IExceptional {
                    opcode,
                    rt,
                    value: rs,
                    immediate,
                }
            }
            // j, opcode 2
            0b000010 => {
                let address = machine_code & 0x3FFFFFF;
                Instruction::J { opcode, address }
            }
            // jal, opcode 3
            0b000011 => {
                let address = machine_code & 0x3FFFFFF;
                Instruction::J { opcode, address }
            }
            // lb, opcode 32
            0b100000 => {
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let immediate = (machine_code & 0xFFFF) as i16; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                Instruction::ISigned {
                    opcode,
                    rs,
                    rt,
                    immediate,
                }
            }
            // lui, opcode 15
            0b001111 => {
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let immediate = (machine_code & 0xFFFF) as u16; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                Instruction::IUnsigned {
                    opcode,
                    rs: 0,
                    rt,
                    immediate,
                }
            }
            // lw, opcode 35
            0b100011 => {
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let immediate = (machine_code & 0xFFFF) as i16; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                Instruction::ISigned {
                    opcode,
                    rs,
                    rt,
                    immediate,
                }
            }
            // sb, opcode 40
            0b101000 => {
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let immediate = (machine_code & 0xFFFF) as i16; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                Instruction::ISigned {
                    opcode,
                    rs,
                    rt,
                    immediate,
                }
            }
            // slti, opcode 10
            0b001010 => {
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
            // sw, opcode 43
            0b00101011 => {
                let rt = ((machine_code >> 16) & 0b11111) as u8;
                let rs = ((machine_code >> 21) & 0b11111) as u8;
                let immediate = (machine_code & 0xFFFF) as i16; // 0xFFFF = 65535 = 2^16 - 1 = 0b1111111111111111
                Instruction::ISigned {
                    opcode,
                    rs,
                    rt,
                    immediate,
                }
            }
            _ => panic!("Unknown opcode {:b} (in bin), {} (in dec)", opcode, opcode),
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
            "beq" => match parts[1..] {
                [rt, rs, address] => {
                    Ok(Instruction::IExceptional {
                        opcode: 0b000100, // Opcode is 4
                        value: parse_register(rs).unwrap_or_else(|e| panic!("{}", e)),
                        rt: parse_register(rt).unwrap_or_else(|e| panic!("{}", e)),
                        immediate: parse_immediate_signed(address)
                            .unwrap_or_else(|e| panic!("{}", e)),
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
            "bne" => match parts[1..] {
                [rt, rs, address] => {
                    Ok(Instruction::IExceptional {
                        opcode: 0b000101, // Opcode is 5
                        value: parse_register(rs).unwrap_or_else(|e| panic!("{}", e)),
                        rt: parse_register(rt).unwrap_or_else(|e| panic!("{}", e)),
                        immediate: parse_immediate_signed(address)
                            .unwrap_or_else(|e| panic!("{}", e)),
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
            "j" => match parts[1..] {
                [address] => {
                    Ok(Instruction::J {
                        opcode: 0b000010, // Opcode is 2
                        address: parse_address(address).unwrap_or_else(|e| panic!("{}", e)),
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
            "jal" => match parts[1..] {
                [address] => {
                    Ok(Instruction::J {
                        opcode: 0b000011, // Opcode is 3
                        address: parse_address(address).unwrap_or_else(|e| panic!("{}", e)),
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
            "lb" => match parts[1..] {
                [rt, immediate, rs] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));
                    let rs = parse_register(rs).unwrap_or_else(|e| panic!("{}", e));
                    let immediate =
                        parse_immediate_signed(immediate).unwrap_or_else(|e| panic!("{}", e));
                    Ok(Instruction::ISigned {
                        opcode: 0b100000, // Opcode is 32
                        rs,
                        rt,
                        immediate,
                    })
                }
                [rt, relative_value] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));

                    let relative_value_parts = relative_value.split("(").collect::<Vec<_>>();

                    let immediate = relative_value_parts.get(0).unwrap_or_else(|| {
                        panic!(
                            "Missing immediate in relative value parts {:?}",
                            relative_value_parts
                        )
                    });
                    let immediate =
                        parse_immediate_signed(immediate).unwrap_or_else(|e| panic!("{}", e));

                    let rs = relative_value_parts.get(1).unwrap_or_else(|| {
                        panic!(
                            "Missing rs in relative value parts {:?}",
                            relative_value_parts
                        )
                    });
                    let rs = rs.replace(")", "");
                    let rs = parse_register(&rs).unwrap_or_else(|e| panic!("{}", e));

                    Ok(Instruction::ISigned {
                        opcode: 0b100000, // Opcode is 32
                        rs,
                        rt,
                        immediate,
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
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
                [rt, relative_value] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));

                    let relative_value_parts = relative_value.split("(").collect::<Vec<_>>();

                    let immediate = relative_value_parts.get(0).unwrap_or_else(|| {
                        panic!(
                            "Missing immediate in relative value parts {:?}",
                            relative_value_parts
                        )
                    });
                    let immediate =
                        parse_immediate_signed(immediate).unwrap_or_else(|e| panic!("{}", e));

                    let rs = relative_value_parts.get(1).unwrap_or_else(|| {
                        panic!(
                            "Missing rs in relative value parts {:?}",
                            relative_value_parts
                        )
                    });
                    let rs = rs.replace(")", "");
                    let rs = parse_register(&rs).unwrap_or_else(|e| panic!("{}", e));

                    Ok(Instruction::ISigned {
                        opcode: 0b100011, // Opcode is 35
                        rs,
                        rt,
                        immediate,
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
            "nop" => Ok(Instruction::Nop),
            "nor" => define_r_instruction_parse!(parts, 0b100111), // Funct is 39
            "or" => define_r_instruction_parse!(parts, 0b100101),  // Funct is 37
            "sb" => match parts[1..] {
                [rt, immediate, rs] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));
                    let rs = parse_register(rs).unwrap_or_else(|e| panic!("{}", e));
                    let immediate =
                        parse_immediate_signed(immediate).unwrap_or_else(|e| panic!("{}", e));
                    Ok(Instruction::ISigned {
                        opcode: 0b101000, // Opcode is 40
                        rs,
                        rt,
                        immediate,
                    })
                }
                [rt, relative_value] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));

                    let relative_value_parts = relative_value.split("(").collect::<Vec<_>>();

                    let immediate = relative_value_parts.get(0).unwrap_or_else(|| {
                        panic!(
                            "Missing immediate in relative value parts {:?}",
                            relative_value_parts
                        )
                    });
                    let immediate =
                        parse_immediate_signed(immediate).unwrap_or_else(|e| panic!("{}", e));

                    let rs = relative_value_parts.get(1).unwrap_or_else(|| {
                        panic!(
                            "Missing rs in relative value parts {:?}",
                            relative_value_parts
                        )
                    });
                    let rs = rs.replace(")", "");
                    let rs = parse_register(&rs).unwrap_or_else(|e| panic!("{}", e));

                    Ok(Instruction::ISigned {
                        opcode: 0b101000, // Opcode is 40
                        rs,
                        rt,
                        immediate,
                    })
                }
                _ => panic!("Unknown structure for instruction \"{}\"", parts[0]),
            },
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
                [rt, relative_value] => {
                    let rt = parse_register(rt).unwrap_or_else(|e| panic!("{}", e));

                    let relative_value_parts = relative_value.split("(").collect::<Vec<_>>();

                    let immediate = relative_value_parts.get(0).unwrap_or_else(|| {
                        panic!(
                            "Missing immediate in relative value parts {:?}",
                            relative_value_parts
                        )
                    });
                    let immediate =
                        parse_immediate_signed(immediate).unwrap_or_else(|e| panic!("{}", e));

                    let rs = relative_value_parts.get(1).unwrap_or_else(|| {
                        panic!(
                            "Missing rs in relative value parts {:?}",
                            relative_value_parts
                        )
                    });
                    let rs = rs.replace(")", "");
                    let rs = parse_register(&rs).unwrap_or_else(|e| panic!("{}", e));

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
            } => {
                let rt = REGISTERS[*rt as usize];
                let value = REGISTERS[*value as usize];

                match opcode {
                    0b000100 => format!("beq {}, {}, {}", value, rt, immediate), // Opcode is 4
                    0b000101 => format!("bne {}, {}, {}", value, rt, immediate), // Opcode is 5
                    _ => panic!("Unknown opcode \"{}\" for IExceptional instruction", opcode),
                }
            }
            Instruction::ISigned {
                opcode,
                rs,
                rt,
                immediate,
            } => {
                let rt = REGISTERS[*rt as usize];
                let rs = REGISTERS[*rs as usize];

                match opcode {
                    0b001000 => format!("addi {}, {}, {}", rt, rs, immediate), // Opcode is 8
                    0b001100 => format!("andi {}, {}, {}", rt, rs, immediate), // Opcode is 12
                    0b100000 => format!("lb {}, {}({})", rt, immediate, rs),   // Opcode is 32
                    0b001111 => format!("lui {}, {}", rt, immediate),          // Opcode is 15
                    0b100011 => format!("lw {}, {}({})", rt, immediate, rs),   // Opcode is 35
                    0b101000 => format!("sb {}, {}({})", rt, immediate, rs),   // Opcode is 40
                    0b001010 => format!("slti {}, {}, {}", rt, rs, immediate), // Opcode is 10
                    0b101011 => format!("sw {}, {}({})", rt, immediate, rs),   // Opcode is 43
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
            Instruction::J { opcode, address } => match opcode {
                0b000010 => format!("j {}", address),
                0b000011 => format!("jal {}", address),
                _ => panic!("Unknown opcode \"{}\" for J instruction", opcode),
            },
            Instruction::Nop => String::from("nop"),
            Instruction::R {
                rs, rt, rd, funct, ..
            } => {
                let rd = REGISTERS[*rd as usize];
                let rs = REGISTERS[*rs as usize];
                let rt = REGISTERS[*rt as usize];

                match funct {
                    0b100000 => format!("add {}, {}, {}", rd, rs, rt), // Funct is 32
                    0b100001 => format!("addu {}, {}, {}", rd, rs, rt), // Funct is 33
                    0b100100 => format!("and {}, {}, {}", rd, rs, rt), // Funct is 36
                    0b100111 => format!("nor {}, {}, {}", rd, rs, rt), // Funct is 39
                    0b100101 => format!("or {}, {}, {}", rd, rs, rt),  // Funct is 37
                    0b101010 => format!("slt {}, {}, {}", rd, rs, rt), // Funct is 42
                    0b101011 => format!("sltu {}, {}, {}", rd, rs, rt), // Funct is 43
                    0b100010 => format!("sub {}, {}, {}", rd, rs, rt), // Funct is 34
                    0b100011 => format!("subu {}, {}, {}", rd, rs, rt), // Funct is 35
                    _ => panic!(
                        "Unknown funct \"{:b}\" ({}) for instruction {:?}",
                        funct, funct, self
                    ),
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
            Instruction::J { opcode, address } => {
                let mut machine_code = 0u32;
                machine_code |= (*opcode as u32) << 26;
                machine_code |= *address as u32;
                machine_code
            }
            Instruction::Nop => 0u32,
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

fn parse_address(content: &str) -> Result<u32, String> {
    if content.starts_with("0x") || content.starts_with("0X") {
        u32::from_str_radix(&content[2..], 16)
            .map_err(|e| format!("Could not parse address \"{}\": {}", content, e.to_string()))
    } else if content.starts_with("-") {
        Err(format!(
            "Could not parse address \"{}\": {}",
            content, "Negative number given when only unsigned immediate is expected"
        ))
    } else {
        content
            .parse::<u32>()
            .map_err(|e| format!("Could not parse address \"{}\": {}", content, e.to_string()))
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
        let line = line.split("#").collect::<Vec<&str>>();
        let line = line[0].trim();

        // If the line is empty, skip the line.
        if line.is_empty() {
            continue;
        }
        // If the line contains an assignment
        else if line.starts_with("let") {
            let assignment_parts = line[3..].split("=").collect::<Vec<&str>>();

            if assignment_parts.len() != 2 {
                return Err(format!(
                    "Could not parse assignment \"{}\": Invalid assignment format",
                    line
                ));
            }

            let variable_name = assignment_parts[0].trim().to_string();
            let variable_value = assignment_parts[1].trim();

            // If the variable value is a string
            if variable_value.starts_with("\"") && variable_value.ends_with("\"") {
                let variable_value = variable_value[1..variable_value.len() - 1].to_string();
                nodes.push(Node {
                    kind: NodeKind::Assignment(variable_name, variable_value),
                });
            }
            // If the variable value is something else
            else {
                return Err(format!(
                    "Could not parse assignment \"{}\": Invalid value format. Only strings marked with double quotes are supported.",
                    line
                ));
            }
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
    fn disassemble_beq_instruction_from_bytes() {
        // beq v0, zero 0x02
        let instruction = Instruction::parse_from_le_bytes(&[0x02, 0x00, 0x40, 0x10]);
        assert_eq!(instruction.to_instruction(), "beq v0, zero, 2");
    }

    #[test]
    fn parse_bne_instruction_from_string() {
        // bne t0, t1, 0x20
        let instruction = Instruction::parse_from_str("bne t0, t1, 0x20").unwrap();
        let result_bin = 0b00010101000010010000000000100000;
        let result_hex = 0x15090020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);

        // bne v0, zero, 0x4
        let instruction = Instruction::parse_from_str("bne v0, zero, 0x4").unwrap();
        let result_bin = 0b00010100010000000000000000000100;
        let result_hex = 0x14400004;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }

    #[test]
    fn disassemble_jal_instruction_from_bytes() {
        // jal 0x14870
        let result_bin = 0b1100000000010100100001110000;
        let result_hex = 0xc014870;
        assert_eq!(result_bin, result_hex);

        let instruction = Instruction::parse_from_le_bytes(&[0x70, 0x48, 0x01, 0x0c]);
        assert_eq!(instruction.to_machine_code(), result_bin);
        assert_eq!(instruction.to_instruction(), "jal 84080"); // 84080 = 0x14870
    }
    #[test]
    fn disassemble_jal_instruction_from_machine_code() {
        // jal 0x32
        let instruction = Instruction::parse_from_machine_code(0x0C000032);
        assert_eq!(instruction.to_instruction(), "jal 50"); // 50 = 0x32
    }
    #[test]
    fn parse_jal_instruction_from_string() {
        // jal 0x32
        let instruction = Instruction::parse_from_str("jal 0x32").unwrap();
        let result_bin = 0b00001100000000000000000000110010;
        let result_hex = 0x0C000032;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }

    #[test]
    fn disassemble_lb_instruction_from_bytes() {
        // lb t0, 0x20(t1)
        let result_bin = 0b10000001001010000000000000100000;
        let result_hex = 0x81280020;
        assert_eq!(result_bin, result_hex);

        let instruction = Instruction::parse_from_le_bytes(&[0x20, 0x00, 0x28, 0x81]);
        assert_eq!(instruction.to_machine_code(), result_bin);
        assert_eq!(instruction.to_instruction(), "lb t0, 32(t1)");
    }
    #[test]
    fn parse_lb_instruction_from_string() {
        // lb t0, 0x20(t1)
        let instruction = Instruction::parse_from_str("lb t0, 0x20(t1)").unwrap();
        let result_bin = 0b10000001001010000000000000100000;
        let result_hex = 0x81280020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);
    }

    #[test]
    fn disassemble_slti_instruction_from_bytes() {
        // slti t0, t1, 0x20
        let result_bin = 0b00101001001010000000000000100000;
        let result_hex = 0x29280020;
        assert_eq!(result_bin, result_hex);

        let instruction = Instruction::parse_from_le_bytes(&[0x20, 0x00, 0x28, 0x29]);
        assert_eq!(instruction.to_machine_code(), result_bin);
        assert_eq!(instruction.to_instruction(), "slti t0, t1, 32"); // 32 = 0x20

        // slti v0, v0, 0x5
        let result_bin = 0b00101000010000100000000000000101;
        let result_hex = 0x28420005;
        assert_eq!(result_bin, result_hex);

        let instruction = Instruction::parse_from_le_bytes(&[0x05, 0x00, 0x42, 0x28]);
        assert_eq!(instruction.to_machine_code(), result_bin);
        assert_eq!(instruction.to_instruction(), "slti v0, v0, 5"); // 5 = 0x5
    }
    #[test]
    fn parse_slti_instruction_from_machine_code() {
        // slti t0, t1, 0x20
        let instruction = Instruction::parse_from_machine_code(0x29280020);
        let result_bin = 0b00101001001010000000000000100000;
        let result_hex = 0x29280020;
        assert_eq!(result_bin, result_hex);
        assert_eq!(instruction.to_machine_code(), result_bin);

        // slti v0, v0, 0x5
        let instruction = Instruction::parse_from_machine_code(0x28420005);
        let result_bin = 0b00101000010000100000000000000101;
        let result_hex = 0x28420005;
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
    Assignment(String, String),
    CustomCommand(CustomCommand),
    Instruction(Instruction),
    Label(String),
}
