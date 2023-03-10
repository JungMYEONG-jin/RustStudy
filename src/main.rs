extern crate core;
fn main() {
    println!("Hello, world!");
    let mut vec = vec![1, 2, 3, 4,5 ];
    let iter = vec.iter();
    for v in iter{
        println!("number is {}", v);
    }

    let mut two = 2;
    two = two + 1;

    let a = 3;
    let b = 4;
    let c = add(a, b);
    let str = "hello world";
    println!("{:?} {:?}",str, c);


    program(b);
    program(11);

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

