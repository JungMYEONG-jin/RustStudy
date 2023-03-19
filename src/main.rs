mod a1;
mod a2;
mod a3;
mod a3b;
mod a5;
mod a6;
mod a7;
mod a8;
extern crate core;
extern crate alloc;


use core::borrow::{Borrow, BorrowMut};
use rand::Rng;
use std::io;
use std::collections::{BTreeMap, HashMap};
use crate::a7::Color;
use crate::a8::{Drink, Flavor};


fn getInput() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?; // ? 의 의미는 실패할수도 있따는걸 알림
    Ok(buffer.trim().to_owned()) // 소유권 다시 가져오기
}


struct Bill{
    name: String,
    priceInfo: PriceInfo,
}

struct PriceInfo{
    price: i32,
    prevPrice: i32,
}

impl PriceInfo{
    fn changePrice(&mut self){
        let mut temp = self.price;
        self.price = self.prevPrice;
        self.prevPrice = temp;
    }

    fn setPrice(&mut self, price: i32){
        self.prevPrice = self.price;
        self.price = price;
    }
}

fn addBills(bill: Bill, map:&mut HashMap<String, PriceInfo>){
    map.insert(bill.name, bill.priceInfo);
}

fn removeBills(str: &str, map:&mut HashMap<String, PriceInfo>){
    if map.contains_key(str) {
        map.remove(str);
    }else {
        println!("Not Existed on bills");
    }
}

fn main() {
    let mut map: HashMap<String, PriceInfo> = HashMap::new();
    let mut buffer = String::new();
    let mut buffer2 = String::new();
    println!("Add bill");
    io::stdin().read_line(&mut buffer);
    io::stdin().read_line(&mut buffer2);

    // lv1 add bill
        let bill = Bill{
            name: buffer.trim().to_owned(),
            priceInfo: PriceInfo{
                price: buffer2.trim().to_owned().parse::<i32>().unwrap(),
                prevPrice: 0,
            },
        };
        addBills(bill, &mut map);

    buffer.clear();
    buffer2.clear();
    // lv1 view bills
    for (name, price) in &map{
        println!("name {:?}, price {:?}, prevPrice {:?}", name, price.price, price.prevPrice);
    }

    // lv2 remove bill

    let result2 = io::stdin().read_line(&mut buffer);
    if result2.is_ok() {
        removeBills(buffer.trim().to_owned().as_str(), &mut map);
    }
    for (name, price) in &map{
        println!("name {:?}, price {:?}, prevPrice {:?}", name, price.price, price.prevPrice);
    }

    buffer.clear();
    buffer2.clear();
    // edit
    println!("type name and price that you want to edit");
    io::stdin().read_line(&mut buffer);
    io::stdin().read_line(&mut buffer2);
    let price = buffer2.trim().to_owned().parse::<i32>().unwrap();
    let temp = map.get(buffer.trim().to_owned().as_str()).unwrap();
    let mut info = PriceInfo{
        price: price,
        prevPrice: temp.price
    };

    map.insert(buffer.trim().to_owned(), info);
    for (name, price) in &map{
        println!("name {:?}, price {:?}, prevPrice {:?}", name, price.price, price.prevPrice);
    }

}

