fn decimal_to_snafu(mut decimal: i64) -> String {
    let mut res = Vec::new();
    while decimal != 0 {
        decimal += 2;
        match decimal % 5 {
            0 => res.push(b'='),
            1 => res.push(b'-'),
            2 => res.push(b'0'),
            3 => res.push(b'1'),
            4 => res.push(b'2'),
            _ => {}
        }
        decimal /= 5;
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut res = 0;
    for c in snafu.chars() {
        res *= 5;
        res += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => 0,
        };
    }
    res
}

pub(crate) fn part1(text: &str) -> String {
    decimal_to_snafu(text.lines().map(|line| snafu_to_decimal(line)).sum())
}

pub(crate) fn part2(text: &str) -> usize {
    todo!("still need 50 stars")
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day25.txt";
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let text = read_to_string(PATH).unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    #[ignore = "TODO"]
    fn part2_bench(b: &mut Bencher) {
        let text = read_to_string(PATH).unwrap();
        b.iter(|| part2(&text));
    }
}
