// use std::fs;
// use std::path::{Path, PathBuf};
// use std::error::Error;

// fn load_commands() -> Result<Vec<Box<dyn Fn()>>, Box<dyn Error>> {
//     let dir_path = Path::new("./commands");
//     let mut commands = Vec::new();

//     if let Ok(entries) = fs::read_dir(dir_path) {
//         for entry in entries.flatten() {
//             if let Ok(file_type) = entry.file_type() {
//                 if file_type.is_dir() {
//                     if let Ok(subdir_entries) = fs::read_dir(entry.path()) {
//                         for subdir_entry in subdir_entries.flatten() {
//                             if let Ok(subdir_file_type) = subdir_entry.file_type() {
//                                 if subdir_file_type.is_file() {
//                                     if let Some(module) = subdir_entry.path().file_stem().and_then(|f| f.to_str()) {
//                                         if let Some(function) = subdir_entry.path().file_name().and_then(|f| f.to_str()) {
//                                             let command_fn = format!("commands::{}::{}", module, function);
//                                             let command_fn = command_fn.parse::<Box<dyn Fn()>>()?;
//                                             commands.push(command_fn);
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Ok(commands)
// }