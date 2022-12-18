pub(crate) fn part1(text: &str) -> usize {
    let mut cube = vec![vec![vec![false; 21]; 21]; 21];
    let mut lava = Vec::new();
    for line in text.lines() {
        let mut split = line.split(',');
        let (x, y, z) = (
            split.next().unwrap().parse::<usize>().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
        );
        lava.push((x, y, z));
        cube[x][y][z] = true;
    }

    let mut surface = 0;
    for &(x, y, z) in lava.iter() {
        for (i, j, k) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            if (x == 21 && i == 1 || y == 21 && j == 1 || z == 21 && k == 1)
                || !cube[x + i][y + j][z + k]
            {
                surface += 1;
            }
            if (x == 0 && i == 1 || y == 0 && j == 1 || z == 0 && k == 1)
                || !cube[x - i][y - j][z - k]
            {
                surface += 1;
            }
        }
    }
    surface
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Lava,
    Water,
    Air,
}

pub(crate) fn part2(text: &str) -> usize {
    let mut cube = vec![vec![vec![State::Air; 22]; 22]; 22];
    for line in text.lines() {
        let mut split = line.split(',');
        let (x, y, z) = (
            split.next().unwrap().parse::<usize>().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
        );
        cube[x + 1][y + 1][z + 1] = State::Lava;
    }

    let mut surface = 0;
    let mut next = vec![(0, 0, 0)];
    while let Some((x, y, z)) = next.pop() {
        for (i, j, k) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            if !(x == 21 && i == 1 || y == 21 && j == 1 || z == 21 && k == 1) {
                match cube[x + i][y + j][z + k] {
                    State::Lava => surface += 1,
                    State::Water => {}
                    State::Air => {
                        next.push((x + i, y + j, z + k));
                        cube[x + i][y + j][z + k] = State::Water;
                    }
                }
            }
            if !(x == 0 && i == 1 || y == 0 && j == 1 || z == 0 && k == 1) {
                match cube[x - i][y - j][z - k] {
                    State::Lava => surface += 1,
                    State::Water => {}
                    State::Air => {
                        next.push((x - i, y - j, z - k));
                        cube[x - i][y - j][z - k] = State::Water;
                    }
                }
            }
        }
    }
    surface
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day18.txt";
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
