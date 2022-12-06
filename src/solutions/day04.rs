pub(crate) fn part1(text: &str) -> u32 {
    let mut count = 0;
    let mut a = [0; 4];
    for line in text.lines() {
        let mut split = line.split(',').flat_map(|s| s.split('-'));
        for a in a.iter_mut() {
            *a = split.next().unwrap().parse().unwrap();
        }
        if (a[0] <= a[2] && a[1] >= a[3]) || (a[0] >= a[2] && a[1] <= a[3]) {
            count += 1;
        }
    }
    count
}
pub(crate) fn part2(text: &str) -> u32 {
    let mut count = 0;
    let mut a = [0; 4];
    for line in text.lines() {
        let mut split = line.split(',').flat_map(|s| s.split('-'));
        for a in a.iter_mut() {
            *a = split.next().unwrap().parse().unwrap();
        }
        if a[0] <= a[3] && a[1] >= a[2] {
            count += 1;
        }
    }
    count
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &'static str = "res/day04.txt";
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
