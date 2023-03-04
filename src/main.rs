#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
   let mut my_age =202;

   let is_my_age:bool= if my_age==233 {
       true
   }else{
    false
   };
   println!("is my age {}",is_my_age);

   match my_age {
       1..=100 => println!("is between 1 and 100"),
       10|20=> println!("OR"),
       _ => println!("all equal to default")
   }
let voting_age=18;

 match my_age.cmp(&voting_age) {
    Ordering::Less => println!("CANNOT VOTE"),
    Ordering::Greater=>println!("CAN VOTE"),
    Ordering::Equal=>println!("GAIN THE RIGHT TO VOTE")

 }
}
