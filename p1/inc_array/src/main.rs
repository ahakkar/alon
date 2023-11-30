#![allow(unused_parens)]

use std::io;

fn main() {       
    let mut input = String::new();   

    // discard size of vector
    io::stdin().read_line(&mut input).unwrap_or_default();
    input.clear();
    
    // actual data for vector
    io::stdin().read_line(&mut input).unwrap_or_default();    
    let numbers: Vec<i64> = input.split_whitespace()
        .map(|num| num.parse().unwrap_or_default())
        .collect();
    //println!("{:?}", numbers);

    let mut sum:i64 = 0;
    let mut prev_number = numbers[0];

    // add to the sum and remember prrevious number for comparison
    for &number in &numbers[1..] {
        if (prev_number > number) {
            sum += prev_number - number;
        } else {
            prev_number = number;
        }
    }

    println!("{}", sum);
}