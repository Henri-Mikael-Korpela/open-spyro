use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Read;

use mips::{parse_nodes, CustomCommand, NodeKind};
use ps1exe::{PS1Exe, PS1ExeReader, PS1ExeWriteResult, PS1ExeWriter};
use rom_manager::CDROMXAVolume;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("------------------");
    println!("    OPEN SPYRO");
    println!("------------------");

    let args = env::args().collect::<Vec<String>>();

    let command = args
        .get(1)
        .ok_or_else(|| "No command provided as a command line argument.")?;

    match (command.as_str(), &args[2..]) {
        // Convert machine code into instruction string
        ("mips-disassemble", [value]) => {
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
        // Assemble MIPS assembly code from a given text file into a Playstation executable
        (
            "ps1exe-assemble",
            [input_assembly_code_file_path, input_ps1_exe_file_path, output_ps1_exe_file_path],
        ) => {
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

            let mut ps1_exe_writer = PS1ExeWriter::new(&mut ps1_exe);

            let mut current_address = 0;

            use colored::*;

            for node in nodes.iter() {
                match &node.kind {
                    NodeKind::Assignment(variable_name, value) => {
                        current_address += 4;

                        if let PS1ExeWriteResult::Changed { original_code } =
                            ps1_exe_writer.write_code(current_address, value.as_bytes())
                        {
                            println!(
                                "{}",
                                format!(
                                    "Assignment {} = {} - changed bytes to {:?} from {:?}",
                                    variable_name,
                                    value,
                                    value.as_bytes(),
                                    original_code
                                )
                                .red()
                            );
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
                                    "{} - changed bytes to {:?} from {:?}",
                                    instruction.to_instruction(),
                                    instruction.to_le_bytes(),
                                    original_code
                                )
                                .red()
                            );
                        }
                    }
                    _ => {}
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
        // Disassemble MIPS assembly code from given address (as hexadecimal) memory onwards
        ("ps1exe-disassemble", [input_ps1_exe_file_path, address_in_memory, instruction_count]) => {
            let ps1_exe = PS1Exe::from_file_path(&input_ps1_exe_file_path)?;

            let address_in_memory = u64::from_str_radix(address_in_memory, 16).map_err(|_| {
                format!(
                    "Failed to parse given address in memory \"{}\" as a hexadecimal number.",
                    address_in_memory
                )
            })?;

            let instruction_count = instruction_count.parse::<usize>().map_err(|_| {
                format!(
                    "Failed to parse given instruction count \"{}\" as a number.",
                    instruction_count
                )
            })?;

            let ps1_exe_reader = PS1ExeReader::new(&ps1_exe);
            ps1_exe_reader.disassemble_code_at(address_in_memory, instruction_count);

            Ok(())
        }
        // Check the given ROM file for validity.
        ("rom-check", [rom_path]) => {
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
                ).map_err(|err| format!("ROM file given has invalid data: failed to read sub-records by root directory: {}", err.to_string()))?;

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
        // Extracts a file from a given ROM to a given extract path.
        ("rom-extract", [rom_path, entry_input_path, entry_extract_path]) => {
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
                ).map_err(|err| format!("ROM file given has invalid data: failed to read sub-records by root directory: {}", err.to_string()))?;

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
        // Replaces a file in a given ROM with a given input file.
        ("rom-replace", [rom_path, input_file_path, output_file_path]) => {
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
                ).map_err(|err| format!("ROM file given has invalid data: failed to read sub-records by root directory: {}", err.to_string()))?;

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
        // Failure happened.
        _ => Err(format!(
            "No supported command or enough arguments for it provided (command \"{}\" given).",
            command
        )
        .into()),
    }
}
