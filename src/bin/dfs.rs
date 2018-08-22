#![allow(unused_imports, unused_variables, dead_code)]
use std::io::*;
use std::fmt::*;
use std::str::*;
use std::cmp::*;
use std::collections::*;

fn bound_check(x: i32, y: i32, width: i32, height: i32) -> bool {
    if x < 0 || y < 0 || x >= width || y >= height {
        return false;
    } else {
        return true;
    }
}

struct Pos<T> {
    x: T,
    y: T,
}

static DX: &'static [i32] = &[0, 1, 0, -1];
static DY: &'static [i32] = &[-1, 0, 1, 0];

fn dfs(field: &mut Vec<Vec<char>>, p: Pos<i32>) -> bool {
    field[p.y as usize][p.x as usize] = 's';
    for i in 0..4 as usize {
        let nx = p.x + DX[i];
        let ny = p.y + DY[i];
        if !bound_check(nx, ny, field[0].len() as i32, field.len() as i32) {
            continue;
        }
        match field[ny as usize][nx as usize] {
            'g' => return true,
            's' | '#' => continue,
            _ => if dfs(field, Pos { x: nx, y: ny }) {
                return true;
            },
        }
    }
    return false;
}

fn main() {
    let (height, width): (usize, usize) = read();
    let mut field: Vec<Vec<char>> = Vec::new();
    let mut start = Pos { x: 0, y: 0 };
    for i in 0..height {
        let line: String = read();
        field.push(line.chars().collect::<Vec<char>>());
        for (j, c) in field[i].iter().enumerate() {
            if *c == 's' {
                start.x = j as i32;
                start.y = i as i32;
            }
        }
    }

    if dfs(&mut field, start) {
        println!("Yes");
    } else {
        println!("No");
    }
}


pub trait InputValue {
    fn parse(s: &str) -> Self;
}

pub fn read<T: InputValue>() -> T {
    let mut buf = String::new();
    let _ = stdin().read_line(&mut buf);
    T::parse(&buf.trim())
}

pub fn readnc<T: InputValue>() -> Vec<T> {
    let mut vec = vec![];
    let line: String = read();
    for token in line.split_whitespace() {
        vec.push(T::parse(token));
    }
    vec
}

pub fn readn<T: InputValue>(n: usize) -> Vec<T> {
    let mut vec = vec![];
    for _ in 0..n {
        vec.push(read());
    }
    vec
}

macro_rules! parse_single_value {
    ($($t:ty),*) => {
        $(
            impl InputValue for $t {
                fn parse(s: &str) -> $t { s.parse().unwrap() }
            }
        )*
    }
}
parse_single_value!(i32, i64, f32, f64, usize, String);

macro_rules! parse_tuple {
    ($($t:ident),*) => {
        impl<$($t),*> InputValue for ($($t),*) where $($t: InputValue),* {
            fn parse(s: &str) -> ($($t),*) {
                let mut tokens = s.split_whitespace();
                let t = ($($t::parse(tokens.next().unwrap())),*);
                t
            }
        }
    }
}
parse_tuple!(A, B);
parse_tuple!(A, B, C);