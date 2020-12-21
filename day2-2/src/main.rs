use std::env;
use aoc_util::read_file_to_vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content: Vec<String> = read_file_to_vec(filename).unwrap();
    let mut correct_password_count: usize = 0;
    for line in file_content {
        // Parse out the different values we need from the input
        let mut iter = line.split_whitespace();

        let count_range: String = iter.next().unwrap().to_string();
        let mut count_iter = count_range.split("-");
        let idx_1: usize = count_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let idx_2: usize = count_iter.next().unwrap().parse::<usize>().unwrap() - 1;

        let target_char: char = iter.next().unwrap().to_string().replace(":", "").chars().nth(0).unwrap();

        let password: String = iter.next().unwrap().to_string();

        // Compare the password to the criteria        
        if (password.chars().nth(idx_1).unwrap() == target_char) != (password.chars().nth(idx_2).unwrap() == target_char) {
            correct_password_count += 1;
        }
    }
    println!("{}", correct_password_count);
}
