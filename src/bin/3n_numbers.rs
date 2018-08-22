#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

// https://beta.atcoder.jp/contests/abc062/tasks/arc074_b

fn main() {
    let cin = stdin(); let cin = cin.lock(); let mut sc = Scanner::new(cin);

    let mut hp: BinaryHeap<i64> = BinaryHeap::new();
    let n: usize = sc.read();
    let mut input = vec![0; n];
    let mut cnt: i64 = 0;
    for i in 0..n {
        let num: i64 = sc.read();
        hp.push(-num);
        cnt += num;
    }
    
    let mut half1 = vec![cnt; n + 1];
    for i in 1..(n+1) {
        let next: i64 = sc.read();
        hp.push(-next);
        let out = hp.pop().unwrap();
        half1[i] = cnt + next + out;
        cnt = half1[i];
        input[i-1] = next;
    }
    
    let mut hp: BinaryHeap<i64> = BinaryHeap::new();
    let mut cnt: i64 = 0;
    for i in 0..n {
        let num: i64 = sc.read();
        hp.push(num);
        cnt += num;
    }

    let mut half2 = vec![cnt; n + 1];
    for i in 1..(n+1) {
        let next: i64 = input[n-1 - (i-1)];
        hp.push(next);
        let out = hp.pop().unwrap();
        half2[n-i] = cnt + next - out;
        cnt = half2[n-i];
    }

    let mut ans = i64::min_value();
    for k in 0..(n+1) { ans = max(ans, half1[k] - half2[k]); }
    println!("{}", ans);
}

struct Scanner<'a> {
    cin: StdinLock<'a>,
}
impl<'a> Scanner<'a> {
    fn new(cin: StdinLock<'a>) -> Scanner<'a> {
        Scanner { cin: cin }
    }

    fn read1<T: FromStr>(&mut self) -> Option<T> {
        let token = self.cin.by_ref().bytes().map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        token.parse::<T>().ok()
    }

    fn read<T: FromStr>(&mut self) -> T {
        self.read1().unwrap()
    }
}