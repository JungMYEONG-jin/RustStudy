mod a1;
mod a2;
mod a3;
mod a3b;
mod a5;
mod a6;
mod a7;
mod a8;
extern crate core;

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

struct Book{
    pages: i32,
    rating: i32,
}

fn main() {
    let mut buffer = String::new();
    let result = io::stdin().read_line(&mut buffer);
    if result.is_ok() {
        match Power::new(&buffer) {
            Some(state) => checkPowerState(state),
            None => println!("invalid input"),
        }
    } else {
        println!("error occurred while reading");
    };


}

fn displayCount(book: &Book) {
    println!("{:?}", book.pages);
}

fn displayRating(book: &Book) {
    println!("{:?}", book.rating);
}

// 메모리 추적
// rust는 소유권 모형을 이용해 사용한다.
// 이동을 안하게 만드려면 대여를 해야한다. & ampersand 사용
// 데이터는 소유권자만 삭제 가능.

enum Power{
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl Power {
    fn new(state: &str) -> Option<Power>{
        let lower = state.trim().to_lowercase();
        // as str String을 빌려와서 &str 처럼 만들어
        match lower.as_str() {
            "off" => Some(Power::Off),
            "sleep" => Some(Power::Sleep),
            "reboot" => Some(Power::Reboot),
            "shutdown" => Some(Power::Shutdown),
            "hibernate" => Some(Power::Hibernate),
            _ => None,
        }
    }
}

fn checkPowerState(power: Power) {
    use Power::*;
    match power {
        Off => println!("Power Off!"),
        Sleep => println!("Power Sleep!"),
        Reboot => println!("Reboot start!"),
        Shutdown => println!("Shutdown start!"),
        Hibernate => println!("Hibernate!!"),
        _ => println!("you typed wrong message..."),
    }
}