use std::env;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn make_set_from_file (file_name: String) -> HashSet<String> {

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
    let file_name_1 = env::args().nth(1).unwrap();
    let file_name_2 = env::args().nth(2).unwrap();

    let function_type_raw = env::args().nth(3);
    let function_type: String;
    match function_type_raw {
        Some(my_val) => function_type = my_val,
        None => panic!("Error wrong number of arguments. Example input: \x1b[0;32mset_func file_name_1.txt file_name_2.txt -u\x1b[0m. Where -u -> union, -d -> difference, i -> intersection"),
    }

    let my_set_1 = make_set_from_file(file_name_1);
    let my_set_2 = make_set_from_file(file_name_2);

    let result_set: HashSet<String>;
    
    match &*function_type {
        "-u" => result_set = my_set_1.union(&my_set_2).cloned().collect(),
        "-d" => result_set = my_set_1.difference(&my_set_2).cloned().collect(),
        "-i" => result_set = my_set_1.intersection(&my_set_2).cloned().collect(),
        _ => panic!("Error function type not correctly specified. Example input: \x1b[0;32mset_func file_name_1.txt file_name_2.txt -u\x1b[0m. Where -u -> union, -d -> difference, i -> intersection")
    }

    for ele in result_set {
        println!("{}", ele);
    }
}
