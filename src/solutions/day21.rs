use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Job {
    Add(u32, u32),
    Sub(u32, u32),
    Mul(u32, u32),
    Div(u32, u32),
    Num(i64),
}

fn name_to_u32(name: &str) -> u32 {
    let mut res = 0;
    let bytes = name.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        res <<= 8;
        res += bytes[i] as u32;
        i += 1;
    }
    res
}

fn run_job(key: u32, monkeys: &HashMap<u32, Job>) -> i64 {
    match *monkeys.get(&key).unwrap() {
        Job::Add(l, r) => run_job(l, monkeys) + run_job(r, monkeys),
        Job::Sub(l, r) => run_job(l, monkeys) - run_job(r, monkeys),
        Job::Mul(l, r) => run_job(l, monkeys) * run_job(r, monkeys),
        Job::Div(l, r) => run_job(l, monkeys) / run_job(r, monkeys),
        Job::Num(n) => n,
    }
}

pub(crate) fn part1(text: &str) -> i64 {
    let mut monkeys = HashMap::new();
    for line in text.lines() {
        let mut split = line.split(": ");
        let name = name_to_u32(split.next().unwrap());
        let mut job = split.next().unwrap().split_whitespace();
        match job.next().unwrap() {
            c if c.as_bytes()[0].is_ascii_digit() => {
                monkeys.insert(name, Job::Num(c.parse().unwrap()));
            }
            c => match job.next().unwrap() {
                "+" => {
                    let next = job.next().unwrap();
                    monkeys.insert(name, Job::Add(name_to_u32(c), name_to_u32(next)));
                }
                "-" => {
                    let next = job.next().unwrap();
                    monkeys.insert(name, Job::Sub(name_to_u32(c), name_to_u32(next)));
                }
                "*" => {
                    let next = job.next().unwrap();
                    monkeys.insert(name, Job::Mul(name_to_u32(c), name_to_u32(next)));
                }
                "/" => {
                    let next = job.next().unwrap();
                    monkeys.insert(name, Job::Div(name_to_u32(c), name_to_u32(next)));
                }
                _ => unreachable!(),
            },
        }
    }
    run_job(name_to_u32("root"), &monkeys)
}

#[derive(Debug, Clone, Copy)]
enum Input {
    Human(Job),
    Const(Job),
}

fn run_job2(key: u32, monkeys: &HashMap<u32, Input>) -> i64 {
    match *monkeys.get(&key).unwrap() {
        Input::Const(Job::Add(l, r)) => run_job2(l, monkeys) + run_job2(r, monkeys),
        Input::Const(Job::Sub(l, r)) => run_job2(l, monkeys) - run_job2(r, monkeys),
        Input::Const(Job::Mul(l, r)) => run_job2(l, monkeys) * run_job2(r, monkeys),
        Input::Const(Job::Div(l, r)) => run_job2(l, monkeys) / run_job2(r, monkeys),
        Input::Const(Job::Num(n)) => n,
        _ => unreachable!(),
    }
}

