use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn make_set_from_file (file_name: &str) -> HashSet<String> {

    let mut my_set: HashSet<String> = HashSet::new();
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        my_set.insert(line);
    }
    my_set 
}

fn main() {
    let file_name_1: &str = &env::args().nth(1).unwrap();
    let file_name_2: &str = &env::args().nth(2).unwrap();
    let function_type = env::args().nth(3).unwrap();

    let my_set_1 = make_set_from_file(file_name_1);
    let my_set_2 = make_set_from_file(file_name_2);

    let mut result_set: HashSet<String> = HashSet::new();
    
    if function_type == "-u" {
        result_set = my_set_1.union(&my_set_2).cloned().collect();
    } else if function_type == "-d" {
        result_set = my_set_1.difference(&my_set_2).cloned().collect();
    } else if function_type == "-i" {
        result_set = my_set_1.intersection(&my_set_2).cloned().collect();
    } else {
        println!("ERROR: function_type is not specified: use: -u, -d, -i")
    }

    for ele in result_set {
        println!("{}", ele);
    }
}
