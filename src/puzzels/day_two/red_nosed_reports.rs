use crate::helpers::file_helper;

pub fn solve() {
    println!("Solving Puzzel Day Two");

    // First get the Input as String
    let content = file_helper::get_string_input_from_file("./input/day_two/input.txt");
    println!("Content Length: {}", content.lines().count());

    // Define counter for our result
    let mut safe_line_counter = 0;
    let mut unsafe_line_counter = 0;

    // Last but not least, check if the line is Safe or not
    for line in content.lines() {
        if is_line_safe(line){
            safe_line_counter +=1;
        } else {
            unsafe_line_counter +=1;
        }
    }

    // Tada.. our Result
    println!("Safe Lines: {}", safe_line_counter);
    println!("Unsafe Lines: {}", unsafe_line_counter);
}

fn is_line_safe(line: &str) -> bool {
    let values: Vec<i32> = line
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    if values.len() <= 1 {
        return true; // A single value is always “safe”
    }

    let mut all_increasing = true;
    let mut all_decreasing = true;

    for i in 0..values.len() - 1 {
        let diff = (values[i + 1] - values[i]).abs();
        if diff < 1 || diff > 3 {
            return false; // Difference outside the permitted range
        }

        if values[i + 1] < values[i] {
            all_increasing = false;
        } else if values[i + 1] > values[i]{
            all_decreasing = false;
        } else {
            return false; // No change between adjacent values
        }
    }

    all_increasing || all_decreasing
}
