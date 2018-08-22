#![allow(unused_imports, unused_variables, dead_code)]
use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;
use std::ops::*;
use std::num::*;

// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1132

fn main() {
    let cin = stdin(); let cin = cin.lock(); let mut sc = Scanner::new(cin);

    let mut n: usize;
    loop {
        n = sc.read();
        if n == 0 { break; }
        
        let mut ps: Vec<Point<f64>> = Vec::new();
        for i in 0..n {
            ps.push(Point{ x: sc.read(), y: sc.read() });
        }

        let mut ans = 1;
        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }
                let (p1, p2) = (ps[i], ps[j]);
                let mid: Point<f64> = (p1 + p2) / 2.0;
                let d2 = dist2(&p1, &p2);
                let d = d2.sqrt();
                if d > add_eps(2.0) { continue; }
                let theta = (d / 2.0).asin();
                let l = (1.0 - d2 / 4.0).sqrt();
                let (dx, dy) = (p1.x - p2.x, p1.y - p2.y);
                let diff = Point{ x: -l * dy / d, y: l * dx / d};
                let (c1, c2) = (mid + diff, mid - diff);
                
                let mut cnt1 = 2;
                let mut cnt2 = 2;
                for k in 0..n {
                    if (k == i) | (k == j) { continue; }
                    if dist2(&ps[k], &c1) < add_eps(1.0) { cnt1 += 1; }
                    if dist2(&ps[k], &c2) < add_eps(1.0) { cnt2 += 1; }
                }
                ans = max(ans, cnt1); ans = max(ans, cnt2);
            }
        }
        println!("{}", ans);
    }
}

// 

fn add_eps(num: f64) -> f64 {
    num + 1e-6
}

fn dist2(p1: &Point<f64>, p2: &Point<f64>) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    dx*dx + dy*dy
}

#[derive(Debug, Copy, Clone, Eq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add<Point<T>> for Point<T> where T: Add<Output=T> {
    type Output = Point<T>;
    
    fn add(self, other: Point<T>) -> Point<T> {
        Point{ x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T> Sub<Point<T>> for Point<T> where T: Sub<Output=T> {
    type Output = Point<T>;
    
    fn sub(self, other: Point<T>) -> Point<T> {
        Point{ x: self.x - other.x, y: self.y - other.y }
    }
}

impl<T> Div<T> for Point<T> where T: Div<Output=T> + Copy {
    type Output = Point<T>;
    
    fn div(self, divisor: T) -> Point<T> {
        Point{ x: self.x / divisor, y: self.y / divisor }
    }
}

impl<T> Ord for Point<T> where T: Ord {
    fn cmp(&self, other: &Point<T>) -> Ordering {
        if self.x == other.x { return self.y.cmp(&other.y); }
        else { return self.x.cmp(&other.x); }
    }
}

impl<T> PartialOrd<Point<T>> for Point<T> where T: PartialOrd {
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        if self.x == other.x { return self.y.partial_cmp(&other.y); }
        else { return self.x.partial_cmp(&other.x); }
    }
}

impl<T> PartialEq<Point<T>> for Point<T> where T: PartialEq {
    fn eq(&self, other: &Point<T>) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }
}


// input
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