#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(clippy::let_and_return)]
#![allow(dead_code)]
use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    // Gather input
    let mut input = String::new();      
    io::stdin().read_line(&mut input).unwrap();

    let (amt_tickets, amt_customers) = input
        .trim()
        .split_once(' ')
        .map(|(s, e)| (s.parse::<u32>().unwrap(), e.parse::<u32>().unwrap()))
        .unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let mut tickets: Vec<i32> = input
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let bids: Vec<i32> = input
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    input.clear();
    // Sort tickets for easier binary search    
    tickets.sort_unstable();
    solve_prices(amt_tickets, amt_customers, tickets, bids);
}

fn solve_prices(
    amt_tickets: u32,
    amt_customers: u32,
    tickets: Vec<i32>,
    bids: Vec<i32>
) {   
    let mut vq: VecDeque<_> = VecDeque::from(tickets);
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut w = io::BufWriter::new(lock);

    
    for bid in bids {
        let res = vq.binary_search(&bid);

        match res {
            Ok(index) => {
                writeln!(w, "{}", vq[index]).unwrap();
                if vq[0] == vq[index] {
                    vq.pop_front();
                } else {
                    vq.remove(index);
                }
            },
            Err(index) => {
                if index > 0 && index < vq.len() -1 {
                    writeln!(w, "{}", vq[index-1]).unwrap();
                    if vq[0] == vq[index] {
                        vq.pop_front();
                    } else {
                        vq.remove(index-1);
                    }
                }
                else if !vq.is_empty() {
                    if vq[0] < bid {
                        writeln!(w, "{}", vq[0]).unwrap();
                        vq.pop_front();      
                    }
                    else {
                        writeln!(w, "-1").unwrap();
                    }
                } else {
                    writeln!(w, "-1").unwrap();
                }
            },
        } 
    } 
    w.flush().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_solve() {
        let amt_tickets: u32 = 5;
        let amt_customers: u32 = 3;
        let mut tickets: Vec<i32> = vec![5, 3, 7, 8, 5];
        let bids: Vec<i32> = vec![4, 8, 3];          
        tickets.sort_unstable();
        solve_prices(amt_tickets, amt_customers, tickets, bids);
    }

    #[test]
    fn test_solve2() {
        let amt_tickets: u32 = 3;
        let amt_customers: u32 = 4;
        let mut tickets: Vec<i32> = vec![2, 2, 2];
        let bids: Vec<i32> = vec![4,4,4,4]; 
        tickets.sort_unstable();          
        solve_prices(amt_tickets, amt_customers, tickets, bids);
    }

    #[test]
    fn test_solve3() {
        let amt_tickets: u32 = 1;
        let amt_customers: u32 = 2;
        let mut tickets: Vec<i32> = vec![1];
        let bids: Vec<i32> = vec![10,10];    
        tickets.sort_unstable();       
        solve_prices(amt_tickets, amt_customers, tickets, bids);
    }

    #[test]
    fn test_solve4() {
        let amt_tickets: u32 = 10;
        let amt_customers: u32 = 10;
        let mut tickets: Vec<i32> = vec![9, 3, 9, 6, 6, 8, 6, 2, 6, 3];
        let bids: Vec<i32> = vec![9, 5, 4, 6, 3, 9, 3, 3, 5, 2];     
        tickets.sort_unstable();      
        solve_prices(amt_tickets, amt_customers, tickets, bids);
    }

    #[test]
    fn test_solve5() {
        let amt_tickets: u32 = 200000;
        let amt_customers: u32 = 200000;
        let mut tickets: Vec<i32> = vec![1; (amt_tickets-1).try_into().unwrap()];
        tickets.push(2);
        let bids: Vec<i32> = vec![1; amt_customers.try_into().unwrap()];
        tickets.sort_unstable();      
        solve_prices(amt_tickets, amt_customers, tickets, bids);
    }
}
