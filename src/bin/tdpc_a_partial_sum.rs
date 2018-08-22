#![allow(unused_imports, unused_variables, dead_code)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

// https://tdpc.contest.atcoder.jp/tasks/tdpc_contest

fn main() {
    let cin = stdin(); let cin = cin.lock(); let mut sc = Scanner::new(cin);

    let n: usize = sc.read();
    let mut ps: Vec<i32> = vec![];
    for i in 0..n { ps.push(sc.read()); }

    let size = 10010;
    let mut dp = vec![vec![false; size]; 110];
    dp[0][0] = true;
    let mut ans: i32 = 0;

    for i in 1..n + 1 {
        let p = ps[i-1] as usize;
        for j in 0..size {
            if j < p { dp[i][j] = dp[i-1][j]; }
            else { dp[i][j] = dp[i-1][j] | dp[i-1][j - p]; }
            if i == n && dp[i][j] { ans += 1; }
        }
    }

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