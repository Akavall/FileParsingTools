use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let file_name = &env::args().nth(1).unwrap();
    let col_number_str = &env::args().nth(2).unwrap();
    let col_number: usize = col_number_str.parse::<usize>().unwrap();
    let delim = &env::args().nth(3).unwrap();
    let n_lines_str = &env::args().nth(4).unwrap();
    let n_lines: usize = n_lines_str.parse::<usize>().unwrap();

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
            print!("{}{}", my_vec[i], delim);
        }
        print!("{}", colored_string);
        for i in ((col_number+1))..(my_vec.len()) {
            print!("{}{}", delim, my_vec[i]);
        }
        println!("");
        row_number += 1;
    }
}
