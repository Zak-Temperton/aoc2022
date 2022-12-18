use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Rock {
    shape: [[bool; 4]; 4],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

const ROCKS: [Rock; 5] = [
    Rock {
        shape: [[true; 4], [false; 4], [false; 4], [false; 4]],
        x: 2,
        y: 0,
        width: 4,
        height: 1,
    },
    Rock {
        shape: [
            [false, true, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false; 4],
        ],
        x: 2,
        y: 0,
        width: 3,
        height: 3,
    },
    Rock {
        shape: [
            [false, false, true, false],
            [false, false, true, false],
            [true, true, true, false],
            [false; 4],
        ],
        x: 2,
        y: 0,
        width: 3,
        height: 3,
    },
    Rock {
        shape: [[true, false, false, false]; 4],
        x: 2,
        y: 0,
        width: 1,
        height: 4,
    },
    Rock {
        shape: [
            [true, true, false, false],
            [true, true, false, false],
            [false; 4],
            [false; 4],
        ],
        x: 2,
        y: 0,
        width: 2,
        height: 2,
    },
];

fn collide(chamber: &[[bool; 7]], rock: &Rock, arg_1: isize, arg_2: isize) -> bool {
    for x in 0..rock.width {
        for y in 0..rock.height {
            if !rock.shape[y][x] {
                continue;
            }
            if let Some(row) = chamber.get((rock.y as isize + arg_2) as usize - y) {
                if *row
                    .get((rock.x as isize + arg_1) as usize + x)
                    .unwrap_or(&false)
                {
                    return true;
                }
            }
        }
    }
    false
}

fn drop_rock(
    count: usize,
    highest: &mut usize,
    chamber: &mut Vec<[bool; 7]>,
    text: &str,
    t: &mut usize,
) {
    let mut rock = ROCKS[count % ROCKS.len()].clone();
    rock.y = *highest + rock.height + 3;
    if rock.y >= chamber.len() {
        chamber.extend(vec![[false; 7]; rock.y - chamber.len()]);
    }
    loop {
        match text.as_bytes()[*t % text.len()] {
            b'<' => {
                if rock.x != 0 && !collide(&*chamber, &rock, -1, 0) {
                    rock.x -= 1;
                }
            }
            b'>' => {
                if rock.x + rock.width < 7 && !collide(&*chamber, &rock, 1, 0) {
                    rock.x += 1;
                }
            }
            _ => {
                panic!()
            }
        }
        *t += 1;
        if rock.y == rock.height || collide(&*chamber, &rock, 0, -1) {
            break;
        }
        rock.y -= 1;
    }
    if rock.y > *highest {
        *highest = rock.y;
    }
    for x in 0..rock.width {
        for y in 0..rock.height {
            chamber[rock.y - y][rock.x + x] |= rock.shape[y][x];
        }
    }
}

pub(crate) fn part1(text: &str) -> usize {
    let mut chamber = vec![[false; 7]];
    let mut highest = 0;
    let mut count = 0;
    let mut t = 0;
    while count < 2022 {
        drop_rock(count, &mut highest, &mut chamber, text, &mut t);
        count += 1;
    }
    highest
}

fn column_heights(map: &[[bool; 7]], highest: usize) -> [usize; 7] {
    let mut heights = [0; 7];
    for i in 0..7 {
        heights[i] = (0..highest)
            .find(|&x| map[highest - x][i])
            .unwrap_or(usize::MAX);
    }
    heights
}

pub(crate) fn part2(text: &str) -> usize {
    let mut chamber = vec![[false; 7]];
    let mut highest = 0;
    let mut total_height = 0;
    let mut t = 0;
    let mut count = 0usize;
    let mut cache = HashMap::new();
    while count <= 1_000_000_000_000 {
        drop_rock(count, &mut highest, &mut chamber, text, &mut t);

        let key = (
            count % ROCKS.len(),
            t % text.len(),
            column_heights(&chamber, highest),
        );
        if let Some((idx, height)) = cache.get(&key) {
            let repeats = (1_000_000_000_000 - idx) / (count - idx) - 1;
            count += (count - idx) * repeats;
            total_height += (highest - height) * repeats;
        } else {
            cache.insert(key, (count, highest));
        }
        count += 1;
    }

    total_height + highest - 1
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day17.txt";
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
