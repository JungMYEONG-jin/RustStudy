use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

// panic을 하지 않고 unwrap 하는법 try!

type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {

    let first = vec.first().ok_or("There is no element in vector".to_owned())?;

    let value = first.parse::<i32>().map_err(|e| e.to_string())?;
    Ok(2 * value)
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