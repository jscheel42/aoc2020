use std::convert::TryInto;
use std::env;
use aoc_util::read_file_to_vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content: Vec<String> = read_file_to_vec(filename).unwrap();
    let mut correct_password_count: u64 = 0;
    for line in file_content {
        // Parse out the different values we need from the input
        let mut iter = line.split_whitespace();

        let count_range: String = iter.next().unwrap().to_string();
        let mut count_iter = count_range.split("-");
        let count_min: u64 = count_iter.next().unwrap().parse::<u64>().unwrap();
        let count_max: u64 = count_iter.next().unwrap().parse::<u64>().unwrap();

        let target_char: String = iter.next().unwrap().to_string().replace(":", "");

        let password: String = iter.next().unwrap().to_string();

        // Compare the password to the criteria
        let target_char_count: u64 = password.matches(&target_char).count().try_into().unwrap();
        if (target_char_count >= count_min) && (target_char_count <= count_max) {
            correct_password_count += 1;
        }
    }
    println!("{}", correct_password_count);
}
