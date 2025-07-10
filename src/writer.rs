use std::fs;
use std::path::Path;

pub fn write_to_file(output_dir: &str, class_name: &str, contents: &str) -> std::io::Result<()> {
    let filename = format!("{class_name}.cs");
    let full_path = Path::new(output_dir).join(filename);
    fs::create_dir_all(output_dir)?;
    fs::write(full_path, contents)?;
    Ok(())
}
