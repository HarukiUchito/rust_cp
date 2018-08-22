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

static MOD: i64 = 1e9 as i64 + 7;

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

// W x H の左上から左下にいく組み合わせの計算
// return (W+H)!, (W!H!), ans 
fn wh_comb(W: i64, H: i64) -> (i64, i64, i64) {
    let W = W - 1; let H = H - 1;

    let mut c = 1;
    for i in 1..W+H+1 { c *= i; c %= MOD; }
    
    let mut m = 1;
    for i in 1..W+1 { m *= i; m %= MOD; }
    for i in 1..H+1 {
        m *= i;
        m %= MOD;
    }
 
    let mom = ab_mod_p(m, MOD-2, MOD);
    (c, m, (c*mom) % MOD)
}

fn main() {
    let mut printer = ContestPrinter::new();

    let (H, W, A, B) = readl!(i64, i64, i64, i64);

    let mut up: Vec<i64> = vec![0; (W - B) as usize];
    
    let LH = H - A;
    let B1 = B + 1;
    let (mut child, mut mom, a) = wh_comb(B1, LH);
    up[0] = a;
    for i in 1..W-B {
        child *= LH - 1 + B1 + i - 1; child %= MOD;
        mom *= B1 + i - 1; mom %= MOD;
        up[i as usize] = (child * ab_mod_p(mom, MOD-2, MOD)) % MOD;
    }

    let LW = W - B;
    let (mut child, mut mom, a) = wh_comb(1, A);
    let mut ans = a * up[LW as usize - 1];
    for i in 1..LW {
        child *= A - 1 + i; child %= MOD;
        mom *= i; mom %= MOD;
        let a = (child * ab_mod_p(mom, MOD-2, MOD)) % MOD;
        ans += up[LW as usize - 1 - i as usize] * a; ans %= MOD;
    }

    printer.println(ans);
}