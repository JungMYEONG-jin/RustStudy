pub enum Flavor{
    SOFT,
    HARD
}

pub struct Drink{
    pub flavor: Flavor,
    pub ounce: f64,
}

pub fn printDrinkInfo(drink: Drink){
    match drink.flavor{
        Flavor::SOFT => println!("SOFT flavor"),
        Flavor::HARD => println!("HARD flavor"),
    }
    println!("ounce {:?}", drink.ounce);
}

