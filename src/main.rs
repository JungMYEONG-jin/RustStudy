// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = match vec.first() {
        Some(value) => value,
        None => return Err("There is no element in vector".to_owned()),
    };
    match first.parse::<i32>(){
        Ok(i) => Ok(2*i),
        Err(e) => Err(e.to_string()),
    }
}

fn print(result: Result<i32>){
    match result{
        Ok(n) => println!("The first double value is {}", n),
        Err(e) => println!("Error {}", e),
    }
}

fn main() {
    let vec = vec![];
    let strings = vec!["tt", "93", "22"];
    print(double_first(vec));
    print(double_first(strings));
}