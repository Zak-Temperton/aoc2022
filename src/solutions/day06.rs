use std::collections::VecDeque;

fn solution(text: &str, packet_len: usize) -> usize {
    let mut packet = VecDeque::with_capacity(packet_len);
    let mut count = 0;

    for (i, c) in text.chars().enumerate().take(packet_len) {
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
            if (packet_len - 1) - j >= count {
                count = (packet_len - 1) - j;
            }
        } else if count == 0 {
            return i + 1;
        }
        count -= 1;
        packet.push_back(c);
    }
    panic!("Marker not found")
}

pub(crate) fn part1(text: &str) -> usize {
    solution(text, 4)
}

pub(crate) fn part2(text: &str) -> usize {
    solution(text, 14)
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
