use std::fs;

pub fn solve(){
    println!("Solving Puzzle Day One");

    // First get the Values from the Input file
    let content = get_input_from_file();

    // Then parse it into an Vector
    let (left_values, right_values) = parse_input_into_array(content);
    println!("Left Values Count: {:?}", left_values.len());
    println!("Right Values Count: {:?}", right_values.len());

    // Now solve the Puzzle
    let distances = get_distances(left_values, right_values);
    let mut final_value: i32 = 0;
    for value in distances.iter() {
        final_value += value;
    }

    println!("Finale Result: {}", final_value);
}


fn get_input_from_file() -> String {
    let file_path = "./input/day_one/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return contents;
}

fn parse_input_into_array(content: String) -> (Vec<i32>, Vec<i32>) {
    if &content == "" {
        panic!("Content is Empty");
    } 
    println!("Content is not Empty");

    let line_count: &usize = &content.lines().count();

    // Init two Vectors
    let mut left_values = vec![0; line_count.clone()];
    let mut right_values = vec![0; line_count.clone()];

    // Fill Vectors with values
    let mut counter = 0;
    for line in content.lines() {
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse().expect("Not an integer")).collect();

        left_values[counter] = values[0];
        right_values[counter] = values[1];

        counter+=1;
    }

    return (left_values,right_values);
}

fn get_distances(values_left: Vec<i32>, values_right: Vec<i32>) -> Vec<i32> {
    let mut left_values = values_left.to_owned();
    let mut right_values = values_right.to_owned();
    
    // First check if the left and right values have the same count else panic
    if &left_values.len() != &right_values.len() {
        panic!();
    }

    let countet_size = left_values.len().to_owned();

    let mut distances = vec![0; countet_size.clone()];

    let mut counter = 0;
    while counter != countet_size {
        // first finde the two smallest values
        let min_value_left = &left_values.iter().min().unwrap().to_owned();
        let index_left = left_values.iter().position(|x| x == min_value_left).unwrap();

        let min_value_right = &right_values.iter().min().unwrap().to_owned();
        let index_right = right_values.iter().position(|x| x == min_value_right).unwrap();

        // remove this values from vectors
        left_values.remove(index_left);
        right_values.remove(index_right);
        
        // get distance between this values
        let distance = get_distance_from_values(*min_value_left, *min_value_right);
        distances[counter] = distance;

        counter+=1;
    }

    return distances;
}

fn get_distance_from_values(value_left: i32, value_right: i32) -> i32 {
    let distance: i32;
    if value_left > value_right {
        distance = value_left - value_right;
    } else if value_right > value_left {
        distance = value_right - value_left;
    } else if value_left == value_right{
        distance = 0;
    } else {
        distance = 0;
        println!("WTF");
    }

    return distance;
}