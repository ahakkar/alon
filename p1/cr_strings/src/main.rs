#![allow(unused_parens)]
#![allow(unused_variables)]
use std::io;
use std::collections::HashSet;

/**
 * aabac
 */
fn main() {
    // gather input from user
    let mut input = String::new();  
    io::stdin().read_line(&mut input).unwrap_or_default();
    input = String::from(input.trim());    
    let mut chars: Vec<char> = input.chars().collect();

    // Store only unique permutations
    let mut results: HashSet<Vec<char>> = HashSet::new();
    let size:usize = chars.len();

    permute(&mut chars, size, &mut results);

    // Sort results to lexiographical order
    let mut sorted_results: Vec<Vec<char>> = results.into_iter().collect();
    sorted_results.sort();

    // print result size and each result
    println!("{}", sorted_results.len());
    for result in &sorted_results {
        print_formatted(result);
    }
}

/**
 * Calculate *all* permutations of string with Heap's algorithm
 * https://en.wikipedia.org/wiki/Heap%27s_algorithm
 */
fn permute(chars: &mut [char], size:usize, results: &mut HashSet<Vec<char>>) {
    if(size == 1) {
        results.insert(chars.to_vec());
        return;
    }

    for i in 0..size {
        permute(chars, size-1, results);

        if (size % 2 == 0) {
            chars.swap(i, size-1);
        } else {
            chars.swap(0, size-1);
        }        
    }
}

fn print_formatted(chars: &[char]) {
    let formatted_string:String = chars.iter().collect();

    println!("{}", formatted_string);
}