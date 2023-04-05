use crate::helper::print_from_hep;

pub mod helper;
pub mod group;
mod customError;
mod newType;
mod sample_error;
mod sample_enum;

pub fn print_from_lib(){
    use helper::{print_from_hep, print_again};
    println!("hello from lib");
    print_from_hep();
    group::g1::helloG1();
}