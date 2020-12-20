use std::env;
use aoc_util::read_file_to_vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let (i, j) = get_nums(filename).unwrap();
    println!("{} x {}", i, j);
    println!("{}", i * j);    
}

// fn get_nums(filename: &String) -> std::io::Result<(u64, u64)>{
fn get_nums(filename: &String) -> Result<(u64, u64), std::fmt::Error> {
    let vec: Vec<u64> = read_file_to_vec(filename).unwrap();
    
    for i in vec.iter() {
        for j in vec.iter() {
            if i + j == 2020 {
                return Ok((*i, *j))
            }
        }
    }

    Err(std::fmt::Error)
}