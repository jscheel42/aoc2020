use std::fs::File;
use std::vec::Vec;
use std::io::{self, prelude::*, BufReader}; // https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust

pub fn read_file_to_vec<T: std::str::FromStr>(filename: &String) -> io::Result<Vec<T>> where <T as std::str::FromStr>::Err: std::fmt::Debug {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut vec: Vec<T> = Vec::new();
    for line_str in reader.lines() {
        let line: T = line_str?.parse().unwrap();
        vec.push(line);
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
