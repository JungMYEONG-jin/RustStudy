pub fn printNumberWithLoop(a: i32) {
    let mut temp = a;
    loop {
        println!("{:?}", temp);
        temp = temp - 1;
        if temp == 0 {
            println!("done");
            break;
        }
    }
}