// I AM NOT DONE

mod district;
use std::{collections::HashMap, fs};
fn main() {
    let provinces = district::count_provinces();
    println!("provinces: {provinces}");
}
