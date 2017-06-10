use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp;

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

    let mut my_counter: HashMap<String, usize> = HashMap::new();

    for line in reader.lines() {

        let line = line.unwrap();
        let col_string = color_col(line.to_string(), col_number, delim);
        
        if my_counter.contains_key(&col_string) {
            *my_counter.get_mut(&col_string).unwrap() += 1;
        } else {
            my_counter.insert(col_string, 1);
        }
    }

    let mut my_counter_items: Vec<(String, usize)> = my_counter.clone().into_iter().collect();
    my_counter_items.sort_by(|a, b| b.1.cmp(&a.1));

    let n_lines_to_print = cmp::min(n_lines, my_counter_items.len());

    let green: &str = "\x1b[0;33m";
    let no_color: &str = "\x1b[0m"; 

    let mut header: String = "".to_string();
    header.push_str(green);
    header.push_str("Rank: Value: Count");
    header.push_str(no_color);

    println!("{}", header);

    for ind in 0..(n_lines_to_print) {
        println!("{}: {}: {}", ind+1, my_counter_items[ind].0, my_counter_items[ind].1);
    }
    
}

    // let green: &str = "\x1b[0;32m";
    // let no_color: &str = "\x1b[0m";


fn color_col(my_string: String, col: usize, delim: char) -> String {

    let mut col_tracker: usize = 0;
    
    let mut quote = false;
    let mut col_string = "".to_string();

    for c in my_string.chars() {
        if col == col_tracker {
            if c != delim || quote == true {
                col_string.push(c)
            }
        }
        
        if c == '"' {
            if quote == false {
                quote = true
            } else {
                quote = false
            }
        }
        
        if c == delim && quote == false {
            col_tracker += 1;
        }

        if col_tracker > col {
            break;
        }
    }
    return col_string;
}
