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
use std::collections::{BTreeMap, HashMap};
use crate::a7::Color;
use crate::a8::{Drink, Flavor};

struct Book{
    pages: i32,
    rating: i32,
}

fn main() {
    let book = Book{
        pages: 3,
        rating: 1,
    };

    displayCount(&book);
    displayRating(&book);
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