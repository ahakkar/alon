#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::io;

/**
 * 2 = ei, summa 3 
 * 3 = [1,2] [3] = 3, summa 6
 * 4 = [1,4] [2,3] = 5, summa 10
 * 5 = ei, summa 15
 * 6 = ei, summa 21
 * 7 = [2,4,6] [1,3,5,7] = 14, summa 28
 * 8 = [1,2,4,5,6] [3,7,8] = 18, summa 36
 * 9 = ei
 * 10 = ei
 * 11 = kyllä (33), summa 64
 */
fn main() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap_or_default();  
    let n = input.trim().parse::<i64>().unwrap_or_default();
    let vec: Vec<i64> = (1..=n).collect(); 
    let sum = n * (n+1) / 2;

    if (sum % 2 == 0) {
        println!("YES");
        println!("{}", sum);
        print_result(&vec);
    } 
    else {
        println!("NO");
        println!("{}", sum);
    }
}

fn print_result(vec: &Vec<i64>) {
    let mut s1: Vec<i64> = vec![];
    let mut s2: Vec<i64> = vec![];

    let mut add: bool = false;

    for &number in vec {
        if add {
            s1.push(number); 
            add = false;
        } else {
            s2.push(number);
            add = true;
        }
    }

    println!("s1: {:?}, sum: {}", s1, s1.iter().sum::<i64>());
    println!("s2: {:?}, sum: {}", s2, s2.iter().sum::<i64>());
}

