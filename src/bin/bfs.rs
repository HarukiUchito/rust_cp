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

// ===

static DX: &'static [i32] = &[0, 1, 0, -1];
static DY: &'static [i32] = &[-1, 0, 1, 0];

fn bfs(field: &Vec<Vec<char>>, (sx, sy): (i32, i32), i_num: i32) -> bool {
    let mut visited: Vec<Vec<bool>> = vec![ vec![false; field[0].len()]; field.len() ];
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    visited[sy as usize][sx as usize] = true;
    q.push_back((sx, sy));
    let mut cnt = 1;
    while !q.is_empty() {
        let &(cx, cy) = q.front().unwrap();
        for i in 0..4 as usize {
            let (nx, ny) = (cx + DX[i], cy + DY[i]);
            if !bound_check(nx, ny, field[0].len() as i32, field.len() as i32) { continue; }
            if field[ny as usize][nx as usize] == 'x' { continue; }
            if visited[ny as usize][nx as usize] { continue; }
            q.push_back((nx, ny));
            visited[ny as usize][nx as usize] = true;
            cnt += 1;
        }
        q.pop_front();
    }
    return cnt == i_num
}

fn main() {
    let size = 10;
    let (height, width): (usize, usize) = (size, size);
    let mut field: Vec<Vec<char>> = Vec::new();
    let mut i_num: i32 = 0;
    for i in 0..height {
        let line: String = read();
        field.push(line.chars().collect::<Vec<char>>());
        for c in field[i].iter() {
            if *c == 'o' { i_num += 1; }
        }
    }

    let mut once: bool = true;
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            let mut end: bool = false;
            match field[i][j] {
                'o' if once => {
                    if bfs(&field, (j as i32, i as i32), i_num) { end = true; }
                    once = false;
                },
                'o' => continue,
                _ => {
                    field[i][j] = 'o';
                    if bfs(&field, (j as i32, i as i32), i_num + 1) { end = true; }
                    field[i][j] = 'x';
                },
            }

            if end { println!("YES"); return; }
        }
    }

    println!("NO");
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