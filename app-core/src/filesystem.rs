use std::fs;
use std::io;
use std::path::Path;

/// Returns the names of non-hidden folders in the given directory.
pub fn list_folder_names(root: &Path) -> io::Result<Vec<String>> {
    let mut folders = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if entry.file_type()?.is_dir() && !name.starts_with('.') {
            folders.push(name.into_owned());
        }
    }
    folders.sort();
    Ok(folders)
}

/// Returns the names of non-hidden files in the given directory.
pub fn list_file_names(root: &Path) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if entry.file_type()?.is_file() && !name.starts_with('.') {
            files.push(name.into_owned());
        }
    }
    files.sort();
    Ok(files)
}

/// Returns the names of hidden files (starting with '.') in the given directory.
pub fn list_hidden_file_names(root: &Path) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if entry.file_type()?.is_file() && name.starts_with('.') {
            files.push(name.into_owned());
        }
    }
    files.sort();
    Ok(files)
}

/// Returns the names of hidden folders (starting with '.') in the given directory.
pub fn list_hidden_folder_names(root: &Path) -> io::Result<Vec<String>> {
    let mut folders = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if entry.file_type()?.is_dir() && name.starts_with('.') {
            folders.push(name.into_owned());
        }
    }
    folders.sort();
    Ok(folders)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    fn setup_test_dir() -> TempDir {
        let dir = TempDir::new().unwrap();
        let root = dir.path();

        // Non-hidden folders
        fs::create_dir(root.join("docs")).unwrap();
        fs::create_dir(root.join("src")).unwrap();

        // Hidden folders
        fs::create_dir(root.join(".git")).unwrap();
        fs::create_dir(root.join(".config")).unwrap();

        // Non-hidden files
        File::create(root.join("README.md")).unwrap();
        File::create(root.join("main.rs")).unwrap();

        // Hidden files
        File::create(root.join(".gitignore")).unwrap();
        File::create(root.join(".env")).unwrap();

        dir
    }

    #[test]
    fn test_list_folder_names() {
        let dir = setup_test_dir();
        let result = list_folder_names(dir.path()).unwrap();
        assert_eq!(result, vec!["docs", "src"]);
    }

    #[test]
    fn test_list_file_names() {
        let dir = setup_test_dir();
        let result = list_file_names(dir.path()).unwrap();
        assert_eq!(result, vec!["README.md", "main.rs"]);
    }

    #[test]
    fn test_list_hidden_file_names() {
        let dir = setup_test_dir();
        let result = list_hidden_file_names(dir.path()).unwrap();
        assert_eq!(result, vec![".env", ".gitignore"]);
    }

    #[test]
    fn test_list_hidden_folder_names() {
        let dir = setup_test_dir();
        let result = list_hidden_folder_names(dir.path()).unwrap();
        assert_eq!(result, vec![".config", ".git"]);
    }

    #[test]
    fn test_nonexistent_directory() {
        let result = list_folder_names(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }
}
