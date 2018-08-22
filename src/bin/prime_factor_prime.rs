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

// 指定した数までの素数のvec エラトステネスの篩
fn prime_list(n: usize) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    let num = n - 1;
    let mut table: Vec<bool> = vec![false; num];
    for i in 0..num {
        if !table[i] {
            let next = i + 2;
            ret.push(next);
            let mut j = i + next;
            while j < num {
                table[j] = true;
                j += next;
            }
        }
    }
    ret
} // [2, 3, 5, 7, ..., n]

// 素因数分解
fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let pl = prime_list((n as f64).sqrt() as usize + 1);
    let mut num = n;
    for p in pl { // 各素数で割れるだけ割る
        let mut cnt = 0;
        while (num % p) == 0 {
            num /= p;
            cnt += 1;
        }
        if cnt > 0 { ret.push((p, cnt)); }
    }
    if ret.is_empty() { vec![(n, 1); 1] }
    else { ret }
} // n = 2*2*2 + 3 -> [(2, 3), (3, 1)]

fn is_prime(n: i64) -> bool {
    if n == 1 { return false; }
    if n == 2 { return true; }
    let mut i = 2;
    loop {
        if (n % i) == 0 {return false; }
        i += 1;
        if (i*i) > n { break; }
    }
    return true;
}

// オイラーのφ関数 nと互いに素（gcdが1）な数の個数が取れる
fn euler_phi_vec(n: usize) -> Vec<u64> {
    let mut f: Vec<u64> = vec![0; n];
    let mut p: Vec<u64> = vec![1; n];
    for i in 0..n { f[i] = i as u64; }
    for i in 2..n {
        if p[i] != 0 {
            f[i] -= f[i] / i as u64;
            let mut j = i + i;
            loop {
                if j >= n { break; }
                p[j] = 0;
                f[j] -= f[j] / i as u64;
                j += i;
            }
        }
    }
    f
}

fn is_max_i64(num: i64) -> bool { if num == i64::max_value() { true } else { false } }

fn main() {
    let mut pr = ContestPrinter::new();
    
    let (L, R) = readl!(usize, usize);
    //let pl = prime_list((R as f64).sqrt() as usize);
    let pl = prime_list(40101);
    let size = R - L + 1;
    let mut pfp_cnt = vec![0; size];

    let mut nums: Vec<usize> = (L..R+1).collect();
    for p in pl {
        let mut num = (L + p - 1) / p * p;
        loop {
            if num > R { break; }
            let mut lnum = nums[num - L];
            let mut cnt = 0;
            while (lnum % p) == 0 { lnum /= p; cnt += 1; }
            pfp_cnt[num - L] += cnt;
            nums[num - L] = lnum;
            num += p;
        }
    }

    for i in 0..size { if 1 < nums[i] { pfp_cnt[i] += 1; } }

    let mut ans = 0;
    for pfp in pfp_cnt {
        if is_prime(pfp as i64) {
            ans += 1;
        }
    }
    pr.println(ans);
}