use advent_of_code_2024::puzzels::{day_one::historian_hysteria, day_two::red_nosed_reports};

fn main() {
    const DAY: i32 = 2; // Define the Puzzle

    match DAY{
        1 => {
            historian_hysteria::solve();
            historian_hysteria::solve_part_two();
        },
        2 => {
            red_nosed_reports::solve();
        },
        _=>println!("Out of Index"),
    }
    
}
