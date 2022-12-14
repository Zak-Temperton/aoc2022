use std::collections::HashSet;

pub(crate) fn part1(text: &str) -> usize {
    let mut cave = HashSet::new();
    let mut deepest = 0;
    for line in text.lines() {
        let mut split = line.split(" -> ");
        let mut last = get_pos(&mut split).unwrap();
        if last.1 > deepest {
            deepest = last.1;
        }
        while let Some(new) = get_pos(&mut split) {
            if last.0 == new.0 {
                if new.1 > deepest {
                    deepest = new.1;
                }
                for y in (last.1.min(new.1))..=last.1.max(new.1) {
                    cave.insert((last.0, y));
                }
            } else {
                for x in (last.0.min(new.0))..=last.0.max(new.0) {
                    cave.insert((x, last.1));
                }
            }
            last = new;
        }
    }
    let mut count = 0;
    'abyss: loop {
        let mut sand = (500, 0);
        'settle: loop {
            for c in [(0, 1), (-1, 1), (1, 1)] {
                let new = (sand.0 + c.0, sand.1 + c.1);
                if !cave.contains(&new) {
                    sand = new;
                    if sand.1 > deepest {
                        break 'abyss;
                    }
                    continue 'settle;
                }
            }
            count += 1;
            cave.insert(sand);
            break 'settle;
        }
    }
    count
}

fn get_pos(split: &mut std::str::Split<&str>) -> Option<(isize, isize)> {
    let mut last = split.next()?.split(',');
    Some((last.next()?.parse().unwrap(), last.next()?.parse().unwrap()))
}

pub(crate) fn part2(text: &str) -> usize {
    let mut cave = HashSet::new();
    let mut floor = 0;
    for line in text.lines() {
        let mut split = line.split(" -> ");
        let mut last = get_pos(&mut split).unwrap();
        if last.1 > floor {
            floor = last.1;
        }
        while let Some(new) = get_pos(&mut split) {
            if last.0 == new.0 {
                if new.1 > floor {
                    floor = new.1;
                }
                for y in (last.1.min(new.1))..=last.1.max(new.1) {
                    cave.insert((last.0, y));
                }
            } else {
                for x in (last.0.min(new.0))..=last.0.max(new.0) {
                    cave.insert((x, last.1));
                }
            }
            last = new;
        }
    }
    floor += 2;
    let mut count = 1;
    add_sand((500, 0), floor, &mut cave, &mut count);
    count
}

fn add_sand(
    sand: (isize, isize),
    floor: isize,
    cave: &mut HashSet<(isize, isize)>,
    count: &mut usize,
) {
    for c in [(0, 1), (-1, 1), (1, 1)] {
        let new = (sand.0 + c.0, sand.1 + c.1);
        if !cave.contains(&new) && new.1 != floor {
            cave.insert(new);
            *count += 1;
            add_sand(new, floor, cave, count)
        }
    }
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day14.txt";
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
