mod a1;
mod a2;
mod a3;
mod a3b;
extern crate core;

fn main() {
    let mut cnt = 10;
    loop {
        println!("{:?} seconds to leave...", cnt);
        cnt = cnt -1;
        if cnt == 0 {
            print!("start!!");
            break;
        }
    }
}


fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
}

