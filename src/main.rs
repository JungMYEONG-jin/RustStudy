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
}