use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let file_name = &env::args().nth(1).unwrap();

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let mut row_number = 0;

    for line in reader.lines() {
        row_number += 1;
    }
    println!("Number of rows: {}", row_number)
}
