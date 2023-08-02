use std::env;
use std::fs::File;

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
        _ => Err(format!(
            "No supported command provided (command \"{}\" given).",
            command
        )
        .into()),
    }
}
