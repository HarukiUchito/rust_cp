#![allow(unused_imports, unused_variables, dead_code, non_snake_case, unused_macros)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
fn getline() -> String {
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).ok();
    res
}
macro_rules ! readl { ( $ t : ty ) => { { let s = getline ( ) ; s . trim ( ) . parse ::<$ t > ( ) . unwrap ( ) } } ; ( $ ( $ t : ty ) ,+ ) => { { let s = getline ( ) ; let mut iter = s . trim ( ) . split ( ' ' ) ; ( $ ( iter . next ( ) . unwrap ( ) . parse ::<$ t > ( ) . unwrap ( ) , ) * ) } } ; }
macro_rules ! readlvec { ( $ t : ty ) => { { let s = getline ( ) ; let iter = s . trim ( ) . split ( ' ' ) ; iter . map ( | x | x . parse ( ) . unwrap ( ) ) . collect ::< Vec <$ t >> ( ) } } }
macro_rules ! mvec { ( $ v : expr , $ s : expr ) => { vec ! [ $ v ; $ s ] } ; ( $ v : expr , $ s : expr , $ ( $ t : expr ) ,* ) => { vec ! [ mvec ! ( $ v , $ ( $ t ) ,* ) ; $ s ] } ; }
macro_rules ! debug { ( $ x : expr ) => { println ! ( "{}: {:?}" , stringify ! ( $ x ) , $ x ) } }
fn printiter<'a, T>(v: &'a T)
where
    &'a T: std::iter::IntoIterator,
    <&'a T as std::iter::IntoIterator>::Item: std::fmt::Display,
{
    for (i, e) in v.into_iter().enumerate() {
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
        ContestPrinter { s: String::new() }
    }
    fn print<T>(&mut self, x: T)
    where
        T: std::fmt::Display,
    {
        self.s.push_str(format!("{}", x).as_str());
    }
    fn println<T>(&mut self, x: T)
    where
        T: std::fmt::Display,
    {
        self.s.push_str(format!("{}\n", x).as_str());
    }
}
impl std::ops::Drop for ContestPrinter {
    fn drop(&mut self) {
        print!("{}", self.s);
    }
}
static MOD: i64 = 1e9 as i64 + 7;
fn is_max_i64(num: i64) -> bool {
    if num == i64::max_value() {
        true
    } else {
        false
    }
}

fn bfs(graph: &Vec<Vec<usize>>, start: Vec<usize>, d: usize) -> i32 {
    let mut visited: Vec<i32> = vec![-1; graph.len()];
    let mut q: VecDeque<usize> = VecDeque::new();
    for s in start.iter() {
        visited[*s] = 0;
        q.push_back(*s);
    }
    let mut cnt = 0;
    while !q.is_empty() {
        let &c_idx = q.front().unwrap();
        let c_cost = visited[c_idx];
        for n_idx in graph[c_idx].iter() {
            let c = visited[*n_idx];
            if c < 0 {
                let n_cost = c_cost + 1;
                if n_cost > d as i32 { cnt += 1; }
                visited[*n_idx] = n_cost;
                q.push_back(*n_idx);
            }
        }
        q.pop_front();
    }
    cnt
}

fn main() {
    let mut pr = ContestPrinter::new();
    let (N, K) = readl!(usize, usize);
    let mut ab = Vec::new();
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); N];
    for i in 0..N-1 {
        let (a, b) = readl!(usize, usize);
        g[a-1].push(b-1);
        g[b-1].push(a-1);
        ab.push((a-1, b-1));
    }

    let mut ans = i32::max_value();
    if (K % 2) == 0 {
        let k2 = K / 2;
        for i in 0..N { // center node
            ans = min(ans, bfs(&g, vec![i], k2));
        }
    } else {
        let k12 = (K - 1) / 2;
        for i in 0..ab.len() {; // center edge
            let (a, b) = ab[i];
            ans = min(ans, bfs(&g, vec![a, b], k12));
        }
    }

    pr.println(ans);
}