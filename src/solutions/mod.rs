use std::fs::read_to_string;

mod day01;
pub fn day01() {
    println!("day01:");
    let text = read_to_string("res/day01.txt").unwrap();
    println!("{}", day01::part1(&text));
    println!("{}", day01::part2(&text));
}
mod day02;
pub fn day02() {
    println!("day02:");
    let text = read_to_string("res/day02.txt").unwrap();
    println!("{}", day02::part1(&text));
    println!("{}", day02::part2(&text));
}
mod day03;
pub fn day03() {
    println!("day03:");
    let text = read_to_string("res/day03.txt").unwrap();
    println!("{}", day03::part1(&text));
    println!("{}", day03::part2(&text));
}
mod day04;
pub fn day04() {
    println!("day04:");
    let text = read_to_string("res/day04.txt").unwrap();
    println!("{}", day04::part1(&text));
    println!("{}", day04::part2(&text));
}
mod day05;
pub fn day05() {
    println!("day05:");
    let text = read_to_string("res/day05.txt").unwrap();
    println!("{}", day05::part1(&text));
    println!("{}", day05::part2(&text));
}
mod day06;
pub fn day06() {
    println!("day06:");
    let text = read_to_string("res/day06.txt").unwrap();
    println!("{}", day06::part1(&text));
    println!("{}", day06::part2(&text));
}
mod day07;
pub fn day07() {
    println!("day07:");
    let text = read_to_string("res/day07.txt").unwrap();
    println!("{}", day07::part1(&text));
    println!("{}", day07::part2(&text));
}
mod day08;
pub fn day08() {
    println!("day08:");
    let text = read_to_string("res/day08.txt").unwrap();
    println!("{}", day08::part1(&text));
    println!("{}", day08::part2(&text));
}
mod day09;
pub fn day09() {
    println!("day09:");
    let text = read_to_string("res/day09.txt").unwrap();
    println!("{}", day09::part1(&text));
    println!("{}", day09::part2(&text));
}
mod day10;
pub fn day10() {
    println!("day10:");
    let text = read_to_string("res/day10.txt").unwrap();
    println!("{}", day10::part1(&text));
    let sprite = day10::part2(&text);
    for l in sprite {
        println!("{}", String::from_utf8(l).unwrap());
    }
}
mod day11;
pub fn day11() {
    println!("day11:");
    let text = read_to_string("res/day11.txt").unwrap();
    println!("{}", day11::part1(&text));
    println!("{}", day11::part2(&text));
}
mod day12;
pub fn day12() {
    println!("day12:");
    let text = read_to_string("res/day12.txt").unwrap();
    println!("{}", day12::part1(&text));
    println!("{}", day12::part2(&text));
}
mod day13;
pub fn day13() {
    println!("day13:");
    let text = read_to_string("res/day13.txt").unwrap();
    println!("{}", day13::part1(&text));
    println!("{}", day13::part2(&text));
}
mod day14;
pub fn day14() {
    println!("day14:");
    let text = read_to_string("res/day14.txt").unwrap();
    println!("{}", day14::part1(&text));
    println!("{}", day14::part2(&text));
}
mod day15;
pub fn day15() {
    println!("day15:");
    let text = read_to_string("res/day15.txt").unwrap();
    println!("{}", day15::part1(&text));
    println!("{}", day15::part2(&text));
}
mod day16;
pub fn day16() {
    println!("day16:");
    let text = read_to_string("res/day16.txt").unwrap();
    println!("{}", day16::part1(&text));
    println!("{}", day16::part2(&text));
}
mod day17;
pub fn day17() {
    println!("day17:");
    let text = read_to_string("res/day17.txt").unwrap();
    println!("{}", day17::part1(&text));
    println!("{}", day17::part2(&text));
}
mod day18;
pub fn day18() {
    println!("day18:");
    let text = read_to_string("res/day18.txt").unwrap();
    println!("{}", day18::part1(&text));
    println!("{}", day18::part2(&text));
}
mod day19;
pub fn day19() {
    println!("day19:");
    let text = read_to_string("res/day19.txt").unwrap();
    println!("{}", day19::part1(&text));
    println!("{}", day19::part2(&text));
}
mod day20;
pub fn day20() {
    println!("day20:");
    let text = read_to_string("res/day20.txt").unwrap();
    println!("{}", day20::part1(&text));
    println!("{}", day20::part2(&text));
}
mod day21;
pub fn day21() {
    println!("day21:");
    let text = read_to_string("res/day21.txt").unwrap();
    println!("{}", day21::part1(&text));
    println!("{}", day21::part2(&text));
}
mod day22;
pub fn day22() {
    println!("day22:");
    let text = read_to_string("res/day22.txt").unwrap();
    println!("{}", day22::part1(&text));
    println!("{}", day22::part2(&text));
}
mod day23;
pub fn day23() {
    println!("day23:");
    let text = read_to_string("res/day23.txt").unwrap();
    println!("{}", day23::part1(&text));
    println!("{}", day23::part2(&text));
}
mod day24;
pub fn day24() {
    println!("day24:");
    let text = read_to_string("res/day24.txt").unwrap();
    println!("{}", day24::part1(&text));
    println!("{}", day24::part2(&text));
}
mod day25;
pub fn day25() {
    println!("day25:");
    let text = read_to_string("res/day25.txt").unwrap();
    println!("{}", day25::part1(&text));
}
