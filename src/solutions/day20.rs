pub(crate) fn part1(text: &str) -> i64 {
    let mut list = Vec::new();
    for (i, line) in text.lines().enumerate() {
        list.push((i, line.parse::<i64>().unwrap()));
    }
    for i in 0..list.len() {
        let pos = list.iter().position(|&(v, _)| v == i).unwrap();
        let x = list.remove(pos).1;
        let new_i = (pos as i64 + x).rem_euclid(list.len() as i64) as usize;
        list.insert(new_i, (i, x));
    }

    let zero = list.iter().position(|&(_, x)| x == 0).unwrap();
    list[(zero + 1000) % list.len()].1
        + list[(zero + 2000) % list.len()].1
        + list[(zero + 3000) % list.len()].1
}

pub(crate) fn part2(text: &str) -> i64 {
    let mut list = Vec::new();
    for (i, line) in text.lines().enumerate() {
        list.push((i, line.parse::<i64>().unwrap() * 811589153));
    }
    for _ in 0..10 {
        for i in 0..list.len() {
            let pos = list.iter().position(|&(v, _)| v == i).unwrap();
            let x = list.remove(pos).1;
            let new_i = (pos as i64 + x).rem_euclid(list.len() as i64) as usize;
            list.insert(new_i, (i, x));
        }
    }
    let zero = list.iter().position(|&(_, x)| x == 0).unwrap();
    list[(zero + 1000) % list.len()].1
        + list[(zero + 2000) % list.len()].1
        + list[(zero + 3000) % list.len()].1
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day20.txt";
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
