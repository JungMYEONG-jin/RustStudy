mod a1;
mod a2;
mod a3;
mod a3b;
mod a5;
mod a6;
mod a7;
mod a8;
extern crate core;

use rand::Rng;
use std::collections::{BTreeMap, HashMap};
use crate::a7::Color;
use crate::a8::{Drink, Flavor};

enum Access{
    ADMIN,
    USER,
    GUEST
}

fn main() {
    let val = 100;
    let str = val > 100;
    let result = match  str {
        true => "big",
        false => "small"
    };
    println!("{:?}", result);
}
