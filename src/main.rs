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
use crate::a1::lastName;
use crate::a7::Color;
use crate::a8::{Drink, Flavor};

#[derive(Debug, Eq, PartialEq)]
enum Access{
    Admin,
    User,
    Guest,
}

fn maybeAccess(name: &str) -> Option<Access>{
    match name {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None,
    }
}

fn root() -> Option<Access>{
    Some(Access::Admin)
}

fn main() {

    let numbers = vec![1, 2, 3, 4, 5];
    println!("triple start...");
    // lv1
    let tripleAndFilter: Vec<_> = numbers.iter().map(|x| 3 * x).filter(|num| num > &10).collect();
    for num in tripleAndFilter{
        println!("{:?}", num);
    }
}
