pub(crate) fn part1(text: &str) -> u32 {
    let mut score = 0;
    for line in text.lines() {
        let line = line.as_bytes();
        let left = line[0];
        let right = line[2];
        score += match (left, right) {
            (b'A', b'Y') | (b'B', b'Z') | (b'C', b'X') => right - b'X' + 7,
            (b'B', b'X') | (b'C', b'Y') | (b'A', b'Z') => right - b'X' + 1,
            _ => right - b'X' + 4,
        } as u32;
    }
    score
}

pub(crate) fn part2(text: &str) -> u32 {
    let mut score = 0;
    for line in text.lines() {
        let line = line.as_bytes();
        let left = line[0] - b'A' + 1;
        let right = line[2];
        score += match right {
            b'X' if left == 1 => 3,
            b'X' => left - 1,
            b'Y' => left + 3,
            _ if left == 3 => 7,
            _ => left + 7,
        } as u32;
    }
    score
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &'static str = "res/day02.txt";
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
