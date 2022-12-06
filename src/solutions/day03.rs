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
    let mut lines = text
        .lines()
        .map(|i| i.bytes().map(|c| char_to_priority(&c)));
    let mut sum = 0;
    let mut letters: [u8; 53];
    while let (Some(group_a), Some(group_b), Some(group_c)) =
        (lines.next(), lines.next(), lines.next())
    {
        letters = [0; 53];
        group_a.for_each(|i| letters[i as usize] = 1);
        for letter in group_b {
            let l = &mut letters[letter as usize];
            if *l == 1 {
                *l = 2;
            }
        }
        for letter in group_c {
            let l = &mut letters[letter as usize];
            if *l == 2 {
                sum += letter as u32;
                break;
            }
        }
    }
    sum
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &'static str = "res/day03.txt";
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
