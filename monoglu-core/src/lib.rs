use std::env;
use std::fs;
use std::path::PathBuf;

pub type Exception = Box<dyn std::error::Error>;

pub fn storage_path() -> Result<PathBuf, Exception> {
    let mut base_path = env::current_dir()?;
    base_path.push("storage");
    Ok(base_path)
}

pub fn load_file(path: &PathBuf, file_name: &str) -> Result<(), Exception> {
    let mut file_path = PathBuf::new();
    file_path.push(path);
    file_path.push(file_name);

    let file = fs::File::options()
        .read(true)
        .open(path)?;
    Ok(())
}