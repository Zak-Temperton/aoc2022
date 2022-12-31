fn visible_sides<R>(
    range: R,
    forest: &[&[u8]],
    y: usize,
    visible: &mut [Vec<bool>],
    count: &mut usize,
) where
    R: DoubleEndedIterator<Item = usize>,
{
    let mut highest = 0;
    for x in range {
        if forest[y][x] > highest {
            highest = forest[y][x];
            if !visible[y][x] {
                visible[y][x] = true;
                *count += 1;
            }
            if highest == b'9' {
                break;
            }
        }
    }
}

fn visibile_surface<R>(
    range: R,
    forest: &[&[u8]],
    x: usize,
    visible: &mut [Vec<bool>],
    count: &mut usize,
) where
    R: DoubleEndedIterator<Item = usize>,
{
    let mut highest = 0;
    for y in range {
        if forest[y][x] > highest {
            highest = forest[y][x];
            if !visible[y][x] {
                visible[y][x] = true;
                *count += 1;
            }
            if highest == b'9' {
                break;
            }
        }
    }
}

pub fn part1(text: &str) -> usize {
    let mut forest = Vec::new();
    for line in text.lines() {
        forest.push(line.as_bytes());
    }
    let side = forest.len();
    let mut visible = vec![vec![false; side]; side];
    let mut count = 0;
    for i in 0..side {
        visible_sides(0..side, &forest, i, &mut visible, &mut count);
        visible_sides((0..side).rev(), &forest, i, &mut visible, &mut count);
        visibile_surface(0..side, &forest, i, &mut visible, &mut count);
        visibile_surface((0..side).rev(), &forest, i, &mut visible, &mut count);
    }
    count
}

fn look_across<R>(score: &mut usize, forest: &[&[u8]], y: usize, cur_tree: u8, range: R)
where
    R: DoubleEndedIterator<Item = usize>,
{
    let mut view = 0;
    for x in range {
        view += 1;
        if forest[y][x] >= cur_tree {
            break;
        }
    }
    *score *= view;
}

fn look_verticle<R>(score: &mut usize, forest: &[&[u8]], x: usize, cur_tree: u8, range: R)
where
    R: DoubleEndedIterator<Item = usize>,
{
    let mut view = 0;
    for y in range {
        view += 1;
        if forest[y][x] >= cur_tree {
            break;
        }
    }
    *score *= view;
}

pub fn part2(text: &str) -> usize {
    let mut forest = Vec::new();
    for line in text.lines() {
        forest.push(line.as_bytes());
    }
    let side = forest.len();

    let mut max_score = 0;
    for y in 1..side - 1 {
        for x in 1..side - 1 {
            let mut score = 1;
            let cur_tree = forest[y][x];

            look_across(&mut score, &forest, y, cur_tree, (0..x).rev());
            look_across(&mut score, &forest, y, cur_tree, x + 1..side);
            look_verticle(&mut score, &forest, x, cur_tree, (0..y).rev());
            look_verticle(&mut score, &forest, x, cur_tree, y + 1..side);

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
