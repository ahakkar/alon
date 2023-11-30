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

    if (sum == 0) {
        println!("YES");
        println!("{}", sum);
    } 
    else {
        println!("NO");
        println!("{}", sum);
    }
}

fn print_result(vec: &[i64]) {

}

