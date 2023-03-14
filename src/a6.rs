pub fn print5To1() {
    let mut five = 5;
    while five>0{
        println!("{:?}", five);
        five = five - 1;
    }
    println!("Done!");
}