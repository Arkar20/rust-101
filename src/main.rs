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

    let mut my_tuple: (u8, String, f64) = (47, "hello world".to_string(), 111.9);

    println!("Id is {}", my_tuple.0);
    println!("Name is {}", my_tuple.1);
    println!("Number is {}", my_tuple.2);

    let mut text = String::new();
    text.push('a');
    text.push_str("rkar");

    println!("Stiring Name is {}", text);

    let new_text = text.replace("a", "jeffery");
    println!("new name is {}", new_text);

    let predefine_string = String::from("a r k a r");
    let mut v1: Vec<char> = predefine_string.chars().collect();

    v1.sort();
    v1.dedup();

    for value in v1 {
        println!("vector {}", value);
    }
    let str4 = "Hello";
    let mut str5 = str4.to_string();

    println!("{}", str5);
    let byte_string = str4.as_bytes();

    //slice
    let slice_string = &str4[0..2];
    println!("Slice String {}", slice_string);

    let myn_name= String::from("arkar");
    let eng_name= String::from("jeffery");

    let full_name= myn_name + &eng_name;
    println!("Full name is {}",full_name);
    //casting
    let num_1:i16=12;
    let num_2:i8=16;

    let sum:i32= (num_1 as i32) + (num_2 as i32);

    println!("sum is {}",sum);

    //enum
    enum Days{
        MONDAY,
        TUESDAY,
        WEDNESSDAY,
    }

    impl  Days  {
        fn is_monday(&self)->bool{
            match self {
                Days::MONDAY => true,
                _ => false,

            }
        }
    }

    let today=Days::MONDAY;

    println!("Today is monday {}", today.is_monday());

    //vector
    //vectors are mutable arrays

    let mut nums:Vec<i32> = Vec::new();

    nums.push(19);

    let mut nums2:Vec<i32> = vec![1,2,3,4];
    nums2.push(188);
   
    for  value in &mut nums2 {
        *value*=2;
        println!("value is {}",value);
        
    }

    println!("value in first index of vector {}",nums2[0]);
    println!("value in first index of vector by fun {:?}",nums2.get(1)); // :? indicates he return value of fn
    println!("Length :{}",nums2.len()); 


}
