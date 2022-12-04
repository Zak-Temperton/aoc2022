use regex::Regex;

pub(crate) fn part1(text: &str) -> u32 {
    let mut count = 0;
    let rgx = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for cap in rgx.captures_iter(text) {
        let mut cap = cap.iter();
        cap.next();
        let cap = cap
            .flat_map(|c| c.unwrap().as_str().parse::<u32>())
            .collect::<Vec<_>>();
        if (cap[0] <= cap[2] && cap[1] >= cap[3]) || (cap[0] >= cap[2] && cap[1] <= cap[3]) {
            count += 1;
        }
    }
    count
}
pub(crate) fn part2(text: &str) -> u32 {
    let mut count = 0;
    let rgx = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for cap in rgx.captures_iter(text) {
        let cap = cap.iter();
        let cap = cap
            .flat_map(|c| c.unwrap().as_str().parse::<u32>())
            .collect::<Vec<_>>();
        if cap[0] <= cap[3] && cap[1] >= cap[2] {
            count += 1;
        }
    }
    count
}

#[allow(soft_unstable, unused_imports)]
mod bench_day04 {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day04_part1(b: &mut Bencher) {
        let text = read_to_string("res/day04.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day04_part2(b: &mut Bencher) {
        let text = read_to_string("res/day04.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
