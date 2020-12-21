use std::env;
use aoc_util::read_file_to_vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content: Vec<String> = read_file_to_vec(filename).unwrap();
    let mut idx: usize = 0;
    let tree: char = "#".chars().nth(0).unwrap();
    let mut tree_count: usize = 0;

    for line in file_content {
        idx = get_current_idx(idx, line.len());
        if line.chars().nth(idx).unwrap() == tree {
            tree_count += 1;
        }
        idx += 3;
    }

    println!("{}", tree_count);
}

fn get_current_idx(mut idx: usize, length: usize) -> usize {
    if idx >= length {
        idx %= length;
    }
    return idx
}