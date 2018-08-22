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

struct BellmanFord {
    pub from: Vec<usize>,
    pub to: Vec<usize>,
    pub cost: Vec<i64>,
    pub start: usize,
    pub node_num: usize,
}

impl BellmanFord {
    // (負閉路があったか，startから各Nodeへの最短距離)
    fn get_shortest_path(&self) -> (bool, Vec<i64>) {
        assert!(self.from.len() == self.to.len(), "invalid input vec length");
        assert!(self.cost.len() == self.to.len(), "invalid input vec length");
        assert!(self.start < self.from.len(), "invalid start node");

        let mut distance = vec![i64::max_value(); self.node_num];
        distance[self.start] = 0;
        let edge_num = self.from.len();
        let mut found_negative = false;
        for n in 0..self.node_num {
            let mut update = false;
            for i in 0..edge_num {
                let (f, t, c) = (self.from[i], self.to[i], self.cost[i]);
                if distance[f] == i64::max_value() { continue; }
                let new_cost = distance[f] + c;
                if distance[t] > new_cost {
                    distance[t] = new_cost;
                    update = true;
                    if n == (self.node_num - 1) { found_negative = true; }
                }
            }
            if !update { break; }
        }
        (found_negative, distance)
    }
}

fn main() {
    let mut printer = ContestPrinter::new();
    let cin = stdin(); let cin = cin.lock(); let mut sc = Scanner::new(cin);
    
    let (V, E, R): (usize, usize, usize) = (sc.read(), sc.read(), sc.read());
    let mut bf = BellmanFord{
        from: Vec::new(), to: Vec::new(), cost: Vec::new(),
        start: R, node_num: V
    };
    for i in 0..E {
        let (s, t, d): (usize, usize, i64) = (sc.read(), sc.read(), sc.read());
        bf.from.push(s);
        bf.to.push(t);
        bf.cost.push(d);
    }

    if E == 0 { printer.println(0); return; }

    let (found, distance) = bf.get_shortest_path();
    if found { printer.println("NEGATIVE CYCLE"); }
    else {
        for (idx, d) in distance.iter().enumerate() {
            if *d == i64::max_value() {
                printer.println("INF");
            } else {
                printer.println(d);
            }
        }
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