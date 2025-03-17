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
