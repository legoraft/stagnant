use std::{fs, io, path::Path};

pub fn copy_directory(source: &Path, destination: &Path) -> io::Result<()> {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }
    
    for entry_result in fs::read_dir(source)? {
        let entry = entry_result?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        
        if source_path.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        } else if source_path.is_file() {
            fs::copy(&source_path, &destination_path)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use fs::File;
    use tempfile::tempdir;
    use std::io::Write;

    use super::*;
    
    #[test]
    fn test_directory_copies() {
        let temp_source = tempdir().unwrap();
        let temp_destination = tempdir().unwrap();
        
        fs::create_dir(temp_source.path().join("subdir")).unwrap();
        let top_path = temp_source.path().join("top_file.txt");
        let sub_path = temp_source.path().join("./subdir/sub_file.txt");

        let mut top_file = File::create(top_path).unwrap();
        let mut sub_file = File::create(sub_path).unwrap();
        writeln!(top_file, "Hello, world!").unwrap();
        writeln!(sub_file, "Hello, again!").unwrap();
        
        copy_directory(temp_source.path(), temp_destination.path()).unwrap();
        
        assert!(temp_destination.path().join("top_file.txt").exists());
        assert!(temp_destination.path().join("subdir/sub_file.txt").exists());
        
        let top_content = fs::read_to_string(temp_destination.path().join("top_file.txt")).unwrap();
        let sub_content = fs::read_to_string(temp_destination.path().join("subdir/sub_file.txt")).unwrap();
        assert_eq!(top_content, "Hello, world!\n");
        assert_eq!(sub_content, "Hello, again!\n");
    }
}