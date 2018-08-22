#![allow(unused_imports, unused_variables, dead_code)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

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

// https://beta.atcoder.jp/contests/joi2008ho/tasks/joi2008ho_b

// s1, s2を左揃えで並べた時の最長連続文字列の長さ
fn common_num(s1: &Vec<char>, s2: &Vec<char>) -> i32 {
    let num = min(s1.len(), s2.len());
    let mut ret: i32 = 0;
    let mut cnt: i32 = 0;
    let mut before: bool = false;
    for i in 0..num {
        if s1[i] == s2[i] {
            if before { cnt += 1; }
            else { cnt = 1; }
        } else {
            ret = max(ret, cnt);
            cnt = 0;
        }
        before = s1[i] == s2[i];
    }
    max(ret, cnt)
}

fn main() {
    let cin = stdin(); let cin = cin.lock(); let mut sc = Scanner::new(cin);

    let s1: String = sc.read(); let n1 = s1.len();
    let s2: String = sc.read(); let n2 = s2.len();
    let mut ans: i32 = 0;
    for i in 0..n1 {
        ans = max(ans,
            common_num(
                &(&s1[i..n1].chars().collect::<Vec<char>>()), 
                &(&s2[0..n2].chars().collect::<Vec<char>>())
            )
        );
    }
    for i in 0..n2 {
        ans = max(ans,
            common_num(
                &(&s2[i..n2].chars().collect::<Vec<char>>()), 
                &(&s1[0..n1].chars().collect::<Vec<char>>())
            )
        );
    }

    println!("{}", ans);
}

