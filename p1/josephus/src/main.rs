#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(clippy::let_and_return)]
#![allow(dead_code)]
use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    let n = read_input();
    josephus(n, 2);
}

fn read_input() -> i32 {
    let mut input = String::new();  
    // gather input from user    
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<i32>().unwrap()
}

fn josephus(mut n:i32, mut k:i32) {
    let mut people: VecDeque<i32> = (1..n + 1).collect();
    let mut idx = 0;

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut w = io::BufWriter::new(lock);

    while !people.is_empty() {
        idx = (idx + k - 1) % people.len() as i32;
        write!(w, "{} ", people.remove(idx as usize).unwrap()).unwrap();
    }
    w.flush().unwrap();
}

/*
 * https://en.wikipedia.org/wiki/Josephus_problem
 */
fn josephus_slow(mut n:usize, mut k:usize) -> String {
    let mut people: Vec<usize> = (1..n + 1).collect();
    let mut idx = 0;
    let mut sequence: Vec<usize> = vec![];

    while !people.is_empty() {
        idx = (idx + k - 1) % people.len();
        // Probably WAY TOO SLOW.. remove runs O(n)
        sequence.push(people.remove(idx));
    }
    
    sequence
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

/* #[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_process() {
        assert_eq!(josephus(7, 2), String::from("2 4 6 1 5 3 7"));
    }
}
 */