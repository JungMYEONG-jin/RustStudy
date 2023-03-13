mod a1;
mod a2;
extern crate core;
use a1::lastName;
use crate::a1::printFirstName;

fn main() {
    printFirstName();
    lastName();
    let i = a2::add(32, 32);
    a2::display_result(i);
}


fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
}

