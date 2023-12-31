mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;
mod aoc10;

use std::io;
use std::env;
use std::process::exit;

use crate::aoc1::AOC1;
use crate::aoc10::AOC10;
use crate::aoc2::AOC2;
use crate::aoc3::AOC3;
use crate::aoc4::AOC4;
use crate::aoc5::AOC5;
use crate::aoc6::AOC6;
use crate::aoc7::AOC7;
use crate::aoc8::AOC8;
use crate::aoc9::AOC9;


fn pad_inputs(lines: Vec<String>, filler: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(lines.len());
    let filler_line = filler.repeat(lines[0].len() + 2);
    res.push(filler_line.clone());
    for line in lines {
        res.push(filler.clone() + &line + &filler);
    }
    res.push(filler_line);
    res 
}

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
        "4" => {
            println!("{}", AOC4::new().solve(input));
        }
        "5-1" => {
            println!("{}", AOC5::new().solve(input));
        }
        "5-2" => {
            println!("{}", AOC5::new().solve_p2(input));
        }
        "6-1" => {
            println!("{}", AOC6::new().solve_p1(input));
        }
        "6-2" => {
            println!("{}", AOC6::new().solve_p2(input));
        }
        "7-1" => {
            println!("{}", AOC7::new().solve_p1(input));
        }
        "7-2" => {
            println!("{}", AOC7::new().solve_p2(input));
        }
        "8-1" => {
            println!("{}", AOC8::new().solve_p1(input));
        }
        "8-2" => {
            println!("{}", AOC8::new().solve_p2(input));
        }
        "9-1" => {
            println!("{}", AOC9::solve_p1(input));
        }
        "9-2" => {
            println!("{}", AOC9::solve_p2(input));
        }
        "10-1" => {
            let input = pad_inputs(input, ".".to_owned());
            println!("{}", AOC10::new().solve_p1(input));
        }
        _ => {
            println!("Args should be 1 - 25");
        }
    }
}


