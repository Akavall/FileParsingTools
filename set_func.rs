use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn make_set_from_file<'a>(file_name: str) -> HashSet<&'a str> {

    let mut my_set: HashSet<&'a str> = HashSet::new();
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line: a'& str = line.unwrap();
        my_set.push(line);
    }
    my_set 
}

fn main() {
    let file_name_1 = &env::args().nth(1).unwrap();
    let file_name_2 = &env::args().nth(2).unwrap();
    let funcion_type = &env::args().nth(3).unwrap();

    let my_set_1 = make_set_from_file(file_name_1);
    let my_set_2 = make_set_from_file(file_name_2);

    let mut result_set: HashSet<&str> = HashSet::new();
    if function_type == "-u" {
        result_set = my_set_1.union(my_set_2);
    }

    for ele in result_set {
        println!("{}", ele);
    }
}
