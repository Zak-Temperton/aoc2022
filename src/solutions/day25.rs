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
            '1' => 1,
            '2' => 2,
            _ => 0,
        };
    }
    res
}

pub fn part1(text: &str) -> String {
    decimal_to_snafu(text.lines().map(snafu_to_decimal).sum())
}

#[cfg(test)]
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
}
