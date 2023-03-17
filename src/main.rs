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

    // secret file admin only
    let accessLevel = Access::GUEST;
    let canAccessFile = match accessLevel {
        Access::ADMIN => true,
        _ => false,
    };
    assert_eq!(canAccessFile, false);
}

pub fn getPos() -> (i32, i32){
    let mut rng = rand::thread_rng();
    (rng.gen::<i32>(), rng.gen::<i32>())
}

fn oneTwoThree() -> (i32, i32, i32){
    (1, 2, 3)
}

struct Shop{
    stock: i32,
    price: f64,
}


pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut db = BTreeMap::new();
    let len = s.len() as i32;
    for i in 0..len{
        db.insert(indices[i as usize], s.as_bytes()[i as usize] as char);
    };
    let mut result = String::new();
    for key in db.keys(){
        result.push(*db.get(key).unwrap()); // &char to char
    };
    return result;
}

enum Dir{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn which_dir(dir: Dir) -> &'static str {
    let mut result = "";
    match dir{
        Dir::UP => result = "up",
        Dir::DOWN => result = "down",
        Dir::LEFT => result = "left",
        Dir::RIGHT => result = "right",
        _ => result = "Wrong value!!"
    };
    return result;
}