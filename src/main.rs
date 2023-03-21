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
use std::time::Duration;
use chrono::{DateTime, Local};
use humantime::format_duration;
use crate::a1::lastName;
use crate::a8::{Drink, Flavor};

// use lib name
use demo::print_from_lib;

fn main() {
    print_from_lib();
}
