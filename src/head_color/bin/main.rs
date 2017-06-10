use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let example_input = "Example Input: \x1b[0;32mhead_color my_file.txt 2 \",\" 10 \x1b[0m. Where 2 -> 2nd column, \",\" -> delimiter, 10 -> first rows displayed";


    let args: Vec<String> = env::args().collect();

    if args.len() - 1 != 4 {
        panic!("Expecting 4 arguments, received: {}, {}", args.len() - 1, example_input);
    }

    let ref file_name = args[1];
    let ref col_number_str = args[2];

    let col_number: usize = match col_number_str.parse::<usize>() {
        Ok(col_number) => col_number,
        Err(_) => panic!("Could not parse the col number: {}", example_input),
    };

    // delim needs to be char type
    let ref delim_raw = args[3];

    // Still needs to be handled properly
    // delim ",34353" => ","
    let delim = match delim_raw.chars().nth(0) {
        Some(delim) => delim,
        None => panic!("Could not parse the delimiter: {}", example_input),
    };

    let ref n_lines_str = args[4];

    let n_lines: usize = match n_lines_str.parse::<usize>() {
        Ok(n_lines) => n_lines,
        Err(_) => panic!("Could not parse n_lines: {}", example_input),
    };

    let f = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => panic!("Could not open file: {} check if the file exists. Are you sure your input is correct? {}", file_name, example_input),
    };

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
