mod a1;
mod a2;
mod a3;
mod a3b;
extern crate core;
use a1::lastName;
use crate::a1::printFirstName;

fn main() {
    a3b::printValueCondition(3);
    a3b::printValueCondition(5);
    a3b::printValueCondition(6);
}


fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
}

