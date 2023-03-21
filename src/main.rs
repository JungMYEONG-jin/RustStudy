mod a1;
mod a2;
mod a3;
mod a3b;
mod a5;
mod a6;
mod a7;
mod a8;
mod bills;

use bills::*;

extern crate core;
extern crate alloc;


use core::borrow::{Borrow, BorrowMut};
use rand::Rng;
use std::io;
use std::collections::{BTreeMap, HashMap};
use crate::a7::Color;
use crate::a8::{Drink, Flavor};


#[derive(Debug)]
struct User{
    id: i32,
    name: String,
}

fn findUser(name: &str) -> Option<i32>{
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        N_ => None,
    }
}


fn main() {
    // closer
    let mut buffer= String::new();
    let result = io::stdin().read_line(&mut buffer);
    if result.is_err() {
        println!("Error...");
    }
    buffer = buffer.trim().to_owned();
    println!("{:?}", buffer);
    let user = findUser(buffer.as_str()).map(|id| User{
        id, name: buffer.to_owned(),
    });

    match user {
        Some(user) => println!("{:?}", user),
        None => println!("user not found"),
    }

}

fn allCaps(word: &str) -> String{
    word.to_uppercase()
}

fn clamp(n: i32, lower: i32, upper: i32) -> i32{
    if n < lower {
        lower
    }else if n > upper {
        upper
    } else {
        n
    }
}

fn div(a: i32, b: i32) -> Option<i32>{
    if b == 0 {
        None
    }else {
        Some(a / b)
    }
}


#[cfg(test)]
mod test{
    // crate 소스의 최상단.
    use crate::*;
    #[test]
    fn check_all_caps(){
        let res = allCaps("hello");
        let expected = String::from("HELLO");
        assert_eq!(res, expected, "string should be all uppercase");
    }

    #[test]
    fn check_clamp(){
        let res = clamp(3, 4, 5);
        let expected = 4;
        assert_eq!(res, expected, "Should be 4");
    }

    #[test]
    fn check_clamp_upper(){
        let res = clamp(3000, 100, 500);
        let expected = 500;
        assert_eq!(res, expected, "Should be 500");
    }

    #[test]
    fn div_test(){
        let res = div(3, 0);
        let expected = None;
        assert_eq!(res, expected, "should be 0");
    }

    #[test]
    fn div_upper_Zero(){
        let res = div(0, 3);
        let expected = 0;
        assert_eq!(res.unwrap(), expected, "should be 0");
    }
}