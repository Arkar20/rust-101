#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
   const PI:f32 =3.14;
   const PI_WHOLE:u32 =3;
 
   //shadowing
   let age="232";

   let mut age:u32=age.trim().parse().expect("msg");

   age=age+1;

   println!("{}",age);
}
