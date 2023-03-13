mod a1;
extern crate core;
use a1::lastName;
use crate::a1::printFirstName;

fn main() {
    printFirstName();
    lastName();
}

fn add(a: i32, b: i32) -> i32 {
    a+b
}

fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
}

