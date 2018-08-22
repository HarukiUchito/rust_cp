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

// 負の辺があると使えない
// edges[from]のベクタの要素それぞれが,from->toのcostを格納
struct Dijkstra {
    pub edges: Vec<Vec<(usize, i64)>>,
    pub start: usize,
}

impl Dijkstra {
    fn get_shortest_path(&self) -> Vec<i64> {
        let node_num = self.edges.len();
        assert!(self.start < node_num, "invalid start node");

        let mut pq: BinaryHeap<(i64, usize)> = BinaryHeap::new(); //最短距離，Node番号のペア
        let mut distance = vec![i64::max_value(); node_num];

        pq.push((0, self.start));
        distance[self.start] = 0;

        while !pq.is_empty() {
            let (min_distance, idx) = pq.pop().unwrap();
            if distance[idx] < -min_distance { continue; }
            for i in 0..self.edges[idx].len() { //idxからのそれぞれの辺
                let (to, cost) = self.edges[idx][i];
                let new_cost = distance[idx] + cost;
                if distance[to] > new_cost {
                    distance[to] = new_cost;
                    pq.push((-new_cost, to));
                }
            }
        }

        distance
    }
}

fn main() {
    let mut printer = ContestPrinter::new();
    
    let (V, E, R) = readl!(usize, usize, usize);
    let mut dj = Dijkstra {
        edges: vec![Vec::new(); V],
        start: R
    };

    for i in 0..E {
        let (s, t, d) = readl!(usize, usize, i64);
        dj.edges[s].push((t, d));
    }

    let distance = dj.get_shortest_path();
    for (idx, d) in distance.iter().enumerate() {
        if *d == i64::max_value() {
            printer.println("INF");
        } else {
            printer.println(d);
        }
    }
}