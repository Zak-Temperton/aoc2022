fn setup(lines: &mut std::str::Lines) -> Vec<Vec<char>> {
    let mut stacks = vec![Vec::new(); 9];
    for line in lines.next_chunk::<8>().unwrap().iter().rev() {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .for_each(|(i, c)| stacks[i].push(c));
    }
    stacks
}

fn get_params(line: &str, stacks: &[Vec<char>]) -> (usize, usize, usize, usize) {
    let mut split = line.split_whitespace().skip(1).step_by(2);
    let amount = split.next().unwrap().parse::<usize>().unwrap();
    let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
    let to = split.next().unwrap().parse::<usize>().unwrap() - 1;
    let from_len = stacks[from].len();
    (amount, from, to, from_len)
}

fn get_result(stacks: Vec<Vec<char>>) -> String {
    let mut res = String::new();
    for s in stacks {
        res.push(*(s.last().unwrap()));
    }
    res
}

pub fn part1(text: &str) -> String {
    let mut lines = text.lines();
    let mut stacks = setup(&mut lines);
    for line in lines.skip(2) {
        let (amount, from, to, from_len) = get_params(line, &stacks);
        for i in 1..=amount {
            let tmp = stacks[from][from_len - i];
            stacks[to].push(tmp);
        }
        stacks[from].truncate(from_len - amount);
    }
    get_result(stacks)
}

pub fn part2(text: &str) -> String {
    let mut lines = text.lines();
    let mut stacks = setup(&mut lines);
    for line in lines.skip(2) {
        let (amount, from, to, from_len) = get_params(line, &stacks);
        for i in (1..=amount).rev() {
            let tmp = stacks[from][from_len - i];
            stacks[to].push(tmp);
        }
        stacks[from].truncate(from_len - amount);
    }
    get_result(stacks)
}

#[cfg(test)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day05.txt";
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
