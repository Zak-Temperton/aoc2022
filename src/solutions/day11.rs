use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: (Option<usize>, Operation, Option<usize>),
    test: usize,
    iftrue: usize,
    iffalse: usize,
    inspections: usize,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Mul,
    Div,
    Add,
    Sub,
}

impl Monkey {
    fn new(lines: [&str; 7]) -> Monkey {
        Monkey {
            items: lines[1][18..].split(", ").flat_map(|s| s.parse()).collect(),
            operation: {
                let mut split = lines[2][19..].split_whitespace();
                (
                    match split.next().unwrap() {
                        "old" => None,
                        c => Some(c.parse().unwrap()),
                    },
                    match split.next().unwrap() {
                        "*" => Operation::Mul,
                        "/" => Operation::Div,
                        "+" => Operation::Add,
                        "-" => Operation::Sub,
                        _ => panic!(),
                    },
                    match split.next().unwrap() {
                        "old" => None,
                        c => Some(c.parse().unwrap()),
                    },
                )
            },
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
                let mut item = monkeys[m].items[0];
                let val1 = match operation.0 {
                    None => item,
                    Some(n) => n,
                };
                let val2 = match operation.2 {
                    None => item,
                    Some(n) => n,
                };
                item = match operation.1 {
                    Operation::Mul => val1 * val2,
                    Operation::Div => val1 / val2,
                    Operation::Add => val1 + val2,
                    Operation::Sub => val1 - val2,
                } / 3;

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
            let operation = monkeys[m].operation;
            monkeys[m].inspections += monkeys[m].items.len();
            while !monkeys[m].items.is_empty() {
                let mut item = monkeys[m].items[0];
                let val1 = match operation.0 {
                    None => item,
                    Some(n) => n,
                };
                let val2 = match operation.2 {
                    None => item,
                    Some(n) => n,
                };
                item = match operation.1 {
                    Operation::Mul => val1 * val2,
                    Operation::Div => val1 / val2,
                    Operation::Add => val1 + val2,
                    Operation::Sub => val1 - val2,
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
