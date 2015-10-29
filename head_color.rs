use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let file_name = &env::args().nth(1).unwrap();
    let col_number_str = &env::args().nth(2).unwrap();
    let col_number: usize = col_number_str.parse::<usize>().unwrap();

    // delim needs to char type
    let delim_raw = &env::args().nth(3).unwrap();
    let delim = delim_raw.chars().nth(0).unwrap();

    let n_lines_str = &env::args().nth(4).unwrap();
    let n_lines: usize = n_lines_str.parse::<usize>().unwrap();

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let mut row_number = 0;

    for line in reader.lines() {
        if row_number >= n_lines {
            break;
        }
        let line = line.unwrap();
        color_col(line.to_string(), col_number, delim);
        row_number += 1;
    }
}

fn color_col(my_string: String, col: usize, delim: char) {

    let mut col_tracker: usize = 0;
    let mut temp: String = "".to_string();
    let green: &str = "\x1b[0;32m";
    let no_color: &str = "\x1b[0m";

    let mut quote = false;

    for c in my_string.chars() {
        if c == '"' {
            if quote == false {
                quote = true
            } else {
                quote = false
            }
        }
        
        if c == delim && quote == false {
            if col_tracker == col {
                let mut colored: String = "".to_string();
                colored.push_str(green);
                colored.push_str(&temp);
                colored.push_str(no_color);
                print!("{}", colored);
                } else {
                print!("{}", temp);
            }
            temp = "".to_string();
            col_tracker += 1;

        }
        temp.push(c);
    }
    
    if col_tracker == col {
        let mut colored: String = "".to_string();
        colored.push_str(green);
        colored.push_str(&temp);
        colored.push_str(no_color);
        println!("{}", colored);
        } else {
            println!("{}", temp);
        }
}
