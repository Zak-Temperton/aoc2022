use std::collections::VecDeque;

struct Cave {
    map: VecDeque<Vec<bool>>,
    offset: isize,
    depth: isize,
}

impl From<&str> for Cave {
    fn from(text: &str) -> Self {
        let mut cave = Self::new();
        for line in text.lines() {
            let mut split = line.split(" -> ");
            let mut last = get_pos(&mut split).unwrap();
            if last.1 > cave.depth {
                cave.depth = last.1;
            }
            while let Some(new) = get_pos(&mut split) {
                if last.0 == new.0 {
                    if new.1 > cave.depth {
                        cave.depth = new.1;
                    }
                    for y in (last.1.min(new.1))..=last.1.max(new.1) {
                        cave.insert(last.0, y as usize);
                    }
                } else {
                    for x in (last.0.min(new.0))..=last.0.max(new.0) {
                        cave.insert(x, last.1 as usize);
                    }
                }
                last = new;
            }
        }
        cave
    }
}

impl Cave {
    pub fn new() -> Self {
        Self {
            map: VecDeque::from([Vec::new()]),
            offset: 500,
            depth: 0,
        }
    }

    pub fn add_sand(&mut self, sand: (isize, isize), count: &mut usize) {
        for dx in [0, -1, 1] {
            let (x, y) = (sand.0 + dx, sand.1 + 1);
            if !self.get(x, y as usize) && y != self.depth {
                self.insert(x, y as usize);
                *count += 1;
                self.add_sand((x, y), count);
            }
        }
    }

    pub fn get(&self, x: isize, y: usize) -> bool {
        if x - self.offset < 0 || x - self.offset >= self.map.len() as isize {
            return false;
        }
        let col = &self.map[(x - self.offset) as usize];
        if y >= col.len() {
            return false;
        }
        col[y]
    }

    pub fn insert(&mut self, x: isize, y: usize) {
        while x - self.offset < 0 {
            self.map.push_front(Vec::new());
            self.offset -= 1;
        }
        while x - self.offset >= self.map.len() as isize {
            self.map.push_back(Vec::new());
        }
        let col = &mut self.map[(x - self.offset) as usize];
        if y >= col.len() {
            col.resize(y + 1, false);
        }
        col[y] = true;
    }
}

fn get_pos(split: &mut std::str::Split<&str>) -> Option<(isize, isize)> {
    let mut last = split.next()?.split(',');
    Some((last.next()?.parse().unwrap(), last.next()?.parse().unwrap()))
}

pub fn part1(text: &str) -> usize {
    let mut cave = Cave::from(text);
    let mut count = 0;
    'abyss: loop {
        let mut sand = (500, 0);
        'settle: loop {
            for dx in [0, -1, 1] {
                let (x, y) = (sand.0 + dx, sand.1 + 1);
                if !cave.get(x, y as usize) {
                    sand = (x, y);
                    if sand.1 > cave.depth {
                        break 'abyss;
                    }
                    continue 'settle;
                }
            }
            count += 1;
            cave.insert(sand.0, sand.1 as usize);
            break 'settle;
        }
    }
    count
}

pub fn part2(text: &str) -> usize {
    let mut cave = Cave::from(text);
    cave.depth += 2;
    let mut count = 1;
    cave.add_sand((500, 0), &mut count);
    count
}

#[cfg(test)]
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
