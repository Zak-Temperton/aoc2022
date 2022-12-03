#![feature(test)]

extern crate test;

mod solutions;
use solutions::*;

use regex::Regex;

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("please choose a day to run");
        println!("cargo run <day>");
        return;
    }
    args.next(); //executable path
    let r_day = Regex::new(r"(?:([\d]+))").unwrap();
    let day = args.next().unwrap();
    let captures = r_day.captures(day.as_str()).unwrap();
    match captures.get(1).unwrap().as_str().parse::<u32>().unwrap() {
        1 => day01(),
        2 => day02(),
        3 => day03(),
        // 4 => day04(),
        // 5 => day05(),
        // 6 => day06(),
        // 7 => day07(),
        // 8 => day08(),
        // 9 => day09(),
        // 10 => day10(),
        // 11 => day11(),
        // 12 => day12(),
        // 13 => day13(),
        // 14 => day14(),
        // 15 => day15(),
        // 16 => day16(),
        // 17 => day17(),
        // 18 => day18(),
        // 19 => day19(),
        // 20 => day20(),
        // 21 => day21(),
        // 22 => day22(),
        // 23 => day23(),
        // 24 => day24(),
        // 25 => day25(),
        _ => println!("Choose a valid day"),
    }
}
