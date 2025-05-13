fn main() {
    match read_file_content("example.txt") {
        Ok(content) => println!("File content : {}", content),
        Err(e) => println!("Error readling file: {}", e),
    }
}

fn read_file_content(filename: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content)
}