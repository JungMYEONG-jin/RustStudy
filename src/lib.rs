use crate::helper::print_from_hep;

pub mod helper;
pub mod group;

pub fn print_from_lib(){
    use helper::{print_from_hep, print_again};
    println!("hello from lib");
    print_from_hep();
    group::g1::helloG1();
}