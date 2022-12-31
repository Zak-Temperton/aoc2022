fn solution<P>(text: &str, p: P) -> u32
where
    P: Fn(&[i32; 4]) -> bool,
{
    let mut count = 0;
    let mut a = [0; 4];
    for line in text.lines() {
        let mut split = line.split(',').flat_map(|s| s.split('-'));
        for a in &mut a {
            *a = split.next().unwrap().parse().unwrap();
        }
        if p(&a) {
            count += 1;
        }
    }
    count
}

pub fn part1(text: &str) -> u32 {
    solution(text, |a| {
        (a[0] <= a[2] && a[1] >= a[3]) || (a[0] >= a[2] && a[1] <= a[3])
    })
}
pub fn part2(text: &str) -> u32 {
    solution(text, |a| a[0] <= a[3] && a[1] >= a[2])
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day04.txt";
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
