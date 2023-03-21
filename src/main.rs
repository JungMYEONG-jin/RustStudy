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

trait Fall{
    fn hitGround(&self);
}

struct Vase;
impl Fall for Vase{
    fn hitGround(&self) {
        println!("the vase broken");
    }
}

struct Cat;
impl Fall for Cat{
    fn hitGround(&self) {
        println!("the cat casually walked away");
    }
}

fn fall(thing: impl Fall){
    thing.hitGround();
}
fn main() {
    fall(Vase  {});
    fall(Cat  {});
}
