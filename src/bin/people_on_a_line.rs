#![allow(unused_imports, unused_variables, dead_code)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

// https://beta.atcoder.jp/contests/arc090/tasks/arc090_b

fn bfs(
    cmap: &Vec<Vec<(usize, i64)>>
) -> bool {
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut visited: [bool; 100005] = [false; 100005];
    let mut node: [i64; 100005] = [0; 100005];
    for n in 0..100000 {
        if visited[n] { continue; }
        visited[n] = true;
        node[n] = 0;
        q.push_back(n);
        while !q.is_empty() {
            let &c = q.front().unwrap();
            for &(r, d) in &cmap[c] {
                let (c_to_i, to) = (d, r);//if c == l { (d, r) } else { (-d, l) };
                if visited[to] { // 既に決めた場所なのでチェック
                    if node[c] + c_to_i == node[to] { continue; } //OK
                    else { return false; }
                }
                // きたことないのでNodeの値を決める
                node[to] = node[c] + c_to_i;
                visited[to] = true;
                q.push_back(to);
            }
            q.pop_front();
        }
    }
    return true;
}

fn main() {
    let cin = stdin(); let cin = cin.lock(); let mut sc = Scanner::new(cin);

    let (n, m): (usize, usize) = (sc.read(), sc.read());
    let mut lrd: Vec<Vec<(usize, i64)>> = vec![Vec::new(); 100000];

    for i in 0..m {
        let (l, r, d): (usize, usize, i64) = (sc.read(), sc.read(), sc.read());
        let l = l - 1; let r = r - 1;
        lrd[l].push((r, d));
        lrd[r].push((l, -d));
    }

    if !bfs(&lrd) { println!("No"); }
    else { println!("Yes"); }
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