fn get_match(key: u32, monkeys: &HashMap<u32, Input>, to_match: i64) -> i64 {
    match *monkeys.get(&key).unwrap() {
        Input::Human(Job::Add(l, r)) => {
            if let Input::Const(_) = monkeys.get(&l).unwrap() {
                get_match(r, &monkeys, to_match - run_job2(l, monkeys))
            } else if let Input::Const(_) = monkeys.get(&r).unwrap() {
                get_match(l, &monkeys, to_match - run_job2(r, monkeys))
            } else {
                unreachable!()
            }
        }
        Input::Human(Job::Sub(l, r)) => {
            if let Input::Const(_) = *monkeys.get(&l).unwrap() {
                get_match(r, &monkeys, run_job2(l, monkeys) - to_match)
            } else if let Input::Const(_) = *monkeys.get(&r).unwrap() {
                get_match(l, &monkeys, to_match + run_job2(r, monkeys))
            } else {
                unreachable!()
            }
        }
        Input::Human(Job::Mul(l, r)) => {
            if let Input::Const(_) = monkeys.get(&l).unwrap() {
                get_match(r, &monkeys, to_match / run_job2(l, monkeys))
            } else if let Input::Const(_) = monkeys.get(&r).unwrap() {
                get_match(l, &monkeys, to_match / run_job2(r, monkeys))
            } else {
                unreachable!()
            }
        }
        Input::Human(Job::Div(l, r)) => {
            if let Input::Const(_) = monkeys.get(&l).unwrap() {
                get_match(r, &monkeys, run_job2(l, monkeys) * to_match)
            } else if let Input::Const(_) = monkeys.get(&r).unwrap() {
                get_match(l, &monkeys, run_job2(r, monkeys) * to_match)
            } else {
                unreachable!()
            }
        }
        Input::Human(Job::Num(_)) => to_match,
        _ => unreachable!(),
    }
}

fn convert(key: u32, monkeys: &mut HashMap<u32, Input>) {
    match *monkeys.get(&key).unwrap() {
        Input::Human(_) => unreachable!(),
        Input::Const(j) => *monkeys.get_mut(&key).unwrap() = Input::Human(j),
    }
}

fn prepare(key: u32, monkeys: &mut HashMap<u32, Input>) -> bool {
    if key == name_to_u32("humn") {
        convert(key, monkeys);
        return true;
    }
    match *monkeys.get(&key).unwrap() {
        Input::Const(Job::Add(l, r))
        | Input::Const(Job::Sub(l, r))
        | Input::Const(Job::Mul(l, r))
        | Input::Const(Job::Div(l, r)) => {
            if prepare(l, monkeys) {
                convert(key, monkeys);
                true
            } else if prepare(r, monkeys) {
                convert(key, monkeys);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

pub(crate) fn part2(text: &str) -> i64 {
    let mut monkeys = HashMap::new();
    for line in text.lines() {
        let mut split = line.split(": ");
        let name = name_to_u32(split.next().unwrap());
        let mut job = split.next().unwrap().split_whitespace();
        match job.next().unwrap() {
            c if c.as_bytes()[0].is_ascii_digit() => {
                monkeys.insert(name, Input::Const(Job::Num(c.parse().unwrap())));
            }
            c => match job.next().unwrap() {
                "+" => {
                    let next = job.next().unwrap();
                    monkeys.insert(
                        name,
                        Input::Const(Job::Add(name_to_u32(c), name_to_u32(next))),
                    );
                }
                "-" => {
                    let next = job.next().unwrap();
                    monkeys.insert(
                        name,
                        Input::Const(Job::Sub(name_to_u32(c), name_to_u32(next))),
                    );
                }
                "*" => {
                    let next = job.next().unwrap();
                    monkeys.insert(
                        name,
                        Input::Const(Job::Mul(name_to_u32(c), name_to_u32(next))),
                    );
                }
                "/" => {
                    let next = job.next().unwrap();
                    monkeys.insert(
                        name,
                        Input::Const(Job::Div(name_to_u32(c), name_to_u32(next))),
                    );
                }
                _ => unreachable!(),
            },
        }
    }
    prepare(name_to_u32("root"), &mut monkeys);
    let (left, right) = match *monkeys.get(&name_to_u32("root")).unwrap() {
        Input::Human(Job::Add(l, r))
        | Input::Human(Job::Sub(l, r))
        | Input::Human(Job::Mul(l, r))
        | Input::Human(Job::Div(l, r)) => (l, r),
        _ => unreachable!(),
    };
    if let Input::Const(_) = monkeys.get(&left).unwrap() {
        let to_match = run_job2(left, &mut monkeys);
        get_match(right, &monkeys, to_match)
    } else {
        let to_match = run_job2(right, &mut monkeys);
        get_match(left, &monkeys, to_match)
    }
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day21.txt";
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
