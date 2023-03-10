fn main() {
    println!("Hello, world!");
    let mut vec = vec![1, 2, 3, 4,5 ];
    let iter = vec.iter();
    for v in iter{
        println!("number is {}", v);
    }

    let mut two = 2;
    two = two + 1;

}
