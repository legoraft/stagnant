use std::{fs, path::Path};

fn copy_directory(source: &Path, destination: &Path) {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_directory_copies() {
        
    }
}