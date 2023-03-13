mod a1;
mod a2;
mod a3;
mod a3b;
extern crate core;

fn main() {
    let some = true;
    match some{
        true => println!("true"),
        false => println!("false"),
    }

    // use underscore

    let num = 10;
    match num {
        10 => println!("is 10"),
        _ => println!("not equal to 10"),
    }
}


fn program(a: i32) -> (){
    if a>10 {
        println!("10보다 큽니다");
    }else {
        println!("10보다 작거나 같습니다");
    }
}

