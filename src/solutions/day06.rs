fn solution(input: &[u8], packet_len: usize) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + packet_len) {
        let mut state = 0;
        if let Some(pos) = slice.iter().rposition(|b| {
            let mask = 1 << (b & 0b00011111);
            let ret = state & mask != 0;
            state |= mask;
            ret
        }) {
            idx += pos + 1;
        } else {
            return Some(idx + packet_len);
        }
    }
    None
}

pub fn part1(text: &str) -> usize {
    solution(text.as_bytes(), 4).unwrap()
}

pub fn part2(text: &str) -> usize {
    solution(text.as_bytes(), 14).unwrap()
}

#[cfg(test)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day06.txt";
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
