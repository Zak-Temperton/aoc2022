fn setup(lines: &mut std::str::Lines) -> Vec<Vec<char>> {
    let mut stacks = vec![Vec::new(); 9];
    for _ in 0..8 {
        let line = lines.next().unwrap();
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c))
    }
    stacks.iter_mut().for_each(|v| v.reverse());
    stacks
}

pub(crate) fn part1(text: &str) -> String {
    let mut lines = text.lines();
    let mut stacks = setup(&mut lines);
    for line in lines.skip(2) {
        let mut split = line.split_whitespace().skip(1).step_by(2);

        let amount = split.next().unwrap().parse::<usize>().unwrap();
        let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = split.next().unwrap().parse::<usize>().unwrap() - 1;
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
        let mut split = line.split_whitespace().skip(1).step_by(2);

        let amount = split.next().unwrap().parse::<usize>().unwrap();
        let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = split.next().unwrap().parse::<usize>().unwrap() - 1;

        let from_len = stacks[from].len();
        for i in (1..amount + 1).rev() {
            let tmp = stacks[from][from_len - i];
            stacks[to].push(tmp);
        }
        stacks[from].truncate(from_len - amount);
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
