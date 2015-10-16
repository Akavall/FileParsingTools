use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let file_name = &env::args().nth(1).unwrap();
    let col_number_str = &env::args().nth(2).unwrap();
    let col_number: u32 = col_number_str.parse::<u32>().unwrap();
    let delim = &env::args().nth(3).unwrap();
    let n_lines_str = &env::args().nth(4).unwrap();
    let n_lines: u32 = n_lines_str.parse::<u32>().unwrap();

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let green = "\x1b[0;32m";
    let nc = "\x1b[0m";

    let mut row_number = 0;

    for line in reader.lines() {
        if row_number > n_lines {
            break;
        }
        let line = line.unwrap();
        let my_vec: Vec<&str> = line.split(",").collect();
        let colored_string = "".to_string() + green + my_vec[2] + nc;

        for i in 0..col_number {
            print!("{}{}", my_vec[i as usize], delim);
        }
        print!("{}", colored_string);
        for i in ((col_number+1) as usize)..(my_vec.len()) {
            print!("{}{}", delim, my_vec[i as usize]);
        }
        println!("");
        row_number += 1;
    }
}
