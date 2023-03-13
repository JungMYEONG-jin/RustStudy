mod a1;
mod a2;
mod a3;
extern crate core;
use a1::lastName;
use crate::a1::printFirstName;

fn main() {
    let mut mybool = true;
    a3::displayMessage(mybool);
    mybool = false;
    a3::displayMessage(mybool);
}


fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
}

