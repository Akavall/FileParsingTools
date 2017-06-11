use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp;

fn main() {


    let example_input = "Example Input: \x1b[0;32mcol_counter my_file.txt 2 \",\" 10 \x1b[0m. Where 2 -> 2nd column, \",\" -> delimiter, 10 -> first rows displayed";


    let args: Vec<String> = env::args().collect();

    if args.len() - 1 != 4 {
        panic!("Expecting 4 arguments, received: {}, {}", args.len() - 1, example_input);
    }

    // args is vector of owned Strings
    // String does not implement copy,
    // therefore, we can't simply index args,
    // because we would be moving an element
    // and invalidating a vector, we have to 
    // use a reference

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
