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
    fn getPerimeter(&self);
}

struct Triangle;
impl Perimeter for Triangle{
    fn getPerimeter(&self) {
        println!("a+b+c");
    }
}

struct Square;
impl Perimeter for Square{
    fn getPerimeter(&self) {
        println!("4 * side");
    }
}

fn calcPrimeter(thing: impl Perimeter){
    thing.getPerimeter();
}

fn main() {
    calcPrimeter(Triangle  {});
    calcPrimeter(Square  {});
}
