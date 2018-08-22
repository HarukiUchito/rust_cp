#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

// https://tdpc.contest.atcoder.jp/tasks/tdpc_dice

fn main() {
    let cin = stdin(); let cin = cin.lock(); let mut sc = Scanner::new(cin);
    
    let (n, mut d): (usize, i64) = (sc.read(), sc.read());
    
    let (mut I, mut J, mut K) = (0, 0, 0);
    while d % 2 == 0 {
        I += 1; d /= 2;
    }
    while d % 3 == 0 {
        J += 1; d /= 3;
    }
    while d % 5 == 0 {
        K += 1; d /= 5;
    }
    if d != 1 {
        println!("0"); return;
    }

    let N = n + 1;
    let I = I + 1; let J = J + 1; let K = K + 1;
    let mut dp: Vec<Vec<Vec<Vec<f64>>>> = vec![vec![vec![vec![0.0; K]; J]; I]; N];
    dp[0][0][0][0] = 1.0;
    for n in 0..(N-1) {
        for i in 0..I {
            for j in 0..J {
                for k in 0..K {
                    let num = dp[n][i][j][k] / 6.0;
                    dp[n + 1][i][j][k] += num;
                    dp[n + 1][i][j][min(k + 1, K-1)] += num;//if (k+1) < K { dp[n + 1][i][j][k + 1] += num; }
                    dp[n + 1][i][min(j + 1, J-1)][k] += num;//if (j+1) < J { dp[n + 1][i][j + 1][k] += num; }
                    dp[n + 1][min(i + 1, I-1)][j][k] += num;
                    dp[n + 1][min(i + 2, I-1)][j][k] += num;
                    dp[n + 1][min(i + 1, I-1)][min(j + 1, J-1)][k] += num;
                }
            }
        }
    }
    println!("{}", dp[N-1][I-1][J-1][K-1]);
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