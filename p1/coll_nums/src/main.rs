#![allow(unused_parens)]
#![allow(unused_variables)]
use std::io;
use std::collections::HashMap;

/**
 * 4 2 1 5 3
 */
fn main() {
    let mut input = String::new();  

    // size of vector
    io::stdin().read_line(&mut input).unwrap_or_default();
    let n = input.trim().parse::<i64>().unwrap_or_default();
    
    // data for vector
    io::stdin().read_line(&mut input).unwrap_or_default();    
    let numbers: Vec<i64> = input.split_whitespace()
        .map(|num| num.parse().unwrap_or_default())
        .collect();

    // Collect values & indexes to a map for faster access
    let map: HashMap<i64, usize> = numbers.into_iter().enumerate()
        .map(|(index, value)| (value, index))
        .collect();

    //println!("{:?}", map);

    let mut rounds:i64 = 0;
    let mut collected:i64 = 1;

    // Use the map to collect numbers one by one
    while (collected <= n) {
        let current_index = *map.get(&collected).unwrap();    
        collected += 1;
    
        // In the end there's no next index to comapre
        if (collected > n) {
            rounds += 1;
            break;
        }
    
        // Add a new round when needed
        let next_index = *map.get(&collected).unwrap();    
        if (current_index > next_index) {
            rounds += 1;
        }
    }

    println!("{}", rounds);
}
