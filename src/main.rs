#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("Name");
    let mut name:String=String::new();
    let greeting: &str="Hello world";
    

    io::stdin().read_line(&mut name).expect("Error");
    println!("Hello, {}! {}",name.trim_end(),greeting);
}
