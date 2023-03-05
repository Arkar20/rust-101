#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{array, io};

fn main() {
    let mut my_age = 202;

    let is_my_age: bool = if my_age == 233 { true } else { false };
    println!("is my age {}", is_my_age);

    match my_age {
        1..=100 => println!("is between 1 and 100"),
        10 | 20 => println!("OR"),
        _ => println!("all equal to default"),
    }
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("CANNOT VOTE"),
        Ordering::Greater => println!("CAN VOTE"),
        Ordering::Equal => println!("GAIN THE RIGHT TO VOTE"),
    }
    let mut array_1 = [1, 2, 3, 4];
    let mut index = 0;
    // loop {
    //     if (array_1.len() < index + 1) {
    //         break;
    //     }
    //     println!("looping through array {}", array_1[index]);
    //     index += 1;
    // }

    // while array_1.len() > index + 1 {
    //     println!("while looping through array {}", array_1[index]);
    //     index+=1;
    // }
    for val in array_1 {
        
        println!("for  looping through array {}", val);
    }

    let mut my_tuple:(u8,String,f64)= (47,"hello world".to_string(),111.9);

    println!("Id is {}", my_tuple.0);
    println!("Name is {}", my_tuple.1);
    println!("Number is {}", my_tuple.2);
}
