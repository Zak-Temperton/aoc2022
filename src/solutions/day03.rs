pub fn part1(text: &str) -> u32 {
    let mut sum = 0;
    let mut letters: [bool; 53];

    for line in text.lines() {
        let half = line.len() / 2;
        let left = line[0..half].bytes().map(char_to_priority);
        let right = line[half..].bytes().map(char_to_priority);
        letters = [false; 53];

        left.for_each(|i| letters[i as usize] = true);
        for i in right {
            let l = letters[i as usize];
            if l {
                sum += i as u32;
                break;
            }
        }
    }
    sum
}

const fn char_to_priority(c: u8) -> u8 {
    if c < b'a' {
        c - b'A' + 27
    } else {
        c - b'a' + 1
    }
}

pub fn part2(text: &str) -> u32 {
    let mut lines = text.lines().map(|i| i.bytes().map(char_to_priority));
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
            let l = letters[letter as usize];
            if l == 2 {
                sum += letter as u32;
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day03.txt";
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
