use std::{collections::HashMap, str::Lines};

struct Directory {
    size: usize,
    pub children: HashMap<String, Directory>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            size: 0,
            children: HashMap::new(),
        }
    }
}

fn create_directories(dir: &mut Directory, lines: &mut Lines) -> usize {
    let mut sum = 0;
    while let Some(intruction) = lines.next() {
        let mut split = intruction.split_whitespace();
        match (split.next().unwrap(), split.next().unwrap()) {
            ("$", "cd") => match split.next().unwrap() {
                ".." => {
                    dir.size += sum;
                    return sum;
                }
                n => {
                    sum += create_directories(
                        dir.children
                            .entry(n.to_string())
                            .or_insert_with(Directory::new),
                        lines,
                    );
                }
            },
            ("$", _) => {}
            ("dir", n) => {
                let _ = dir.children.try_insert(n.to_string(), Directory::new());
            }
            (s, _) => sum += s.parse::<usize>().unwrap(),
        }
    }
    dir.size += sum;
    sum
}

fn sum_small_dir(dir: &Directory, sum: &mut usize) {
    for child in dir.children.values() {
        if child.size < 100_000 {
            *sum += child.size;
        }
        sum_small_dir(child, sum);
    }
}

pub fn part1(text: &str) -> usize {
    let mut lines = text.lines();
    let mut dummy = Directory::new();
    create_directories(&mut dummy, &mut lines);
    let mut size = 0;
    sum_small_dir(&dummy, &mut size);
    size
}

fn find_min(dir: &Directory, space_needed: usize, min: &mut usize) {
    for (_, child) in dir.children.iter().filter(|(_, c)| c.size >= space_needed) {
        if child.size < *min {
            *min = child.size;
        }
        find_min(child, space_needed, min);
    }
}

pub fn part2(text: &str) -> usize {
    let mut lines = text.lines();
    let mut dummy = Directory::new();
    create_directories(&mut dummy, &mut lines);
    let space_needed = 30_000_000 - (70_000_000 - dummy.size);
    let mut min = usize::MAX;
    find_min(&dummy, space_needed, &mut min);
    min
}

#[cfg(test)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day07.txt";
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
