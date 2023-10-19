use std::collections::HashMap;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Read;

use mips::{parse_nodes, CustomCommand, NodeKind};
use ps1exe::{PS1Exe, PS1ExeReader, PS1ExeWriteResult, PS1ExeWriter};
use rom_manager::CDROMXAVolume;
use wad::{WADReader, WAD};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("------------------");
    println!("    OPEN SPYRO");
    println!("------------------");

    let args = env::args().collect::<Vec<String>>();

    let command = args
        .get(1)
        .ok_or_else(|| "No command provided as a command line argument.")?;

    let command_args = &args[2..];

    for (command_name, _, command_function) in COMMANDS.iter() {
        if command == *command_name {
            // Requested command found, execute it.
            return command_function(command_args);
        }
    }

    Err(format!(
        "No supported command or enough arguments for it provided (command \"{}\" given).",
        command
    )
    .into())
}

macro_rules! get_arg {
    ($args: ident, $arg_num: literal, $arg_name: literal) => {
        $args.get($arg_num).ok_or_else(|| {
            format!(
                concat!(
                    "No ",
                    $arg_name,
                    " provided as a command line argument #{}."
                ),
                $arg_num + 2
            )
        })
    };
}

const COMMANDS: &[(
    &str,
    &str,
    fn(args: &[String]) -> Result<(), Box<dyn std::error::Error>>,
)] = &[
    ("generate-doc", "Generates README.md file describing the project at project root.", generate_doc),
    ("mips-assemble", "Converts MIPS assembly instruction into machine code (as hexadecimal) and into LE bytes also.", mips_assemble),
    ("mips-disassemble", "Converts machine code into an MIPS assembly instruction string.", mips_disassemble),
    ("ps1exe-assemble", "Assembles MIPS assembly code from a given text file into a Playstation executable.", ps1exe_assemble),
    ("ps1exe-disassemble", "Disassembles a section of MIPS assembly code from a given Playstation executable binary.", ps1exe_disassemble),
    ("rom-check", "Checks the given ROM file structure for correctness.", rom_check),
    ("rom-extract", "Extracts a file from a ROM to a given extract path.", rom_extract),
    ("rom-replace", "Replaces a file in a given ROM with a given input file.", rom_replace),
    ("wad-read", "Reads information about WAD file. Heavily WIP.", wad_read),
];

