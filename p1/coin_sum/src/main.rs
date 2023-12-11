#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(clippy::let_and_return)]
use std::io;

fn main() {
    let data = read_input();
    println!("{}", process(data));
}

fn read_input() -> Vec<usize> {
    let mut input = String::new();  
    // gather input from user    
    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<i64>().unwrap(); 
    input.clear();

    // parse input like "5 2" to a movie with start and end time
    io::stdin().read_line(&mut input).unwrap();
    let coins: Vec<usize> = input
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    
    coins
}

/*
 * https://en.wikipedia.org/wiki/Coin_problem
 */
fn process(mut data: Vec<usize>) -> usize {
    let mut current_sum = 0;
    data.sort();
    
    for coin in data {
        if coin > current_sum + 1 {
            return current_sum + 1;
        }
        current_sum += coin;
    }
    current_sum + 1
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_process() {
        let test_data: Vec<usize> = vec![2, 9, 1, 2, 7];
        assert_eq!(process(test_data), 6);
    }

    #[test]
    fn test_process2() {
        let test_data: Vec<usize> = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728, 268435456, 536870912, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000];
        assert_eq!(process(test_data), 31073741824
    );
    }
}
