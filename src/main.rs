mod a1;
mod a2;
mod a3;
mod a3b;
mod a5;
mod a6;
mod a7;
mod a8;
mod bills;
mod newType;

use bills::*;

extern crate core;
extern crate alloc;


use core::borrow::{Borrow, BorrowMut};
use rand::Rng;
use std::io;
use std::collections::{BTreeMap, HashMap};
use std::time::Duration;
use chrono::{DateTime, Local};
use humantime::format_duration;
use crate::a1::lastName;
use crate::a8::{Drink, Flavor};

// use ${lib name 내가 toml에 명시한 lib 이름}
use demo::{group, print_from_lib};

use thiserror::Error;

#[derive(Debug, Error)]
enum ProgramError{
    #[error("menu item not found")]
    Menu(#[from] MenuError),

    #[error("divide error")]
    Divide(#[from] MathError),
}

#[derive(Debug, Error)]
enum MenuError{
    #[error("menu item not found")]
    NotFound,
}

#[derive(Debug, Error)]
enum MathError{
    #[error("dive by zero error")]
    DivideByZero,
}

fn pick_menu(choice: &str) -> Result<i32, MenuError>{
    match choice {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => Err(MenuError::NotFound),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, MathError>{
    if b!=0 {
        Ok(a/b)
    }else {
        Err(MathError::DivideByZero)
    }
}

fn run(step: i32) -> Result<(), ProgramError>
{
    if step == 1 {
        pick_menu("4")?;
    }else if step == 2 {
        divide(1, 0)?;
    }
    Ok(())
}
fn main() {
    println!("{:?}", run(1));
    println!("{:?}", run(2));
}


#[cfg(test)]
mod test{
    use crate::newType::{divide, NeverZero};

    #[test]
    fn matchTest(){
        match NeverZero::new(0) {
            Ok(res) => println!("{:?}", divide(10, res)),
            Err(e) => println!("{:?}", e),
        }
    }
}