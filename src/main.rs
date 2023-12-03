mod aoc1;
mod aoc2;
pub mod aoc3;

use std::io;
use std::env;
use std::process::exit;

use aoc1::AOC1;

use crate::aoc2::AOC2;
use crate::aoc3::AOC3;




fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let mut input: Vec<String> = Vec::new();
    for line in stdin.lines() {
        input.push(match line {
            Ok(val) => val,
            Err(_) => {
                println!("Error reading StdIn");
                exit(1);
            }
        })
    }
    
    if args.len() < 3 {
        println!("Specify the problem 1 - 25");
        exit(0);
    }
    match args[2].as_str() {
        "1" => {
            println!("{}", AOC1::new().solve(input));
        }
        "2" => {
            println!("{}", AOC2::new().solve(input));
        }
        "3" => {
            // Its going to be ASCII characters
            // Convert to bytes for simplicty and performance. 
            // Padded for simplicity!!!
            let mut aoc3_inputs: Vec<Vec<u8>> = Vec::with_capacity(input.len() + 2);
            for line in &input {
                let line_byes = line.clone().into_bytes();
                let mut line_vector: Vec<u8> = Vec::with_capacity(line.len() + 2);
                // . - 46
                line_vector.push(46);
                line_vector.extend(line_byes);
                line_vector.push(46);
                aoc3_inputs.push(line_vector);
            }
            println!("{}", AOC3::new().solve(&mut aoc3_inputs));
        }
        _ => {
            println!("Args should be 1 - 25");
        }
    }
}


