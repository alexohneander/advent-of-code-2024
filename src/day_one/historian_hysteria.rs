use std::fs;

pub fn solve(){
    println!("Solving Puzzle Day One");

    // First get the Values from the Input file
    let content = get_input_from_file();

    if content != "" {
        println!("Content is not Empty");
    }
}


fn get_input_from_file() -> String {
    let file_path = "./input/day_one/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return contents;
}

