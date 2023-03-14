mod a1;
mod a2;
mod a3;
mod a3b;
mod a5;
mod a6;
extern crate core;

use std::collections::{BTreeMap, HashMap};

fn main() {
    a6::print5To1();
}


fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
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
