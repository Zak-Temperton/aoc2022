use std::collections::{HashMap, HashSet};

pub(crate) fn part1(text: &str) -> usize {
    let mut map = Vec::new();
    let mut count = 0;
    for line in text.lines() {
        map.push(Vec::new());
        for c in line.chars() {
            if c == '#' {
                count += 1;
            }
            map.last_mut().unwrap().push(c == '#');
        }
    }
    let mut minx;
    let mut miny;
    let mut width = 0;

    let mut plans: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut failed_plans = HashSet::new();
    for i in 0..10 {
        minx = usize::MAX;
        miny = usize::MAX;
        width = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, elf) in row.iter().enumerate() {
                if *elf {
                    let plan = create_plan(x, y, &map, i);
                    if !failed_plans.contains(&plan) {
                        if plans.contains_key(&plan) {
                            let failed = plans.remove(&plan).unwrap();
                            failed_plans.insert(plan);
                            plans.insert(failed, failed);
                            plans.insert((x + 1, y + 1), (x + 1, y + 1));
                        } else {
                            plans.insert(plan, (x + 1, y + 1));
                        }
                    } else {
                        plans.insert((x + 1, y + 1), (x + 1, y + 1));
                    }
                }
            }
        }
        let mut new_map = Vec::new();
        for ((x, y), _) in plans.drain() {
            if y >= new_map.len() {
                new_map.extend(vec![Vec::new(); y - new_map.len() + 1])
            }
            if x >= new_map[y].len() {
                let len = new_map[y].len();
                new_map[y].extend(vec![false; x - len + 1])
            }
            new_map[y][x] = true;
            if x < minx {
                minx = x;
            }
            if y < miny {
                miny = y;
            }
        }
        new_map = new_map[miny..].to_vec();
        for row in &mut new_map {
            if !row.is_empty() {
                *row = row[minx..].to_vec();
                if width < row.len() {
                    width = row.len();
                }
            }
        }
        map = new_map;
        failed_plans.clear();
    }
    width * map.len() - count
}

fn create_plan(x: usize, y: usize, map: &[Vec<bool>], i: usize) -> (usize, usize) {
    let n = is_elf(map, x, (y as isize - 1) as usize);
    let nw = is_elf(map, (x as isize - 1) as usize, (y as isize - 1) as usize);
    let w = is_elf(map, (x as isize - 1) as usize, y);
    let sw = is_elf(map, (x as isize - 1) as usize, y + 1);
    let s = is_elf(map, x, y + 1);
    let se = is_elf(map, x + 1, y + 1);
    let e = is_elf(map, x + 1, y);
    let ne = is_elf(map, x + 1, (y as isize - 1) as usize);
    if !(n || nw || w || sw || s || se || e || ne) {
        return (x + 1, y + 1);
    }
    for j in 0..4 {
        match ((i % 4) + j) % 4 {
            0 => {
                if !(n || ne || nw) {
                    return (x + 1, y);
                }
            }
            1 => {
                if !(s || se || sw) {
                    return (x + 1, y + 2);
                }
            }
            2 => {
                if !(w || nw || sw) {
                    return (x, y + 1);
                }
            }
            3 => {
                if !(e || ne || se) {
                    return (x + 2, y + 1);
                }
            }
            _ => {}
        }
    }
    return (x + 1, y + 1);
}

fn is_elf(map: &[Vec<bool>], x: usize, y: usize) -> bool {
    *map.get(y).and_then(|row| row.get(x)).unwrap_or(&false)
}

pub(crate) fn part2(text: &str) -> usize {
    let mut map = Vec::new();
    for line in text.lines() {
        map.push(Vec::new());
        for c in line.chars() {
            map.last_mut().unwrap().push(c == '#');
        }
    }
    let mut minx;
    let mut miny;
    let mut width;

    let mut plans: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut failed_plans = HashSet::new();
    let mut moving = true;
    let mut i = 0;
    while moving {
        moving = false;
        minx = usize::MAX;
        miny = usize::MAX;
        width = 0;
        for (y, row) in map.iter().enumerate() {
            for (x, elf) in row.iter().enumerate() {
                if *elf {
                    let (plan, moved) = create_planb(x, y, &map, i);
                    if !moving && moved {
                        moving = true;
                    }
                    if !failed_plans.contains(&plan) {
                        if plans.contains_key(&plan) {
                            let failed = plans.remove(&plan).unwrap();
                            failed_plans.insert(plan);
                            plans.insert(failed, failed);
                            plans.insert((x + 1, y + 1), (x + 1, y + 1));
                        } else {
                            plans.insert(plan, (x + 1, y + 1));
                        }
                    } else {
                        plans.insert((x + 1, y + 1), (x + 1, y + 1));
                    }
                }
            }
        }
        let mut new_map = Vec::new();
        for ((x, y), _) in plans.drain() {
            if y >= new_map.len() {
                new_map.extend(vec![Vec::new(); y - new_map.len() + 1])
            }
            if x >= new_map[y].len() {
                let len = new_map[y].len();
                new_map[y].extend(vec![false; x - len + 1])
            }
            new_map[y][x] = true;
            if x < minx {
                minx = x;
            }
            if y < miny {
                miny = y;
            }
        }
        new_map = new_map[miny..].to_vec();
        for row in &mut new_map {
            if !row.is_empty() {
                *row = row[minx..].to_vec();
                if width < row.len() {
                    width = row.len();
                }
            }
        }
        map = new_map;
        failed_plans.clear();
        i += 1;
    }
    i
}

fn create_planb(x: usize, y: usize, map: &[Vec<bool>], i: usize) -> ((usize, usize), bool) {
    let n = is_elf(map, x, (y as isize - 1) as usize);
    let nw = is_elf(map, (x as isize - 1) as usize, (y as isize - 1) as usize);
    let w = is_elf(map, (x as isize - 1) as usize, y);
    let sw = is_elf(map, (x as isize - 1) as usize, y + 1);
    let s = is_elf(map, x, y + 1);
    let se = is_elf(map, x + 1, y + 1);
    let e = is_elf(map, x + 1, y);
    let ne = is_elf(map, x + 1, (y as isize - 1) as usize);
    if !(n || nw || w || sw || s || se || e || ne) {
        return ((x + 1, y + 1), false);
    }
    for j in 0..4 {
        match ((i % 4) + j) % 4 {
            0 => {
                if !(n || ne || nw) {
                    return ((x + 1, y), true);
                }
            }
            1 => {
                if !(s || se || sw) {
                    return ((x + 1, y + 2), true);
                }
            }
            2 => {
                if !(w || nw || sw) {
                    return ((x, y + 1), true);
                }
            }
            3 => {
                if !(e || ne || se) {
                    return ((x + 2, y + 1), true);
                }
            }
            _ => {}
        }
    }
    return ((x + 1, y + 1), false);
}
