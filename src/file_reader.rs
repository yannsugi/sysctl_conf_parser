use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) fn read_file_to_vec_string(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to open file {}: {}", filename, e))
    })?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line =
            line.map_err(|e| io::Error::new(e.kind(), format!("Failed to read line: {}", e)))?;
        lines.push(line);
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_read_file_to_vec_string() {
        let test_file_path = "test_data/test_file.txt";
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
        let test_file_path = "test_data/empty_file.txt";
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
        let test_file_path = "test_data/nonexistent_file.txt";

        // 関数をテスト
        let result = read_file_to_vec_string(test_file_path);
        assert!(result.is_err());
    }
}
