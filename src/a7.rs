pub enum Color{
    RED,
    GREEN,
    BLUE
}

pub fn printColor(color: Color) {
    match color {
        Color::RED => println!("RED"),
        Color::GREEN => println!("GREEN"),
        Color::BLUE => println!("BLUE")
    }
}