fn generate_doc(_args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    #[inline]
    fn code(content: &str) -> String {
        format!("`{}`", content)
    }
    fn list_bulleted(items: &[&str]) -> String {
        items
            .iter()
            .map(|item| format!("* {}", item))
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn title(level: usize, content: &str) -> String {
        format!("{} {}", "#".repeat(level), content)
    }

    // Initialize all the elements of the README.md file.
    let mut commands = COMMANDS
        .iter()
        .map(|(name, description, _)| format!("{} {}", code(name), description))
        .collect::<Vec<_>>();
    // Ensure all commands are sorted alphabetically.
    commands.sort();
    let commands = commands.iter().map(|command| command.as_str()).collect::<Vec<_>>();
    let commands = &commands[..];

    let elements = [
        title(1, "Open Spyro"),
        String::from("![Spyro the Dragon screenshot from Town Square](http://henrijahanna.fi/projects/open_spyro/spyro_town_square.bmp)"),
        
        title(2, "Introduction"),
        String::from("Welcome to the Spyro the Dragon Linux and Windows port project called OpenSpyro! OpenSpyro is an open-source initiative driven by a passionate programmer and fan of the original Spyro the Dragon trilogy on Playstation. The project is focused on reverse-engineering and adapting the game to run natively on Linux and Windows (I don't have a Mac, sorry). I aim to provide an authentic experience that stays true to the original while leveraging the capabilities of modern hardware."),
        String::from("This project will not contain any game data in its original form. You must have an original ROM of the game available in order to play this port."),

        title(2, "Goals"),
        list_bulleted(&[
            "Make the game run natively, no emulation needed to run the game on Linux or Windows.",
            "Maintaining the original experience as is while leveraging the capabilities of modern hardware. The focus is on rewriting only the necessary logic to get the game running on Linux and Windows, although some quality of life improvements may be made.",
            "Support for mods in some way."
        ]),

        title(2, "Current state"),
        String::from("The project is currently in its early stages. Port is currently unplayable. Some of the MIPS assembly code has been disassembled with comments and labels added. Disassembly is in a completely custom assembly, the repository even has its own assembler and disassembler! This custom assembler and disassembler ought to have more documentation, but it is still in the works."),
        String::from("The project is currently focused on reverse-engineering the MIPS assembly and understanding how the data is layed out on the ROM. I am still learning MIPS assembly works, how assembly works in general (now having trouble understanding how to decode/encode j instructions and how stack works for storing local variables). Some tools have been built along the way, like a working ROM file replacer, which is used for saving Playstation executable files back into the ROM file. Some work has been started on reading WAD files, but it is in its early stages."),
        String::from("While learning MIPS assembly and thinking about decompilation, I decided to start working on a custom programming language for aiding reverse engineering seriously. I could use C, but I was having trouble linking stuff and I didn't want to bother with linker scripts and stuff. I have already built tokenization, basic parsing for functions, variable initialization and support for some built-in return types and values. Assembly to MIPS and compilation to MIPS processor compatible machine code still WIP. This custom compiler will be added to this repository as a separate crate once there's enough significant progress made. `cmips` crate is a separate attempt, it will be replaced eventually."),
        String::from("ROM currently worked on is an NTSC version. I have not worked on any other versions yet."),

        title(2, "Commands"),
        String::from("Here's a list CLI commands currently supported:"),
        list_bulleted(commands),

        title(2, "Disclaimer"),
        String::from("OpenSpyro is an independent project and is not affiliated with the original creators or owners of Spyro the Dragon. It is a fan-driven initiative for educational and entertainment purposes. I do not claim ownership of the original game's assets or intellectual property."),
        String::from("I am still learning about reverse engineering, how the Playstation works and how porting with all its intricacies when its comes to like audio and graphics should be handled. I am not an expert. Suggestions and help is welcome and appreciated.")
    ];

    // Write the elements into the README.md file.
    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();

    let output_file_name = "README.md";
    let output_file_path = format!("{}/{}", current_dir, output_file_name);
    let output_file_content = elements.join("\n\n");

    fs::write(output_file_path, output_file_content)
        .map_err(|err| format!("Failed to write {}: {}", output_file_name, err))?;

    println!("README.md file generated successfully!");
    Ok(())
}
/// Converts MIPS assembly instruction into machine code (as hexadecimal) and into LE bytes also.
fn mips_assemble(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let value = get_arg!(args, 0, "value")?;

    let instruction = mips::Instruction::parse_from_str(&value)?;
    println!("{:x}", instruction.to_machine_code());

    let instruction_bytes = instruction.to_le_bytes();
    let instruction_bytes = instruction_bytes
        .iter()
        .map(|v| format!("{:x}", v))
        .collect::<Vec<_>>();
    println!("{:?}", instruction_bytes);
    Ok(())
}
/// Converts machine code into an MIPS assembly instruction string.
fn mips_disassemble(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let value = get_arg!(args, 0, "value")?;

    let value_split = value.split(" ").collect::<Vec<&str>>();

    match value_split.len() {
        4 => {
            let mut bytes = [0u8; 4];
            for i in 0..bytes.len() {
                let parsed_byte = u8::from_str_radix(value_split[i], 16).map_err(|_| {
                    format!(
                        "Failed to parse given byte \"{}\" at index {} as a hexadecimal number.",
                        value_split[i], i
                    )
                })?;
                bytes[i] = parsed_byte;
            }
            let value = u32::from_le_bytes(bytes);
            println!("{:x}", value);
            println!("0x{:x}", value);
            Ok(())
        }
        1 => {
            let value = u32::from_str_radix(value, 16).map_err(|_| {
                format!(
                    "Failed to parse given value \"{}\" as a hexadecimal number.",
                    value
                )
            })?;

            let instruction = mips::Instruction::parse_from_machine_code(value);

            println!("{}", instruction.to_instruction());
            Ok(())
        }
        _ => Err(format!(
            "Failed to parse given value \"{}\" as a hexadecimal number.",
            value
        )
        .into()),
    }
}
/// Assembles MIPS assembly code from a given text file into a Playstation executable.
fn ps1exe_assemble(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let input_assembly_code_file_path = get_arg!(args, 0, "input assembly code file path")?;
    let input_ps1_exe_file_path = get_arg!(args, 1, "input PS1 EXE file path")?;
    let output_ps1_exe_file_path = get_arg!(args, 2, "output PS1 EXE file path")?;

    let mut input_file = File::open(input_assembly_code_file_path).map_err(|_| {
        format!(
            "Failed to open given input MIPS assembly code file in path \"{}\".",
            input_assembly_code_file_path
        )
    })?;

    let mut input_file_content = String::new();
    input_file
        .read_to_string(&mut input_file_content)
        .map_err(|_| {
            format!(
                "Failed to read given input MIPS assembly code file in path \"{}\".",
                input_assembly_code_file_path
            )
        })?;

    let nodes = parse_nodes(&input_file_content).map_err(|e| {
        format!(
            "Failed to parse given input MIPS assembly code file in path \"{}\": {}",
            input_assembly_code_file_path,
            e.to_string()
        )
    })?;

    println!("Input PS1 EXE file path: {}", input_ps1_exe_file_path);
    let mut ps1_exe = PS1Exe::from_file_path(input_ps1_exe_file_path)?;

    // Print Playstation executable header information
    println!(
        "PS1 EXE destination address in RAM: 0x{:X}",
        ps1_exe.destination_address_in_ram
    );
    println!("PS1 EXE file size: {}", ps1_exe.file_size);
    println!("PS1 EXE initial GP R28: 0x{:X}", ps1_exe.initial_gp_r28);
    println!("PS1 EXE initial PC value: 0x{:X}", ps1_exe.initial_pc);
    println!("PS1 EXE ASCII marker: {}", ps1_exe.ascii_marker);

    enum UnfinishedOperation{
        Addr{
            address: u64,
            name: String,
        }
    }

    let mut constants = HashMap::new();
    let mut current_address = 0;
    let mut ps1_exe_writer = PS1ExeWriter::new(&mut ps1_exe);
    let mut unfinished_operations = Vec::new();

    use colored::*;

    for node in nodes.iter() {
        match &node.kind {
            NodeKind::Addr(name) => {
                current_address += 4;

                if let Some(matching_address) = constants.get(name) {
                    let bytes = (*matching_address as u32).to_le_bytes();
                    if let PS1ExeWriteResult::Changed { original_code } =
                        ps1_exe_writer.write_code(current_address, &bytes)
                    {
                        println!(
                            "{}",
                            format!(
                                "Line {}: Addr to {} - changed bytes to {:?} from {:?}",
                                node.line,
                                name,
                                bytes,
                                original_code
                            )
                            .red()
                        );
                    }
                }
                else {
                    unfinished_operations.push(UnfinishedOperation::Addr{
                        address: current_address,
                        name: name.clone(),
                    });
                }
            }
            NodeKind::CustomCommand(command) => match command {
                CustomCommand::At(address) => {
                    current_address = *address - 4;
                }
            },
            NodeKind::Instruction(instruction) => {
                current_address += 4;

                if let PS1ExeWriteResult::Changed { original_code } =
                    ps1_exe_writer.write_code(current_address, &instruction.to_le_bytes())
                {
                    println!(
                        "{}",
                        format!(
                            "Line {}: {} - changed bytes to {:?} from {:?}",
                            node.line,
                            instruction.to_instruction(),
                            instruction.to_le_bytes(),
                            original_code
                        )
                        .red()
                    );
                }
            }
            NodeKind::IntegerAssignment(variable_name, value) => {
                current_address += 4;

                constants.insert(variable_name, node.address);

                if let PS1ExeWriteResult::Changed { original_code } =
                    ps1_exe_writer.write_code(current_address, &value.to_le_bytes())
                {
                    println!(
                        "{}",
                        format!(
                            "Line {}: Assignment {} = {} - changed bytes to {:?} from {:?}",
                            node.line,
                            variable_name,
                            value,
                            value.to_le_bytes(),
                            original_code
                        )
                        .red()
                    );
                }
            }
            NodeKind::StringAssignment(variable_name, value) => {
                current_address += 4;

                constants.insert(variable_name, node.address);

                let value_bytes = value.as_bytes();
                let mut new_value_bytes = Vec::<u8>::with_capacity(value_bytes.len() + 1);

                let mut i = 0;
                while i < value_bytes.len() {
                    let b = value_bytes[i];
                    // If byte is a backslash (\)
                    if b == 92 && (i + 1) < value_bytes.len() {
                        i += 1;
                        // If byte is 0 digit (null termination byte)
                        if value_bytes[i] == 48 {
                            i += 1;
                            new_value_bytes.push(0);
                        }
                        else{
                            return Err(format!(
                                "Failed to parse given string \"{}\". Invalid escape sequence \"\\{}\" at index {}.",
                                value, value_bytes[i], i
                            ).into());
                        }
                    }
                    else{
                        new_value_bytes.push(b);
                    }
                    i += 1;
                }

                if let PS1ExeWriteResult::Changed { original_code } =
                    ps1_exe_writer.write_code(current_address, &new_value_bytes[..])
                {
                    println!(
                        "{}",
                        format!(
                            "Line {}: Assignment {} = {} - changed bytes to {:?} from {:?}",
                            node.line,
                            variable_name,
                            value,
                            value.as_bytes(),
                            original_code
                        )
                        .red()
                    );
                }
            }
            _ => {}
        }
    }

    for operation in unfinished_operations.iter() {
        match operation {
            UnfinishedOperation::Addr { address, name } => {
                if let Some(matching_address) = constants.get(name) {
                    let bytes = (*matching_address as u32).to_le_bytes();
                    if let PS1ExeWriteResult::Changed { original_code } =
                        ps1_exe_writer.write_code(*address, &bytes)
                    {
                        println!(
                            "{}",
                            format!(
                                "Addr to {} - changed bytes to {:?} from {:?}",
                                name, bytes, original_code
                            )
                            .red()
                        );
                    }
                }
                else {
                    return Err(format!(
                        "Failed to find constant \"{}\" at address {}.",
                        name, address
                    )
                    .into());
                }
            }
        }
    }

    ps1_exe_writer.write_into_file(output_ps1_exe_file_path)?;

    println!(
        "Bytes written into binary: {:.2}%",
        ps1_exe_writer.get_percentage_of_written_bytes() * 100f32
    );
    println!(
        "Output PS1 EXE file written to \"{}\".",
        output_ps1_exe_file_path
    );
    println!("Done!");
    Ok(())
}
/// Disassembles a section of MIPS assembly code from a given Playstation executable binary.
fn ps1exe_disassemble(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() > 3 {
        // Disassemble MIPS assembly code from one given address (as hexadecimal) memory until another given address
        let input_ps1_exe_file_path = get_arg!(args, 0, "input PS1 EXE file path")?;

        let start_address_in_memory = get_arg!(args, 1, "start address in memory")?;
        let start_address_in_memory =
            u64::from_str_radix(start_address_in_memory, 16).map_err(|_| {
                format!(
                    "Failed to parse given start address in memory \"{}\" as a hexadecimal number.",
                    start_address_in_memory
                )
            })?;

        let option = get_arg!(args, 2, "until option")?;

        if option == "--until" {
            let end_address_in_memory = get_arg!(args, 3, "end address in memory")?;
                let end_address_in_memory =
                    u64::from_str_radix(end_address_in_memory, 16).map_err(|_| {
                        format!(
                            "Failed to parse given end address in memory \"{}\" as a hexadecimal number.",
                            end_address_in_memory
                        )
                    })?;

                let ps1_exe = PS1Exe::from_file_path(&input_ps1_exe_file_path)?;

                if start_address_in_memory > end_address_in_memory {
                    return Err(format!(
                        "Start address in memory \"{}\" is greater than end address in memory \"{}\". Start address in memory should be less than end address in memory.",
                        start_address_in_memory, end_address_in_memory
                    ).into());
                }

                let ps1_exe_reader = PS1ExeReader::new(&ps1_exe);
                let instruction_count = (end_address_in_memory - start_address_in_memory) as usize / 4; // 4 bytes per instruction
                ps1_exe_reader.disassemble_at_adress_by_count(start_address_in_memory, instruction_count);
        }
        else{
            return Err(format!(
                "Invalid option given after the start address \"{}\". Valid until option is \"until\".",
                option
            )
            .into());
        }
    } else {
        let input_ps1_exe_file_path = get_arg!(args, 0, "input PS1 EXE file path")?;

        let start_address_in_memory = get_arg!(args, 1, "address in memory")?;
        let start_address_in_memory = u64::from_str_radix(start_address_in_memory, 16).map_err(|_| {
            format!(
                "Failed to parse given address in memory \"{}\" as a hexadecimal number.",
                start_address_in_memory
            )
        })?;

        let instruction_count_or_option = get_arg!(args, 2, "instruction count")?;

        if instruction_count_or_option == "--string" {
            let ps1_exe = PS1Exe::from_file_path(&input_ps1_exe_file_path)?;

            let ps1_exe_reader = PS1ExeReader::new(&ps1_exe);
            let end_byte = 0x00; // Null termination byte
            let value = ps1_exe_reader.disassemble_str_at_address_until_byte(start_address_in_memory, end_byte).map_err(|err| {
                format!(
                    "Failed to disassemble string at address \"{}\" until byte \"{}\": {}",
                    start_address_in_memory, end_byte, err.to_string()
                )
            })?;
            println!("@at 0x{:x}\n{} TEMP = \"{}\"", start_address_in_memory, mips::KEYWORD_CONST, value);
        }
        else {
            // Disassemble MIPS assembly code from given address (as hexadecimal) memory onwards
            let instruction_count = instruction_count_or_option.parse::<usize>().map_err(|_| {
                format!(
                    "Failed to parse given instruction count \"{}\" as a number.",
                    instruction_count_or_option
                )
            })?;
    
            let ps1_exe = PS1Exe::from_file_path(&input_ps1_exe_file_path)?;
    
            let ps1_exe_reader = PS1ExeReader::new(&ps1_exe);
            ps1_exe_reader.disassemble_at_adress_by_count(start_address_in_memory, instruction_count);
        }
    }

    Ok(())
}
/// Checks the given ROM file structure for correctness.
fn rom_check(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = get_arg!(args, 0, "ROM path")?;

    // Initialize the volume based on given ROM file path.
    let volume_file = File::open(rom_path)
        .map_err(|_| format!("Failed to open given ROM file in path \"{}\".", rom_path))?;
    let mut volume = CDROMXAVolume::new(volume_file);

    // Read the volume descriptor locations from the volume.
    let vd_locations = volume.read_volume_descriptor_locations().map_err(|e| {
        format!(
            "ROM file given has invalid data: failed to read volume descriptor locations: {}",
            e.to_string()
        )
    })?;

    let pvd = volume
        .read_primary_volume_descriptor(&vd_locations)
        .map_err(|e| {
            format!(
                "ROM file given has invalid data: failed to read primary volume descriptor: {}",
                e.to_string()
            )
        })?;

    // Read the root directory record from the volume and  ensure it has all the expected values.
    let root_record = &pvd.directory_record_for_root_directory;

    macro_rules! root_record_assert_eq {
        ($data_name:literal, $left:expr, $right:expr) => {
            if ($left != $right) {
                return Err(format!("ROM file given has invalid data: root record {} is incorrect (expected {}, got {}).",
                $data_name, $left, $right).into());
            }
        };
    }

    root_record_assert_eq!("length", root_record.length, 34);
    root_record_assert_eq!("location of extent", root_record.location_of_extent, 22);
    root_record_assert_eq!("data length", root_record.data_length, 2048);
    root_record_assert_eq!(
        "recording date and time",
        root_record.recording_date_and_time_formatted(),
        "1998-08-13 16:56:05"
    );
    assert_eq!(root_record.is_dir(), true);
    root_record_assert_eq!(
        "file identifier length",
        root_record.file_identifier_length,
        1
    );

    // Read the sub-records, that is, directories and files by the root directory record
    // and ensure the expected directories and files are present.
    let sub_records = volume
        .read_directory_records(
            &pvd.directory_record_for_root_directory,
            pvd.logical_block_size,
        )
        .map_err(|err| {
            format!(
                "ROM file given has invalid data: failed to read sub-records by root directory: {}",
                err.to_string()
            )
        })?;

    // An array of tuples containing the expected sub-record name and whether it is a directory.
    const TOP_LEVEL_ENTRIES: &[(&'static str, bool)] = &[
        ("SOURCE", true),
        ("PETEXA0.STR", false),
        ("PETEXA1.STR", false),
        ("PETEXA2.STR", false),
        ("PETEXA3.STR", false),
        ("PETEXA4.STR", false),
        ("PETEXA5.STR", false),
        ("S0", true),
        ("SCUS_942.28", false),
        ("SYSTEM.CNF", false),
        ("WAD.WAD", false),
    ];

    if sub_records.len() < TOP_LEVEL_ENTRIES.len() {
        return Err(format!(
            "ROM file given has invalid data: root record should have at least {} entries.",
            TOP_LEVEL_ENTRIES.len()
        )
        .into());
    }

    // Ensure the expected sub-records are present and are of the expected type.
    for (entry, is_dir) in TOP_LEVEL_ENTRIES.iter() {
        let sub_record_find_result = sub_records
            .iter()
            .find(|r| r.file_identifier_as_string() == *entry);

        if let Some(sub_record) = sub_record_find_result {
            let sub_record_is_dir = sub_record.is_dir();
            if sub_record_is_dir == *is_dir {
                continue;
            } else {
                let expected_type = if *is_dir { "directory" } else { "file" };
                let got_type = if sub_record_is_dir {
                    "directory"
                } else {
                    "file"
                };

                return Err(format!(
                    "ROM file given does not has the expected sub-record \"{}\", but it is of incorrect type (expected {}, got {}).",
                    entry, expected_type, got_type
                ).into());
            }
        } else {
            return Err(format!(
                "ROM file given does not contain expected sub-record \"{}\".",
                entry
            )
            .into());
        }
    }

    // Read the sub-records of the SOURCE directory and ensure the expected directories and files are present.
    let source_dir_records = volume
        .read_directory_records(&sub_records[02], pvd.logical_block_size)
        .map_err(|err| format!("ROM file given has invalid data: failed to read sub-records in SOURCE directory: {}", err.to_string()))?;

    let source_dir_entries_num = 3;
    if source_dir_records.len() != source_dir_entries_num {
        return Err(format!(
            "ROM file given has invalid data: SOURCE directory should have {} entries.",
            source_dir_entries_num
        )
        .into());
    }

    if source_dir_records[02].file_identifier_as_string() != "SOURCE.TRD" {
        return Err(format!(
            "ROM file given has invalid data: SOURCE.TRD in SOURCE directory is missing."
        )
        .into());
    }
    if source_dir_records[02].is_dir() {
        return Err(format!(
            "ROM file given has invalid data: SOURCE.TRD in SOURCE directory should be a file, not a directory."
        )
        .into());
    }

    println!("ROM path: \"{}\"", rom_path);
    println!("ROM validity checks passed. ROM includes valid data.");
    Ok(())
}
/// Extracts a file from a ROM to a given extract path.
fn rom_extract(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = get_arg!(args, 0, "ROM path")?;
    let entry_input_path = get_arg!(args, 1, "entry input path")?;
    let entry_extract_path = get_arg!(args, 2, "entry extract path")?;

    // Initialize the volume based on given ROM file path.
    let volume_file = File::open(rom_path)
        .map_err(|_| format!("Failed to open given ROM file in path \"{}\".", rom_path))?;
    let mut volume = CDROMXAVolume::new(volume_file);

    // Read the volume descriptor locations from the volume.
    let vd_locations = volume.read_volume_descriptor_locations().map_err(|e| {
        format!(
            "ROM file given has invalid data: failed to read volume descriptor locations: {}",
            e.to_string()
        )
    })?;

    let pvd = volume
        .read_primary_volume_descriptor(&vd_locations)
        .map_err(|e| {
            format!(
                "ROM file given has invalid data: failed to read primary volume descriptor: {}",
                e.to_string()
            )
        })?;

    // Read the sub-records, that is, directories and files by the root directory record
    let sub_records = volume
        .read_directory_records(
            &pvd.directory_record_for_root_directory,
            pvd.logical_block_size,
        )
        .map_err(|err| {
            format!(
                "ROM file given has invalid data: failed to read sub-records by root directory: {}",
                err.to_string()
            )
        })?;

    // Find the entry in the root directory record that matches the given entry path.
    let entry_record = sub_records
        .iter()
        .find(|r| r.file_identifier_as_string() == *entry_input_path)
        .ok_or_else(|| {
            format!(
                "ROM file given does not contain the given entry path \"{}\".",
                entry_input_path
            )
        })?;

    // Read the entry record data and write it to the given extract path.
    let entry_record_data =
        volume.read_directory_record_data(entry_record, pvd.logical_block_size)?;

    fs::write(entry_extract_path, entry_record_data).map_err(|err| {
        format!(
            "Failed to write extracted file from ROM to path \"{}\": {}",
            entry_extract_path,
            err.to_string()
        )
    })?;

    println!("ROM path: \"{}\"", rom_path);
    println!(
        "Successfully extracted file \"{}\" from ROM to \"{}\"",
        entry_input_path, entry_extract_path
    );
    Ok(())
}
/// Replaces a file in a given ROM with a given input file.
fn rom_replace(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = get_arg!(args, 0, "ROM path")?;
    let input_file_path = get_arg!(args, 1, "input file path")?;
    let output_file_path = get_arg!(args, 2, "output file path")?;

    // Initialize the volume based on given ROM file path.
    let volume_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(rom_path)
        .map_err(|_| format!("Failed to open given ROM file in path \"{}\".", rom_path))?;
    let mut volume = CDROMXAVolume::new(volume_file);

    // Read the volume descriptor locations from the volume.
    let vd_locations = volume.read_volume_descriptor_locations().map_err(|e| {
        format!(
            "ROM file given has invalid data: failed to read volume descriptor locations: {}",
            e.to_string()
        )
    })?;

    let pvd = volume
        .read_primary_volume_descriptor(&vd_locations)
        .map_err(|e| {
            format!(
                "ROM file given has invalid data: failed to read primary volume descriptor: {}",
                e.to_string()
            )
        })?;

    // Read the sub-records, that is, directories and files by the root directory record
    let sub_records = volume
        .read_directory_records(
            &pvd.directory_record_for_root_directory,
            pvd.logical_block_size,
        )
        .map_err(|err| {
            format!(
                "ROM file given has invalid data: failed to read sub-records by root directory: {}",
                err.to_string()
            )
        })?;

    // Find the entry in the root directory record that matches the given entry path.
    let entry_record = sub_records
        .iter()
        .find(|r| r.file_identifier_as_string() == *output_file_path)
        .ok_or_else(|| {
            format!(
                "ROM file given does not contain the given entry path \"{}\".",
                output_file_path
            )
        })?;

    // Read the input file content into buffer.
    let input_file_content = {
        let mut input_file = File::open(input_file_path).map_err(|_| {
            format!(
                "Failed to open given input file in path \"{}\".",
                input_file_path
            )
        })?;

        let mut input_file_buffer = Vec::new();
        input_file
            .read_to_end(&mut input_file_buffer)
            .map_err(|_| {
                format!(
                    "Failed to read entire content of the input file in path \"{}\".",
                    input_file_path
                )
            })?;
        input_file_buffer
    };

    // Replace the entry record content with the input file content.
    volume.replace_file(entry_record, pvd.logical_block_size, &input_file_content)?;

    println!("ROM path: \"{}\"", rom_path);
    println!(
        "Replaced file \"{}\" in ROM successfully.",
        output_file_path
    );
    Ok(())
}
/// Reads information about WAD file. Heavily WIP.
fn wad_read(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let wad_path = get_arg!(args, 0, "WAD path")?;

    let wad = WAD::from_file_path(wad_path)?;

    let wad_reader = WADReader::new(&wad);

    let file_metadatum = wad_reader.read_file_metadatum_from_header()?;

    for (i, file_metadata) in file_metadatum.iter().enumerate() {
        println!("File #{} metadata: {:?}", i + 1, file_metadata);

        for (j, file) in wad_reader
            .read_subfiles_by_file_metadata(file_metadata)?
            .iter()
            .enumerate()
        {
            println!("File #{} content: {:?}", j + 1, file);
        }

        break;
    }

    Ok(())
}
