use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    minute: usize,
}

impl State {
    pub fn new(pos: (usize, usize), minute: usize) -> Self {
        State { pos, minute }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.pos.0 + other.pos.1 + other.minute).cmp(&(self.pos.0 + self.pos.1 + self.minute))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn part1(text: &str) -> usize {
    let height = text.lines().count();
    let width = text.lines().next().unwrap().len();
    let period = 600;
    let mut valleys = init(text, period, height, width);

    min_time(
        State {
            pos: (1, 0),
            minute: 0,
        },
        (width - 2, height - 2),
        period,
        height,
        &mut valleys,
        width,
    )
}

fn init(text: &str, period: usize, height: usize, width: usize) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut valleys = vec![vec![vec![vec![]; width]; height]];
    for (y, line) in text.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' | '>' | 'v' | '<' => {
                    valleys[0][y][x].push(c);
                }
                _ => {}
            }
        }
    }
    for z in 1..period {
        valleys.push(vec![vec![vec![]; width]; height]);
        let mut new_pos = Vec::new();
        for y in 1..height {
            for x in 1..width {
                for &c in &valleys[z - 1][y][x] {
                    new_pos.push(match c {
                        '<' => (
                            ((x as isize - 2).rem_euclid(width as isize - 2) + 1) as usize,
                            y as usize,
                            c,
                        ),
                        '>' => ((x % (width - 2)) + 1, y as usize, c),
                        '^' => (
                            x,
                            ((y as isize - 2).rem_euclid(height as isize - 2) + 1) as usize,
                            c,
                        ),
                        'v' => (x as usize, (y % (height - 2)) + 1, c),
                        _ => panic!(),
                    });
                }
            }
        }
        for (dx, dy, c) in new_pos {
            valleys[z][dy][dx].push(c);
        }
    }
    valleys
}

fn min_time(
    start: State,
    target: (usize, usize),
    period: usize,
    height: usize,
    valleys: &mut [Vec<Vec<Vec<char>>>],
    width: usize,
) -> usize {
    let mut next = BinaryHeap::new();
    next.push(start);
    while let Some(n) = next.pop() {
        let pos = n.pos;
        let z = (n.minute + 1) % period;
        if pos == target {
            return n.minute + 1;
        }

        if pos.1 == 0 {
            if (pos.1 < height - 2) && valleys[z][pos.1 + 1][pos.0].is_empty() {
                valleys[z][pos.1 + 1][pos.0].push('.');
                next.push(State::new((pos.0, pos.1 + 1), n.minute + 1));
            }
            if valleys[z][pos.1][pos.0].is_empty() {
                valleys[z][pos.1][pos.0].push('.');
                next.push(State::new(pos, n.minute + 1));
            }
            continue;
        } else if pos.1 == height - 1 {
            if valleys[z][pos.1][pos.0].is_empty() {
                valleys[z][pos.1][pos.0].push('.');
                next.push(State::new(pos, n.minute + 1));
            }
            if pos.1 > 1 && valleys[z][pos.1 - 1][pos.0].is_empty() {
                valleys[z][pos.1 - 1][pos.0].push('.');
                next.push(State::new((pos.0, pos.1 - 1), n.minute + 1));
            }
            continue;
        } else {
            if (pos.1 < height - 2) && valleys[z][pos.1 + 1][pos.0].is_empty() {
                valleys[z][pos.1 + 1][pos.0].push('.');
                next.push(State::new((pos.0, pos.1 + 1), n.minute + 1));
            }
            if valleys[z][pos.1][pos.0].is_empty() {
                valleys[z][pos.1][pos.0].push('.');
                next.push(State::new(pos, n.minute + 1));
            }
            if pos.0 < width - 2 && valleys[z][pos.1][pos.0 + 1].is_empty() {
                valleys[z][pos.1][pos.0 + 1].push('.');
                next.push(State::new((pos.0 + 1, pos.1), n.minute + 1));
            }
            if pos.0 > 1 && valleys[z][pos.1][pos.0 - 1].is_empty() {
                valleys[z][pos.1][pos.0 - 1].push('.');
                next.push(State::new((pos.0 - 1, pos.1), n.minute + 1));
            }
            if pos.1 > 1 && valleys[z][pos.1 - 1][pos.0].is_empty() {
                valleys[z][pos.1 - 1][pos.0].push('.');
                next.push(State::new((pos.0, pos.1 - 1), n.minute + 1));
            }
        }
    }
    unreachable!()
}

pub(crate) fn part2(text: &str) -> usize {
    let height = text.lines().count();
    let width = text.lines().next().unwrap().len();
    let period = 600;
    let mut valleys = init(text, period, height, width);

    let to = min_time(
        State::new((1, 0), 0),
        (width - 2, height - 2),
        period,
        height,
        &mut valleys,
        width,
    );
    let from = min_time(
        State::new((width - 2, height - 1), to),
        (1, 1),
        period,
        height,
        &mut valleys,
        width,
    );
    min_time(
        State::new((1, 0), from + 1),
        (width - 2, height - 2),
        period,
        height,
        &mut valleys,
        width,
    )
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day24.txt";
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
