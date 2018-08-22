#![allow(unused_imports, unused_variables, dead_code)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

// https://beta.atcoder.jp/contests/arc065/tasks/arc065_b

// 各ノードが接続しているノードを2次元配列で返す
fn node_to_list(node_num: usize, link_num: usize, sc: &mut Scanner) -> Vec<Vec<usize>> {
    let mut ret: Vec<Vec<usize>> = vec![Vec::new(); node_num];
    for i in 0..link_num {
        let (p, q): (usize, usize) = (sc.read(), sc.read());
        ret[p-1].push(q-1); ret[q-1].push(p-1);
    }
    ret
}

fn dfs(
    paths: &Vec<Vec<usize>>,
    cnct: &mut Vec<i32>,
    p: usize,
    num: i32
) {
    cnct[p] = num;
    for next in paths[p].iter() {
        if cnct[*next] == -1 {
            dfs(paths, cnct, *next, num);
        }
    }
}

fn main() {
    let cin = stdin(); let cin = cin.lock(); let mut sc = Scanner::new(cin);

    let (n, k, l): (usize, usize, usize) = (sc.read(), sc.read(), sc.read());
    let pqs: Vec<Vec<usize>> = node_to_list(n, k, &mut sc);
    let rss: Vec<Vec<usize>> = node_to_list(n, l, &mut sc);
    
    let mut c1: Vec<i32> = vec![-1; n];
    let mut cnt = 0;
    for i in 0..n {
        if c1[i] == -1 {
            cnt += 1;
            dfs(&pqs, &mut c1, i, cnt);
        }
    }
    cnt = 0;
    let mut c2: Vec<i32> = vec![-1; n];
    for i in 0..n {
        if c2[i] == -1 {
            cnt += 1;
            dfs(&rss, &mut c2, i, cnt);
        }
    }

    let mut cncts = HashMap::new();
    for i in 0..n {
        let num = cncts.entry((c1[i], c2[i])).or_insert(0);
        *num += 1;
    }
    for i in 0..n {
        if i < n-1 { print!("{} ", cncts.get(&(c1[i], c2[i])).unwrap()); }
        else { println!("{}", cncts.get(&(c1[i], c2[i])).unwrap()); }
    }
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