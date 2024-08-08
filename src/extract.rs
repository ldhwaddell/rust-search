use std::fs;
use std::io::{self};
use std::path::PathBuf;

pub struct Extractor {}

impl Extractor {
    pub fn extract(path: &PathBuf) -> io::Result<String> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("txt") => Self::from_txt(path),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported file extension",
            )),
        }
    }

    fn from_txt(path: &PathBuf) -> io::Result<String> {
        let contents = fs::read_to_string(path)?;
        Ok(contents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_extract_txt_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "Hello, world!").unwrap();

        let result = Extractor::extract(&file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[test]
    fn test_extract_unsupported_extension() {
        let file_path = PathBuf::from("test.unsupported");
        let result = Extractor::extract(&file_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_from_txt_nonexistent_file() {
        let file_path = PathBuf::from("nonexistent.txt");
        let result = Extractor::from_txt(&file_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }
}
