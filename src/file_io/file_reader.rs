use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum FileReaderError {
    #[error("Failed to open file {0}: {1}")]
    OpenError(String, io::Error),
    #[error("Failed to read line: {0}")]
    ReadLineError(io::Error),
}

pub(crate) fn read_file_to_vec_string(filename: &str) -> Result<Vec<String>, FileReaderError> {
    let path = Path::new(filename);
    let file =
        File::open(&path).map_err(|e| FileReaderError::OpenError(filename.to_string(), e))?;
    let reader = io::BufReader::new(file);

    let mut line_list = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(FileReaderError::ReadLineError)?;
        line_list.push(line);
    }

    Ok(line_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_read_file_to_vec_string() {
        let test_file_path = "tests/source/test_file.txt";
        let test_content = "line1\nline2\nline3";

        // テスト用のファイルを作成
        let mut file = File::create(&test_file_path).expect("Failed to create test file");
        file.write_all(test_content.as_bytes())
            .expect("Failed to write to test file");

        // 関数をテスト
        let result = read_file_to_vec_string(test_file_path).expect("Failed to read file");
        assert_eq!(result, vec!["line1", "line2", "line3"]);

        // テスト用のファイルを削除
        std::fs::remove_file(test_file_path).expect("Failed to delete test file");
    }

    #[test]
    fn test_read_file_to_vec_string_empty_file() {
        let test_file_path = "tests/source/empty_file.txt";
        let test_content = "";

        // テスト用のファイルを作成
        let mut file = File::create(&test_file_path).expect("Failed to create test file");
        file.write_all(test_content.as_bytes())
            .expect("Failed to write to test file");

        // 関数をテスト
        let result = read_file_to_vec_string(test_file_path).expect("Failed to read file");
        assert_eq!(result, Vec::<String>::new());

        // テスト用のファイルを削除
        std::fs::remove_file(test_file_path).expect("Failed to delete test file");
    }

    #[test]
    fn test_read_file_to_vec_string_nonexistent_file() {
        let test_file_path = "tests/source/nonexistent_file.txt";

        // 関数をテスト
        let result = read_file_to_vec_string(test_file_path);
        assert!(result.is_err());
    }
}
