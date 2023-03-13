mod a1;
mod a2;
mod a3;
mod a3b;
extern crate core;

fn main() {
    let num = 5;
    match num {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("other"),
    }

}


fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
}

