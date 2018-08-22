#![allow(unused_imports, unused_variables, dead_code, non_snake_case, unused_macros)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

fn getline() -> String{
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).ok();
    res
}

macro_rules! readl {
    ($t: ty) => {
        {
            let s = getline();
            s.trim().parse::<$t>().unwrap()
        }
    };
    ($( $t: ty),+ ) => {
        {
            let s = getline();
            let mut iter = s.trim().split(' ');
            ($(iter.next().unwrap().parse::<$t>().unwrap(),)*) 
        }
    };
}

macro_rules! readlvec {
    ($t: ty) => {
        {
            let s = getline();
            let iter = s.trim().split(' ');
            iter.map(|x| x.parse().unwrap()).collect::<Vec<$t>>()
        }
    }
}

macro_rules! mvec {
    ($v: expr, $s: expr) => {
        vec![$v; $s]
    };
    ($v: expr, $s: expr, $($t: expr),*) => {
        vec![mvec!($v, $($t),*); $s]
    };
}

macro_rules! debug {
    ($x: expr) => {
        println!("{}: {:?}", stringify!($x), $x)
    }
}

fn printiter<'a, T>(v: &'a T)
where
    &'a T: std::iter::IntoIterator, 
    <&'a T as std::iter::IntoIterator>::Item: std::fmt::Display {
    for (i,e) in v.into_iter().enumerate() {
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
        ContestPrinter {
        s: String::new(),
        }
    }

    fn print<T>(&mut self, x: T)
        where T: std::fmt::Display {
        self.s.push_str(format!("{}", x).as_str());
    }

    fn println<T>(&mut self, x: T)
        where T: std::fmt::Display {
        self.s.push_str(format!("{}\n", x).as_str());
    }
}

impl std::ops::Drop for ContestPrinter {
    fn drop(&mut self) {
        print!("{}", self.s);
    }
}

fn is_max_i64(num: i64) -> bool { if num == i64::max_value() { true } else { false } }

fn ab_mod_p(a: i64, b: i64, p: i64) -> i64 {
    if b == 0 { 1 }
    else if (b % 2) == 0 {
        let d = ab_mod_p(a, b / 2, p);
        (d * d) % p
    } else {
        (a * ab_mod_p(a, b - 1, p)) % p
    }
}

fn main() {
    let mut printer = ContestPrinter::new();

    let MOD: i64 = 1e9 as i64 + 7;
    
    let (H, W, A, B) = readl!(i64, i64, i64, i64);

    let mut ans: i64 = 0;

    let mut up: Vec<i64> = vec![0; (W - B) as usize];

    let mut HA = 1;
    for i in 1..H-A+1 {
        HA *= i; 
        HA %= MOD; 
    }

    let mut WHA = HA;
    HA = ab_mod_p(HA, MOD-2, MOD);
    let mut wi = 1;
    for i in 1..B+1 {
        wi *= i; wi %= MOD;
        WHA *= H - A + i; WHA %= MOD;
    }
    for i in B+1..W+1 {
        WHA *= H - A + i; WHA %= MOD;
        wi *= i; wi %= MOD;
        let mut add = WHA * HA; add %= MOD;
        up[i as usize - B as usize - 1] = (add * ab_mod_p(wi, MOD-2, MOD)) % MOD;
    }

    let mut AA = 1;
    for i in 1..A+1 { AA *= i; AA %= MOD; }
    
    let mut AI = AA;
    AA = ab_mod_p(AA, MOD-2, MOD);
    let mut II = 1;
    for i in 1..W-B {
        AI *= A + i; AI %= MOD;
        II *= i;  II %= MOD;
        let mut add = AI * AA; add %= MOD;
        ans += (add * ab_mod_p(II, MOD-2, MOD)) % MOD;
    }

    printer.println(ans);
}