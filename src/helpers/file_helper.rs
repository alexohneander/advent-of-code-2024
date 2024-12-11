use std::fs;


pub fn get_string_input_from_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return contents;
}