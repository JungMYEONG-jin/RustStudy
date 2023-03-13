mod a1;
mod a2;
mod a3;
mod a3b;
mod a5;
extern crate core;

fn main() {
    let mut cnt = 3;
    a5::printNumberWithLoop(cnt);
}


fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
}

