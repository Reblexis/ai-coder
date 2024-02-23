use shellexpand::full;
use super::*;

use super::Command;

pub mod create_dir;
pub mod create_file;
pub mod edit_file;
pub mod read_file;
pub mod remove_dir;
pub mod remove_file;
pub mod view_files;

pub fn expand_path(project_location: PathBuf, path: &str) -> Result<PathBuf, std::io::Error> {
    let binding = project_location.join(path);
    let full_path = binding.to_str().unwrap();
    let expanded_path = full(full_path)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to expand path"))?;
    // Check project_path is prefix of expanded_path
    if !expanded_path.starts_with(project_location.to_str().unwrap()) {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Path is not within project directory"));
    }
    Ok(PathBuf::from(expanded_path.as_ref()))
}

pub fn get_file_write_tools() -> HashMap<String, Box<dyn Command>> {
    let mut tools:HashMap<String, Box<dyn Command>> = HashMap::new();
    tools.insert("create_dir".to_string(), Box::new(create_dir::CreateDirCommand{}));
    tools.insert("create_file".to_string(), Box::new(create_file::CreateFileCommand{}));
    tools.insert("edit_file".to_string(), Box::new(edit_file::EditFileCommand{}));
    tools.insert("remove_dir".to_string(), Box::new(remove_dir::RemoveDirCommand{}));
    tools.insert("remove_file".to_string(), Box::new(remove_file::RemoveFileCommand{}));
    tools
}

pub fn get_file_read_tools() -> HashMap<String, Box<dyn Command>> {
    let mut tools:HashMap<String, Box<dyn Command>> = HashMap::new();
    tools.insert("read_file".to_string(), Box::new(read_file::ReadFileCommand{}));
    tools.insert("view_files".to_string(), Box::new(view_files::ViewFilesCommand{}));
    tools
}

pub fn get_all_file_tools() -> HashMap<String, Box<dyn Command>> {
    let mut tools:HashMap<String, Box<dyn Command>> = HashMap::new();
    tools.extend(get_file_write_tools());
    tools.extend(get_file_read_tools());
    tools
}
