#![allow(unused_imports, unused_variables, dead_code, non_snake_case, unused_macros)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
fn getline() -> String {
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).ok();
    res
}
macro_rules ! readl { ( $ t : ty ) => { { let s = getline ( ) ; s . trim ( ) . parse ::<$ t > ( ) . unwrap ( ) } } ; ( $ ( $ t : ty ) ,+ ) => { { let s = getline ( ) ; let mut iter = s . trim ( ) . split ( ' ' ) ; ( $ ( iter . next ( ) . unwrap ( ) . parse ::<$ t > ( ) . unwrap ( ) , ) * ) } } ; }
macro_rules ! readlvec { ( $ t : ty ) => { { let s = getline ( ) ; let iter = s . trim ( ) . split ( ' ' ) ; iter . map ( | x | x . parse ( ) . unwrap ( ) ) . collect ::< Vec <$ t >> ( ) } } }
macro_rules ! mvec { ( $ v : expr , $ s : expr ) => { vec ! [ $ v ; $ s ] } ; ( $ v : expr , $ s : expr , $ ( $ t : expr ) ,* ) => { vec ! [ mvec ! ( $ v , $ ( $ t ) ,* ) ; $ s ] } ; }
macro_rules ! debug { ( $ x : expr ) => { println ! ( "{}: {:?}" , stringify ! ( $ x ) , $ x ) } }
fn printiter<'a, T>(v: &'a T)
where
    &'a T: std::iter::IntoIterator,
    <&'a T as std::iter::IntoIterator>::Item: std::fmt::Display,
{
    for (i, e) in v.into_iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", e);
    }
    println!("");
}
struct ContestPrinter {
    s: String,
}
impl ContestPrinter {
    fn new() -> ContestPrinter {
        ContestPrinter { s: String::new() }
    }
    fn print<T>(&mut self, x: T)
    where
        T: std::fmt::Display,
    {
        self.s.push_str(format!("{}", x).as_str());
    }
    fn println<T>(&mut self, x: T)
    where
        T: std::fmt::Display,
    {
        self.s.push_str(format!("{}\n", x).as_str());
    }
}
impl std::ops::Drop for ContestPrinter {
    fn drop(&mut self) {
        print!("{}", self.s);
    }
}
static MOD: i64 = 1e9 as i64 + 7;
fn is_max_i64(num: i64) -> bool {
    if num == i64::max_value() {
        true
    } else {
        false
    }
}

fn main() {
    let mut pr = ContestPrinter::new();
   
   
}