use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Vec<String>,
    test: usize,
    iftrue: usize,
    iffalse: usize,
    inspections: usize,
}
impl Monkey {
    fn new(lines: [&str; 7]) -> Monkey {
        Monkey {
            items: lines[1][18..].split(", ").flat_map(|s| s.parse()).collect(),
            operation: lines[2][19..]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
            test: lines[3][21..].parse().unwrap(),
            iftrue: lines[4][29..].parse().unwrap(),
            iffalse: lines[5][30..].parse().unwrap(),
            inspections: 0,
        }
    }
}

pub(crate) fn part1(text: &str) -> usize {
    let mut monkeys = Vec::new();
    for lines in text.lines().array_chunks::<7>() {
        monkeys.push(Monkey::new(lines));
    }
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            monkeys[m].inspections += monkeys[m].items.len();
            let operation = monkeys[m].operation.clone();
            while !monkeys[m].items.is_empty() {
                let val1 = match operation[0].as_str() {
                    "old" => monkeys[m].items[0],
                    c => c.parse().unwrap(),
                };
                let val2 = match operation[2].as_str() {
                    "old" => monkeys[m].items[0],
                    c => c.parse().unwrap(),
                };
                monkeys[m].items[0] = match operation[1].as_str() {
                    "*" => val1 * val2,
                    "/" => val1 / val2,
                    "+" => val1 + val2,
                    "-" => val1 - val2,
                    _ => 1,
                } / 3;

                let target = if monkeys[m].items[0] % monkeys[m].test == 0 {
                    monkeys[m].iftrue
                } else {
                    monkeys[m].iffalse
                };
                let item = monkeys[m].items.pop_front().unwrap();
                monkeys[target].items.push_back(item);
            }
        }
    }
    calc_monkey_business(monkeys)
}

fn calc_monkey_business(monkeys: Vec<Monkey>) -> usize {
    let mut max1 = 0;
    let mut max2 = 0;
    for m in monkeys {
        if m.inspections > max2 {
            if m.inspections >= max1 {
                max2 = max1;
                max1 = m.inspections;
            } else {
                max2 = m.inspections;
            }
        }
    }
    max1 * max2
}

pub(crate) fn part2(text: &str) -> usize {
    let mut monkeys = Vec::new();
    let mut multiple = 1;
    for lines in text.lines().array_chunks::<7>() {
        monkeys.push(Monkey::new(lines));
        multiple *= monkeys.last().unwrap().test;
    }
    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            let operation = monkeys[m].operation.clone();
            monkeys[m].inspections += monkeys[m].items.len();
            while !monkeys[m].items.is_empty() {
                let val1 = match operation[0].as_str() {
                    "old" => monkeys[m].items[0],
                    c => c.parse().unwrap(),
                };
                let val2 = match operation[2].as_str() {
                    "old" => monkeys[m].items[0],
                    c => c.parse().unwrap(),
                };
                let item = match operation[1].as_str() {
                    "*" => val1 * val2,
                    "/" => val1 / val2,
                    "+" => val1 + val2,
                    "-" => val1 - val2,
                    _ => panic!(),
                } % multiple;

                let target = if item % monkeys[m].test == 0 {
                    monkeys[m].iftrue
                } else {
                    monkeys[m].iffalse
                };
                monkeys[m].items.pop_front().unwrap();
                monkeys[target].items.push_back(item);
            }
        }
    }
    calc_monkey_business(monkeys)
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day11.txt";
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let text = read_to_string(PATH).unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let text = read_to_string(PATH).unwrap();
        b.iter(|| part2(&text));
    }
}
