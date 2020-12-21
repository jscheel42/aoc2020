use std::env;
use aoc_util::read_file_to_vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let (i, j) = get_2_nums(filename).unwrap();
    println!("{} x {}", i, j);
    println!("{}", i * j);    
    let (i, j, k) = get_3_nums(filename).unwrap();
    println!("{} x {} x {}", i, j, k);
    println!("{}", i * j * k);    
}

fn get_2_nums(filename: &String) -> Result<(u64, u64), std::fmt::Error> {
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

fn get_3_nums(filename: &String) -> Result<(u64, u64, u64), std::fmt::Error> {
    let vec: Vec<u64> = read_file_to_vec(filename).unwrap();
    
    for i in vec.iter() {
        for j in vec.iter() {
            for k in vec.iter() {
                if i + j + k == 2020 {
                    return Ok((*i, *j, *k))
                }
            }
        }
    }

    Err(std::fmt::Error)
}