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

// euclidの互除法
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a }
    else { gcd(b, a % b) }
}
// ab = lcm(a, b)gcd(a, b)
fn lcm(a: i64, b: i64) -> i64 {
    let g = gcd(a, b);
    (a / g) * b
}
// 約数列挙 昇順ではない
fn divisors(n: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    if n == 1 { ret.push(1); return ret; }
    let mut i = 1;
    loop {
        if (n % i) == 0 {
            ret.push(i);
            if (i != 1) & ((i*i) != n) {
                ret.push(n / i);
            }
        }
        i += 1; if n < (i*i) { break; }
    }
    ret.push(n);
    ret
}

fn is_max_i64(num: i64) -> bool { if num == i64::max_value() { true } else { false } }

// 2部グラフかどうか，片方の色の数，もう片方の色の数
fn is_bipartite(graph: &Vec<Vec<usize>>) -> (bool, usize, usize) {
    let size = graph.len();
    let mut color: Vec<i32> = vec![0; size];
    fn dfs(g: &Vec<Vec<usize>>, color: &mut Vec<i32>, v: usize, c: i32) -> bool {
        color[v] = c;
        for i in 0..g[v].len() {
            let nb = g[v][i];
            let nbc = color[nb];
            if nbc == c { return false; }
            if nbc == 0 { if !dfs(g, color, nb, -c) { return false; } }
        }
        return true;
    }
    for i in 0..size {
        if color[i] == 0 { if !dfs(graph, &mut color, i, 1) { return (false, 0, 0); } }
    }
    
    let (mut c1, mut c2) = (0, 0);
    for c in color { match c {
        1 => c1 += 1,
        _ => c2 += 1,
    } }
    (true, c1, c2)
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind{ parent: (0..size).collect(), rank: vec![0; size] }
    }

    fn find_root(&mut self, index: usize) -> usize {
        let p = self.parent[index];
        if p == index {
            return index;
        } else {
            self.parent[index] = self.find_root(p);
            return self.parent[index];
        }
    }

    // xとyの属する木を併合
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find_root(x);
        let y = self.find_root(y);
        if x != y {
            if self.rank[x] < self.rank[y] {
                self.parent[x] = y;
            } else {
                self.parent[y] = x;
                if self.rank[x] == self.rank[y] { self.rank[x] += 1; }
            }
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }
}

// 無向グラフ　最小全域木
struct MinimumSpanningTree {
    pub node_num: usize,
    pub edges: Vec<(i64, usize, usize)>, // cost, node_idx, node_idx
}

impl MinimumSpanningTree {
    fn new(size: usize) -> MinimumSpanningTree {
        MinimumSpanningTree{ node_num: size, edges: Vec::new() }
    }

    // O(|E|log|V|)
    fn calc(&mut self) -> i64 {
        self.edges.sort();
        let mut uf = UnionFind::new(self.node_num);
        let mut sum = 0;
        for i in 0..self.edges.len() {
            let (cost, u, v) = self.edges[i];
            if uf.same(u, v) { continue; }
            uf.unite(u, v);
            sum += cost;
        }
        sum
    }
}

fn main() {
    let mut pr = ContestPrinter::new();
    
    let N = readl!(usize);
    let mut psx: Vec<(i64, i64, usize)> = Vec::new();
    let mut psy: Vec<(i64, i64, usize)> = Vec::new();

    for i in 0..N {
        let (x, y) = readl!(i64, i64);
        psx.push((x, y, i));
        psy.push((y, x, i));
    }

    psx.sort();
    psy.sort();

    let mut mst = MinimumSpanningTree::new(N);
    let mut before = psx[0];
    for i in 1..N {
        let cost = min((psx[i].0 - before.0).abs(), (psx[i].1 - before.1).abs());
        mst.edges.push((cost, before.2, psx[i].2));
        before = psx[i];
    }
    let mut before = psy[0];
    for i in 1..N {
        let cost = min((psy[i].0 - before.0).abs(), (psy[i].1 - before.1).abs());
        mst.edges.push((cost, before.2, psy[i].2));
        before = psy[i];
    }

    pr.println(mst.calc());
}