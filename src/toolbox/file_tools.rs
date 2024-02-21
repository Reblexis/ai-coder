use std::path::PathBuf;
use shellexpand::full;

mod file_tools_common;
mod view_files;
mod read_file;
mod edit_file;
mod create_directory;
mod create_file;
mod remove_file;

pub fn expand_path(path: &str) -> Result<PathBuf, std::io::Error> {
    let expanded_path = full(path)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to expand path"))?;
    Ok(PathBuf::from(expanded_path.as_ref()))
}
