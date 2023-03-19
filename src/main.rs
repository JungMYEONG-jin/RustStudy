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


fn getInput() -> String {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){ // try until success
        println!("Please retry...");
    }
    // trim remove \n
    buffer.trim().to_owned()
}

fn get_bill_amount() -> f64{
    println!("Enter Amount:");
    loop{
        let input = getInput();
        let result: Result<f64, _> = input.parse();
        // _ 오류 무시
        match result {
            Ok(amount) => return amount,
            Err(_) => println!("please type number"),
        }
    }
}

fn add_bill_menu(bills: &mut Bills){
    let name = getInput();
    let amount = get_bill_amount();
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

    let input = getInput();
    if bills.remove(&input) {
        println!("removed");
    }else {
        println!("The bill not existed");
    }
}


fn edit_bill_menu(bills: &mut Bills){

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
        let input = getInput();
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bill_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => edit_bill_menu(&mut bills),
            "5" => roll_back(&mut bills),
            _ => break,
        }
    }
}


fn main() {
    mainMenu();
}

