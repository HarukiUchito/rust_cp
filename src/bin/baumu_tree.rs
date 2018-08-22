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

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    pub edge_node_num: Vec<(usize, usize)>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind{ parent: (0..size).collect(), rank: vec![0; size], edge_node_num: vec![(0, 1); size] }
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
                self.edge_node_num[y].0 += self.edge_node_num[x].0 + 1;
                self.edge_node_num[y].1 += self.edge_node_num[x].1 + 0;    
            } else {
                self.parent[y] = x;
                self.edge_node_num[x].0 += self.edge_node_num[y].0 + 1;
                self.edge_node_num[x].1 += self.edge_node_num[y].1 + 0;
                if self.rank[x] == self.rank[y] { self.rank[x] += 1; }
            }
        } else {
            self.edge_node_num[x].0 += 1;
            self.edge_node_num[x].1 += 0;
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }
}

fn main() {
    let mut printer = ContestPrinter::new();
    
    let (N, M) = readl!(usize, usize);
    let mut uf = UnionFind::new(N);
    for m in 0..M {
        let (a, b) = readl!(usize, usize);
        uf.unite(a-1, b-1);
    }
    let mut hs = HashSet::new();
    for n in 0..N {
        let root = uf.find_root(n);
        let (edge_num, node_num) = uf.edge_node_num[root];
        if node_num > edge_num {
            hs.insert(root);
        }
    }
    printer.println(hs.len());
}