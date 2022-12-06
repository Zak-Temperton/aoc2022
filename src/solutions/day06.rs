use std::collections::VecDeque;

pub(crate) fn part1(text: &str) -> usize {
    let mut packet = VecDeque::with_capacity(4);
    let mut count = 0;

    for (i, c) in text.chars().enumerate().take(4) {
        if let Some(j) = packet.iter().rev().position(|&x| x == c) {
            if i - j >= count {
                count = i - j;
            }
        }
        packet.push_back(c);
    }
    let mut count = 0;
    for (i, c) in text.chars().enumerate().skip(4) {
        packet.pop_front();
        if let Some(j) = packet.iter().rev().position(|&x| x == c) {
            if 3 - j >= count {
                count = 3 - j;
            }
        } else if count == 0 {
            return i + 1;
        }
        count -= 1;
        packet.push_back(c);
    }
    panic!("Marker not found")
}

pub(crate) fn part2(text: &str) -> usize {
    let mut packet = VecDeque::with_capacity(14);
    let mut count = 0;
    for (i, c) in text.chars().enumerate().take(14) {
        if let Some(j) = packet.iter().rev().position(|&x| x == c) {
            if i - j >= count {
                count = i - j;
            }
        }
        packet.push_back(c);
    }
    for (i, c) in text.chars().enumerate().skip(14) {
        packet.pop_front();
        if let Some(j) = packet.iter().rev().position(|&x| x == c) {
            if 13 - j >= count {
                count = 13 - j;
            }
        } else if count == 0 {
            return i + 1;
        }
        count -= 1;
        packet.push_back(c);
    }
    panic!("Marker not found")
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &'static str = "res/day06.txt";
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
