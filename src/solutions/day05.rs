fn setup(lines: &mut std::str::Lines) -> Vec<Vec<char>> {
    let mut stacks = vec![Vec::new(); 9];
    for _ in 0..8 {
        let bytes = lines.next().unwrap().as_bytes();
        for (i, stack) in stacks.iter_mut().enumerate() {
            match bytes.get(i * 4 + 1) {
                Some(&c) if c != b' ' => stack.push(c as char),
                Some(_) => {}
                _ => break,
            }
        }
    }
    stacks.iter_mut().for_each(|v| v.reverse());
    stacks
}

pub(crate) fn part1(text: &str) -> String {
    let mut lines = text.lines();
    let mut stacks = setup(&mut lines);
    for line in lines.skip(2) {
        let split = line.split_whitespace().collect::<Vec<_>>();

        let amount = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;
        for _ in 0..amount {
            let tmp = stacks[from].pop().unwrap();
            stacks[to].push(tmp);
        }
    }
    let mut res = String::new();
    for s in stacks {
        res.push(*(s.last().unwrap()));
    }
    res
}

pub(crate) fn part2(text: &str) -> String {
    let mut lines = text.lines();
    let mut stacks = setup(&mut lines);
    for line in lines.skip(2) {
        let split = line.split_whitespace().collect::<Vec<_>>();

        let amount = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        let from_len = stacks[from].len();
        let mut tmp = stacks[from].split_off(from_len - amount);
        stacks[to].append(&mut tmp);
    }

    let mut res = String::new();
    for s in stacks {
        res.push(*(s.last().unwrap_or(&' ')) as char);
    }
    res
}
#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &'static str = "res/day05.txt";
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
