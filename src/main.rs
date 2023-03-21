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
    let a: Option<i32> = Some(1);
    let aSome = a.is_some();
    dbg!(aSome);
    let aNone = a.is_none();
    dbg!(aNone);
    let aMapped = a.map(|num| num+1);
    dbg!(aMapped);
    let aFilter = a.filter(|num| num == &1);
    dbg!(aFilter);
    // a가 값이 있으면 a 없다면 closer 값 반환
    let aElse = a.or_else(|| Some(5));
    dbg!(aElse);
    // 값이 없을때 기본값 부여하기에 좋음
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);

}
