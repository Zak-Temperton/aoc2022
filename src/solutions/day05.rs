pub(crate) fn part1(text: &str) -> String {
    let mut stacks = vec![Vec::new(); 9];

    let setup: Vec<_> = text.lines().take(8).collect();
    for line in setup.iter().rev() {
        let bytes = line.as_bytes();
        for i in 0..9 {
            match bytes.get(i * 4 + 1) {
                Some(&c) if c != b' ' => stacks[i].push(c as char),
                _ => {}
            }
        }
    }
    for line in text.lines().skip(10) {
        let line = line.split_whitespace().collect::<Vec<_>>();

        let amount = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap() - 1;
        let to = line[5].parse::<usize>().unwrap() - 1;
        for _ in 0..amount {
            let tmp = stacks[from].pop().unwrap();
            stacks[to].push(tmp);
        }
    }

    let mut res = String::new();
    for s in stacks {
        res.push(*(s.last().unwrap_or(&' ')) as char);
    }
    res
}

pub(crate) fn part2(text: &str) -> String {
    let mut stacks = vec![Vec::new(); 9];

    let setup: Vec<_> = text.lines().take(8).collect();
    for line in setup.iter().rev() {
        let bytes = line.as_bytes();
        for i in 0..9 {
            match bytes.get(i * 4 + 1) {
                Some(&c) if c != b' ' => stacks[i].push(c as char),
                _ => {}
            }
        }
    }
    for line in text.lines().skip(10) {
        let line = line.split_whitespace().collect::<Vec<_>>();

        let amount = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap() - 1;
        let to = line[5].parse::<usize>().unwrap() - 1;

        let from_len = stacks[from].len();
        let mut tmp = stacks[from].split_off(from_len - amount);
        stacks[to as usize].append(&mut tmp);
    }

    let mut res = String::new();
    for s in stacks {
        res.push(*(s.last().unwrap_or(&' ')) as char);
    }
    res
}

#[allow(soft_unstable, unused_imports)]
mod bench_day05 {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day05_part1(b: &mut Bencher) {
        let text = read_to_string("res/day05.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day05_part2(b: &mut Bencher) {
        let text = read_to_string("res/day05.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
