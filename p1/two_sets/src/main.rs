#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::io;

fn main() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap_or_default();  
    let n = input.trim().parse::<i64>().unwrap_or_default();
    let vec: Vec<i64> = (1..=n).rev().collect(); 
    let sum = n * (n+1) / 2;

    if (sum % 2 == 0) {
        println!("YES");
        divide_to_two_sets(&vec);
    } 
    else {
        println!("NO");
    }
}

/*
 * Divide to two sets based on "tirakirja.pdf's" NP hard problem section
 * "Heuristiikat", page 170. 
 * 
 * Which said: put the numbers from largest to smallest, to the set with
 * a smaller current sum. 
 */
fn divide_to_two_sets(vec: &[i64]) {
    let mut s1: Vec<i64> = vec![];
    let mut s2: Vec<i64> = vec![];

    let mut sum1:i64 = 0;
    let mut sum2:i64 = 0;

    for &number in vec {
        if(sum1 > sum2) {
            s2.push(number);
            sum2 += number;
        } else {
            s1.push(number);
            sum1 += number;
        }

    }

    print_set(&s1);
    print_set(&s2);
}


/**
 * Print the vector formatted to exercise's specification (only spaces between
 * nums)
 */
fn print_set(vec: &[i64]) {
    let formatted_string = vec.iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", vec.len());    
    println!("{}", formatted_string);
}

