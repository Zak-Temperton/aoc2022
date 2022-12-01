pub(crate) fn part1(text: &str) -> u32 {
    let mut max = 0;
    let mut cur = 0;
    for line in text.lines() {
        if line.is_empty() {
            if cur > max {
                max = cur;
            }
            cur = 0;
        } else {
            cur += line.parse::<u32>().unwrap();
        }
    }
    max
}

pub(crate) fn part2(text: &str) -> u32 {
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut cur = 0;
    for line in text.lines() {
        if line.is_empty() {
            if cur > first {
                third = second;
                second = first;
                first = cur;
            } else if cur > second {
                third = second;
                second = cur;
            } else if cur > third {
                third = cur;
            }
            cur = 0;
        } else {
            cur += line.parse::<u32>().unwrap();
        }
    }
    first + second + third
}

#[allow(soft_unstable, unused_imports)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day01_part1(b: &mut Bencher) {
        let text = read_to_string("res/day01.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day01_part2(b: &mut Bencher) {
        let text = read_to_string("res/day01.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
