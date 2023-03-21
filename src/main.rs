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

trait Perimeter{
    fn getPerimeter(&self) -> i32;
}

struct Triangle{
    a: i32,
    b: i32,
    c: i32,
}
impl Perimeter for Triangle{
    fn getPerimeter(&self) -> i32 {
        println!("a+b+c");
        self.a+self.b+self.c
    }
}

struct Square{
    a: i32,
}
impl Perimeter for Square{
    fn getPerimeter(&self) -> i32 {
        println!("4 * side");
        4*self.a
    }
}

fn calcPrimeter(thing: impl Perimeter){
    let perimeter = thing.getPerimeter();
    println!("perimeter {:?}", perimeter);
}

fn main() {
    calcPrimeter(Triangle  {a:3, b:2, c:1});
    calcPrimeter(Square  { a: 2});
}
