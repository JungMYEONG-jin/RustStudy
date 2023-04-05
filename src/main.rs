use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;
use crate::sample_error::CustomError;

mod sample_error;

// panic을 하지 않고 unwrap 하는법 try!

type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> sample_error::Result<i32> {
    let first =  vec.first().ok_or(CustomError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: sample_error::Result<i32>){
    match result{
        Ok(n) => println!("The first double value is {}", n),
        Err(e) => println!("Error {}", e),
    }
}

fn main() {
    let nums = vec!["93", "22"];
    let vec = vec![];
    let strings = vec!["tt", "93", "22"];
    print(double_first(vec));
    print(double_first(nums));
    print(double_first(strings));
}