#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    if x < 0 || y < 0 || x >= width || y >= height {
        return false;
    } else {
        return true;
    }
}

// ===

static DX: &'static [i32] = &[0, 1, 0, -1];
static DY: &'static [i32] = &[-1, 0, 1, 0];

// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B&lang=jp

// ナップザック問題　index以降のものを価値weight以下でとって最大の物を返す
fn search(
    ws: &Vec<i32>, vs: &Vec<i32>, dp: &mut Vec<Vec<Option<i32>>>,
    index: usize, weight: usize
) -> i32 {
    if index == ws.len() { return 0; }
    else if let Some(v) = dp[index][weight] { return v; }
    else if ws[index] > weight as i32 { return search(ws, vs, dp, index + 1, weight); }
    else {
        let a1 = search(ws, vs, dp, index + 1, weight);
        let a2 = search(ws, vs, dp, index + 1, weight - ws[index] as usize) + vs[index];
        let ret = max(a1, a2);
        dp[index][weight] = Some(ret);
        return ret;
    }
}

fn main() {
    let (n, w): (usize, usize) = read();
    let mut vs: Vec<i32> = vec![0; n];
    let mut ws: Vec<i32> = vec![0; n];
    for i in 0..n { 
        let (v, w) = read();
        vs[i] = v; ws[i] = w;
    }

    // dpテーブル　Noneで初期化
    let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; 10010]; 110];

    println!("{}", search(&ws, &vs, &mut dp, 0, w));
}


pub trait InputValue {
    fn parse(s: &str) -> Self;
}

pub fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

pub fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

pub fn readn<T: InputValue>(n: usize) -> Vec<T> {
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec
}

macro_rules! parse_single_value {
    ($($t:ty),*) => {
        $(
            impl InputValue for $t {
                fn parse(s: &str) -> $t { s.parse().unwrap() }
            }
        )*
    }
}
parse_single_value!(i32, i64, f32, f64, usize, String);

macro_rules! parse_tuple {
    ($($t:ident),*) => {
        impl<$($t),*> InputValue for ($($t),*) where $($t: InputValue),* {
            fn parse(s: &str) -> ($($t),*) {
                let mut tokens = s.split_whitespace();
                let t = ($($t::parse(tokens.next().unwrap())),*);
                t
            }
        }
    }
}
parse_tuple!(A, B);
parse_tuple!(A, B, C);