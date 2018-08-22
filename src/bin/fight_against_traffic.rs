#![allow(unused_imports, unused_variables, dead_code, non_snake_case, unused_macros)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::iter::FromIterator;
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

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited: Vec<i32> = vec![-1; graph.len()];
    let mut prev: Vec<usize> = vec![0; graph.len()];
    prev[start] = usize::max_value();
    let mut q: VecDeque<usize> = VecDeque::new();
    visited[start] = 0;
    q.push_back(start);
    while !q.is_empty() {
        let &c_idx = q.front().unwrap();
        let c_cost = visited[c_idx];
        for n_idx in graph[c_idx].iter() {
            let c = visited[*n_idx];
            if c < 0 {
                let n_cost = c_cost + 1;
                visited[*n_idx] = n_cost;
                prev[*n_idx] = c_idx;
                q.push_back(*n_idx);
            }
        }
        q.pop_front();
    }
    prev
}

// graph[i][j] graph[i]が頂点iに繋がっている頂点番号を格納 
fn bfs_distance_list(graph: &Vec<Vec<usize>>, start: usize) -> Vec<i32> {
    let mut d: Vec<i32> = vec![-1; graph.len()];
    d[start] = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(start);
    while !q.is_empty() {
        let &c_idx = q.front().unwrap();
        let c_cost = d[c_idx];
        for n_idx in graph[c_idx].iter() {
            let c = d[*n_idx];
            if c < 0 {
                let n_cost = c_cost + 1;
                d[*n_idx] = n_cost;
                q.push_back(*n_idx);
            }
        }
        q.pop_front();
    }
    d
}

// graph[i][j] 頂点i,j間に辺があるか
fn bfs_distance_matrix(graph: &Vec<Vec<bool>>, start: usize) -> Vec<i32> {
    let mut d: Vec<i32> = vec![-1; graph.len()];
    d[start] = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(start);
    while !q.is_empty() {
        let &c_idx = q.front().unwrap();
        let c_cost = d[c_idx];
        for (idx, connected) in graph[c_idx].iter().enumerate() {
            if !*connected { continue; }
            if d[idx] < 0 {
                let n_cost = c_cost + 1;
                d[idx] = n_cost;
                q.push_back(idx);
            }
        }
        q.pop_front();
    }
    d
}

fn get_path(prev: Vec<usize>, goal: usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut c = goal;
    while c != usize::max_value() {
        path.push(c);
        c = prev[c];
    }
    path
}

fn main() {
    let mut pr = ContestPrinter::new();
    let (N, M, S, T) = readl!(usize, usize, usize, usize);
    let mut g: Vec<Vec<bool>> = vec![vec![false; N]; N];
    for i in 0..M {
        let (a, b) = readl!(usize, usize);
        g[a-1][b-1] = true;
        g[b-1][a-1] = true;
    }

    let from_s = bfs_distance_matrix(&g, S-1);
    let from_t = bfs_distance_matrix(&g, T-1);
    let D = from_s[T-1];
    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            if i == j { continue; }
            if g[i][j] { continue; } 
            if (from_s[i] + 1 + from_t[j]) < D { continue; }
            if (from_s[j] + 1 + from_t[i]) < D { continue; }
            ans += 1;
        }
    }
    
    pr.println(ans / 2);
}