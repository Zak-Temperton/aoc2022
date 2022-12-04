pub(crate) fn part1(text: &str) -> u32 {
    let mut sum = 0;
    for line in text.lines() {
        let half = line.len() / 2;
        let left = line[0..half].as_bytes();
        let right = line[half..].as_bytes();
        for l in left.iter() {
            if right.contains(l) {
                sum += char_to_priority(l) as u32;
                break;
            }
        }
    }
    sum
}

const fn char_to_priority(c: &u8) -> u8 {
    if *c < b'a' {
        *c - b'A' + 27
    } else {
        *c - b'a' + 1
    }
}

pub(crate) fn part2(text: &str) -> u32 {
    let lines: Vec<Vec<u8>> = text
        .lines()
        .map(|i| i.bytes().map(|c| char_to_priority(&c)).collect())
        .collect();
    let mut sum = 0;
    let mut letters: [u8; 53];
    for group in lines.chunks(3) {
        letters = [0; 53];
        group[0].iter().for_each(|&i| letters[i as usize] = 1);
        for &letter in group[1].iter() {
            let l = &mut letters[letter as usize];
            if *l == 1 {
                *l = 2;
            }
        }
        for &letter in group[2].iter() {
            let l = &mut letters[letter as usize];
            if *l == 2 {
                sum += letter as u32;
                break;
            }
        }
    }
    sum
}

#[allow(soft_unstable, unused_imports)]
mod bench_day03 {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;

    #[bench]
    fn day03_part1(b: &mut Bencher) {
        let text = read_to_string("res/day03.txt").unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    fn day03_part2(b: &mut Bencher) {
        let text = read_to_string("res/day03.txt").unwrap();
        b.iter(|| part2(&text));
    }
}
