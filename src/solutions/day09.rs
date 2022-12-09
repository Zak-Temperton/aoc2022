use std::collections::HashSet;

pub(crate) fn part1(text: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited_by_tail = HashSet::new();
    visited_by_tail.insert(tail);
    for line in text.lines() {
        let dir = line.as_bytes()[0];
        let steps = line[2..].parse::<i32>().unwrap();
        match dir {
            b'U' => head.1 += steps,
            b'D' => head.1 -= steps,
            b'L' => head.0 -= steps,
            b'R' => head.0 += steps,
            _ => unreachable!(),
        }
        let mut x_dif;
        let mut y_dif;
        while {
            x_dif = head.0 - tail.0;
            y_dif = head.1 - tail.1;
            x_dif.abs() > 1 || y_dif.abs() > 1
        } {
            tail.0 += x_dif.signum();
            tail.1 += y_dif.signum();
            visited_by_tail.insert(tail);
        }
    }
    visited_by_tail.len()
}

pub(crate) fn part2(text: &str) -> usize {
    let mut rope = [(0, 0); 10];
    let mut visited_by_tail = HashSet::new();
    visited_by_tail.insert((0, 0));
    for line in text.lines() {
        let dir = line.as_bytes()[0];
        let steps = line[2..].parse::<i32>().unwrap();
        match dir {
            b'U' => rope[0].1 += steps,
            b'D' => rope[0].1 -= steps,
            b'L' => rope[0].0 -= steps,
            b'R' => rope[0].0 += steps,
            _ => unreachable!(),
        }

        let mut x_dif;
        let mut y_dif;
        let mut settled = false;
        while !settled {
            settled = true;
            for i in 0..8 {
                if {
                    x_dif = rope[i].0 - rope[i + 1].0;
                    y_dif = rope[i].1 - rope[i + 1].1;
                    x_dif.abs() > 1 || y_dif.abs() > 1
                } {
                    rope[i + 1].0 += x_dif.signum();
                    rope[i + 1].1 += y_dif.signum();
                    settled = false;
                }
            }
            if {
                x_dif = rope[8].0 - rope[9].0;
                y_dif = rope[8].1 - rope[9].1;
                x_dif.abs() > 1 || y_dif.abs() > 1
            } {
                rope[9].0 += x_dif.signum();
                rope[9].1 += y_dif.signum();
                visited_by_tail.insert(rope[9]);
                settled = false;
            }
        }
    }
    visited_by_tail.len()
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day09.txt";
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
