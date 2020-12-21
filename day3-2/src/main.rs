use std::env;
use aoc_util::read_file_to_vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content: Vec<String> = read_file_to_vec(filename).unwrap();
    
    let tree_counts: Vec<usize> = vec!(
        find_trees(file_content.clone(), 1, 1),
        find_trees(file_content.clone(), 3, 1),
        find_trees(file_content.clone(), 5, 1),
        find_trees(file_content.clone(), 7, 1),
        find_trees(file_content, 1, 2)
    );

    let product = tree_counts.iter().product::<usize>();

    println!("{}", product)

}

fn get_next_idx(mut idx: usize, length: usize, slope_right: usize) -> usize {
    idx += slope_right;
    if idx >= length {
        idx %= length;
    }
    return idx
}

fn find_trees(file_content: Vec<String>, slope_right: usize, slope_down: usize) -> usize {
    let mut idx: usize = 0;
    let tree: char = "#".chars().nth(0).unwrap();
    let mut tree_count: usize = 0;
    let mut i = 0;
    
    while i < file_content.len() {
        let line = file_content.get(i).unwrap();
        if line.chars().nth(idx).unwrap() == tree {
            tree_count += 1;
        }

        idx = get_next_idx(idx, line.len(), slope_right);
        if slope_down > 1 {
            i += slope_down - 1;
        }

        i += 1;
    }

    println!("Right {}, down {} -> {}", slope_right, slope_down, tree_count);
    return tree_count
}