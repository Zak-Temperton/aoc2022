#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: (Operation, Option<usize>),
    test: usize,
    if_true: usize,
    if_false: usize,
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
                let mut split = lines[2][23..].split_whitespace();
                (
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
            if_true: lines[4][29..].parse().unwrap(),
            if_false: lines[5][30..].parse().unwrap(),
            inspections: 0,
        }
    }

    fn test(&self, item: usize) -> usize {
        if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
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

pub(crate) fn part1(text: &str) -> usize {
    let mut monkeys = Vec::new();
    for lines in text.lines().array_chunks::<7>() {
        monkeys.push(Monkey::new(lines));
    }
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let operation = monkeys[m].operation;
            monkeys[m].inspections += monkeys[m].items.len();
            for i in 0..monkeys[m].items.len() {
                let mut item = monkeys[m].items[i];
                let val = match operation.1 {
                    None => item,
                    Some(n) => n,
                };
                item = match operation.0 {
                    Operation::Mul => item * val,
                    Operation::Div => item / val,
                    Operation::Add => item + val,
                    Operation::Sub => item - val,
                } / 3;

                let target = monkeys[m].test(item);
                monkeys[target].items.push(item);
            }
            monkeys[m].items.clear();
        }
    }
    calc_monkey_business(monkeys)
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
            for i in 0..monkeys[m].items.len() {
                let mut item = monkeys[m].items[i];
                let val = match operation.1 {
                    None => item,
                    Some(n) => n,
                };
                item = match operation.0 {
                    Operation::Mul => item * val,
                    Operation::Div => item / val,
                    Operation::Add => item + val,
                    Operation::Sub => item - val,
                } % multiple;

                let target = monkeys[m].test(item);
                monkeys[target].items.push(item);
            }
            monkeys[m].items.clear();
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
