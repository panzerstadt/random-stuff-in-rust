use std::fs;
use std::path::Path;

pub fn basic_ls(dir: &Path) -> Result<(), String> {
    if dir.is_dir() {
        // read directory
        let entries = dir.read_dir().unwrap();
        entries.for_each(|entry| {
            let entry = entry.unwrap();
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid: {:?}", f)));
            let file_name_result = file_name.unwrap();

            println!("{}", file_name_result);
        })
    }
    return Ok(());
}
