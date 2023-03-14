use rand::Rng;


pub fn getPos() -> (i32, i32){
    let mut rng = rand::thread_rng();
    (rng.gen::<i32>(), rng.gen::<i32>())
}