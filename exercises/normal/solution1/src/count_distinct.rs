use std::collections::HashSet;


pub fn new_count_distinct(input_str: &str) -> usize {
    let mut hash_set = HashSet::<&str>::new();
    let vec_char: Vec<&str> = input_str.split(",").collect();
    for item in vec_char {
        hash_set.insert(item);
    }
    hash_set.len()
}