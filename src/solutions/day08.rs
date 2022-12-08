pub(crate) fn part1(text: &str) -> usize {
    let mut forest = Vec::new();
    for line in text.lines() {
        forest.push(line.as_bytes());
    }
    let height = forest.len();
    let width = forest[0].len();
    let mut visible = vec![vec![false; width]; height];
    let mut count = 0;
    let mut highest;
    for y in 0..height {
        highest = 0;
        for x in 0..width {
            if forest[y][x] > highest {
                highest = forest[y][x];
                if !visible[y][x] {
                    visible[y][x] = true;
                    count += 1;
                }
                if highest == b'9' {
                    break;
                }
            }
        }
        highest = 0;
        for x in (0..width).rev() {
            if forest[y][x] > highest {
                highest = forest[y][x];
                if !visible[y][x] {
                    visible[y][x] = true;
                    count += 1;
                }
                if highest == b'9' {
                    break;
                }
            }
        }
    }
    for x in 0..width {
        highest = 0;
        for y in 0..height {
            if forest[y][x] > highest {
                highest = forest[y][x];
                if !visible[y][x] {
                    visible[y][x] = true;
                    count += 1;
                }
                if highest == b'9' {
                    break;
                }
            }
        }
        highest = 0;
        for y in (0..height).rev() {
            if forest[y][x] > highest {
                highest = forest[y][x];
                if !visible[y][x] {
                    visible[y][x] = true;
                    count += 1;
                }
                if highest == b'9' {
                    break;
                }
            }
        }
    }
    count
}

pub(crate) fn part2(text: &str) -> usize {
    let mut forest = Vec::new();
    for line in text.lines() {
        forest.push(line.as_bytes());
    }
    let height = forest.len();
    let width = forest[0].len();
    let mut max_score = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut score = 1;
            let cur_tree = forest[y][x];
            let mut view = 0;

            for x in (0..x).rev() {
                view += 1;
                if forest[y][x] >= cur_tree {
                    break;
                }
            }
            score *= view;
            view = 0;
            for x in x + 1..width {
                view += 1;
                if forest[y][x] >= cur_tree {
                    break;
                }
            }
            score *= view;
            view = 0;
            for y in (0..y).rev() {
                view += 1;
                if forest[y][x] >= cur_tree {
                    break;
                }
            }
            score *= view;
            view = 0;
            for y in y + 1..height {
                view += 1;
                if forest[y][x] >= cur_tree {
                    break;
                }
            }
            score *= view;

            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day08.txt";
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
