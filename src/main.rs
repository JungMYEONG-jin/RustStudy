mod a1;
mod a2;
mod a3;
mod a3b;
mod a5;
mod a6;
mod a7;
mod a8;
mod bills;

use bills::*;

extern crate core;
extern crate alloc;


use core::borrow::{Borrow, BorrowMut};
use rand::Rng;
use std::io;
use std::collections::{BTreeMap, HashMap};
use crate::a7::Color;
use crate::a8::{Drink, Flavor};


fn getInput() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){ // try until success
        println!("Please retry...");
    }
    // trim remove \n
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    }else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64>{
    println!("Enter Amount:");
    loop{
        let input: String = match getInput() {
            Some(val) => val,
            None => "".to_string(),
        };
        if &input == "" { // 다시 입력할 기회를 주기 위해 &
            return None;
        }
        let result: Result<f64, _> = input.parse();
        // _ 오류 무시
        match result {
            Ok(amount) => return Some(amount),
            Err(_) => println!("please type number"),
        }
    }
}

fn add_bill_menu(bills: &mut Bills){
    let name = match getInput() {
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount() {
        Some(input) => input,
        None => return,
    };
    let bill = Bill{ name, amount };
    bills.add(bill);
    println!("Bill Added");
}

fn view_bill_menu(bills: &Bills){
    for bill in bills.getAll(){
        println!("{:?}", bill);
    }
}

fn remove_bill_menu(bills: &mut Bills){
    for bill in bills.getAll(){
        println!("{:?}", bill)
    }

    println!("type name to delete");

    let input = match getInput(){
        Some(input) => input,
        None => return,
    };
    if bills.remove(&input) {
        println!("removed");
    }else {
        println!("The bill not existed");
    }
}


fn edit_bill_menu(bills: &mut Bills){
    for bill in bills.getAll(){
        println!("{:?}", bill)
    }

    println!("type name to edit");
    let input = match getInput(){
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount(){
        Some(amount) => amount,
        None => return,
    };
    if bills.edit(&input, amount){
        println!("updated");
    }else {
        println!("The bill not existed");
    }
}

fn mainMenu(){
    fn show(){
        println!("");
        println!("Mange Bills");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("");
        println!("Enter Selection:");
    }

    let mut bills = Bills::new();
    loop{
        show();
        let input = match getInput(){
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bill_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => edit_bill_menu(&mut bills),
            _ => break,
        }
    }
}

fn maybe_num() -> Option<i32> {

}

fn maybe_word() -> Option<String> {

}



fn main() {
    // closer
    let plus = match maybe_num() {
        Some(num) => Some(num+1),
        None => None,
    };
    // 위 코드를 아래와 같이 map + 클로저로 해결 가능.
    // map option이 None일때는 실행안됨.
    let plus = maybe_num().map(|num| num+1);
    let wordLenTwice = maybe_word().map(|word| word.len()).map(|len| 2*len);
}

