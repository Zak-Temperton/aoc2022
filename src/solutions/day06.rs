use std::collections::VecDeque;

pub(crate) fn part1(text: &str) -> usize {
    let mut packet = VecDeque::new();
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
            } else {
                count -= 1;
            }
            packet.push_back(c);
        } else if count == 1 {
            return i + 1;
        } else {
            packet.push_back(c);
            count -= 1;
        }
    }
    panic!("Marker not found")
}

pub(crate) fn part2(text: &str) -> usize {
    let mut packet = VecDeque::new();
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
            } else {
                count -= 1;
            }
            packet.push_back(c);
        } else if count == 1 {
            return i + 1;
        } else {
            packet.push_back(c);
            count -= 1;
        }
    }
    panic!("Marker not found")
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let text = read_to_string("res/day06.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let text = read_to_string("res/day06.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
