// 요구사항
// create new

#[derive(Debug)]
pub enum Color{
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
pub struct Cloth(Color);
impl Cloth{
    pub fn new(color: Color) -> Self{
        Self(color)
    }
}

#[derive(Debug)]
pub struct Shoes(Color);
impl Shoes{
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
pub struct Pants(Color);
impl Pants{
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

pub fn display_cloth(cloth: Cloth) {
    println!("Cloth clolor {:?}", cloth.0);
}

pub fn display_shoes(shoe: Shoes) {
    println!("Shoes clolor {:?}", shoe.0);
}

pub fn display_pants(pant: Pants) {
    println!("Pants clolor {:?}", pant.0);
}