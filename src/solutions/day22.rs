fn take_steps(
    steps: usize,
    facing: u8,
    map: &[(usize, Vec<bool>)],
    posy: &mut usize,
    posx: &mut usize,
    yoffsets: &[usize],
    heights: &[usize],
) {
    match facing {
        0 => {
            let oldx = *posx;
            for s in 1..=steps {
                let offset = map[*posy].0;
                let len = map[*posy].1.len();
                let x = (oldx - offset + s).rem_euclid(len);
                if map[*posy].1[x] {
                    *posx = x + offset;
                } else {
                    break;
                }
            }
        }
        1 => {
            let oldy = *posy;
            for s in 1..=steps {
                let y = (oldy + s - yoffsets[*posx]).rem_euclid(heights[*posx] - yoffsets[*posx])
                    + yoffsets[*posx];
                let offset = map[y].0;
                if map[y].1[*posx - offset] {
                    *posy = y;
                } else {
                    break;
                }
            }
        }
        2 => {
            let oldx = *posx;
            for s in 1..=steps {
                let offset = map[*posy].0;
                let len = map[*posy].1.len();
                let x = (oldx as isize - offset as isize - s as isize).rem_euclid(len as isize)
                    as usize;
                if map[*posy].1[x] {
                    *posx = x + offset;
                } else {
                    break;
                }
            }
        }
        3 => {
            let oldy = *posy as isize;
            for s in 1..=steps {
                let y = (oldy - s as isize - yoffsets[*posx] as isize)
                    .rem_euclid((heights[*posx] - yoffsets[*posx]) as isize)
                    as usize
                    + yoffsets[*posx];
                let offset = map[y].0;
                if map[y].1[*posx - offset] {
                    *posy = y;
                } else {
                    break;
                }
            }
        }
        _ => panic!(),
    }
}

pub(crate) fn part1(text: &str) -> usize {
    let mut map = vec![];
    let lines = text.lines().enumerate();
    let mut instructions = "";
    let mut width = 0;
    for (y, line) in lines {
        if y < 200 {
            map.push((0, vec![]));
            for c in line.chars() {
                if c == '.' {
                    map.last_mut().unwrap().1.push(true);
                } else if c == '#' {
                    map.last_mut().unwrap().1.push(false);
                } else {
                    map.last_mut().unwrap().0 += 1;
                }
            }
            if map.last().unwrap().1.len() + map.last_mut().unwrap().0 > width {
                width = map.last().unwrap().1.len() + map.last_mut().unwrap().0;
            }
        } else if y == 201 {
            instructions = line;
        }
    }

    let mut yoffsets = vec![0; width];
    for i in 0..width {
        for (j, (o, y)) in map.iter().enumerate() {
            if i >= *o && y.get(i - *o).is_some() {
                yoffsets[i] = j;
                break;
            }
        }
    }
    let mut heights = vec![0; width];
    for i in 0..width {
        for (j, (o, y)) in map.iter().enumerate().rev() {
            if i >= *o && y.get(i - *o).is_some() {
                heights[i] = j;
                break;
            }
        }
    }
    let (mut posx, mut posy): (usize, usize) = (map[0].0, 0);
    let mut facing: u8 = 0;
    let mut steps = 0;

    for c in instructions.bytes() {
        match c {
            b'L' => {
                //take steps then turn left
                take_steps(
                    steps, facing, &map, &mut posy, &mut posx, &yoffsets, &heights,
                );
                facing = (facing as i8 - 1).rem_euclid(4) as u8;
                steps = 0;
            }
            b'R' => {
                //take steps then turn right
                take_steps(
                    steps, facing, &map, &mut posy, &mut posx, &yoffsets, &heights,
                );
                facing = (facing + 1) % 4;
                steps = 0;
            }
            b => {
                steps = steps * 10 + (b - b'0') as usize;
            }
        }
    }
    //take final steps
    take_steps(
        steps, facing, &map, &mut posy, &mut posx, &yoffsets, &heights,
    );

    1000 * (posy + 1) + 4 * (posx + 1) + facing as usize
}

fn connections(dir: usize, posx: usize, posy: usize) -> (usize, usize, usize) {
    let (nx, ny, ndir) = match (posx / 50, posy / 50, dir) {
        (1, 0, 2) => (0, 2, 0),
        (1, 0, 3) => (0, 3, 0),
        (2, 0, 0) => (1, 2, 2),
        (2, 0, 1) => (1, 1, 2),
        (2, 0, 3) => (0, 3, 3),
        (1, 1, 0) => (2, 0, 3),
        (1, 1, 2) => (0, 2, 1),
        (0, 2, 2) => (1, 0, 0),
        (0, 2, 3) => (1, 1, 0),
        (1, 2, 1) => (0, 3, 2),
        (1, 2, 0) => (2, 0, 2),
        (0, 3, 0) => (1, 2, 3),
        (0, 3, 1) => (2, 0, 1),
        (0, 3, 2) => (1, 0, 1),
        _ => unreachable!(),
    };
    let (grid_x, grid_y) = (posx % 50, posy % 50);
    let i = [49 - grid_y, grid_x, grid_y, 49 - grid_x][dir];
    let (ox, oy) = [(0, 49 - i), (i, 0), (49, i), (49 - i, 49)][ndir];
    (nx * 50 + ox, ny * 50 + oy, ndir)
}

fn take_steps_on_cube(
    map: &[&[u8]],
    steps: &mut usize,
    dir: &mut usize,
    posx: &mut usize,
    posy: &mut usize,
) {
    for _ in 0..*steps {
        let (nx, ny) = match dir {
            0 => (*posx + 1, *posy),
            1 => (*posx, *posy + 1),
            2 => (posx.wrapping_sub(1), *posy),
            3 => (*posx, posy.wrapping_sub(1)),
            _ => unreachable!(),
        };
        match map.get(ny).and_then(|row| row.get(nx)).unwrap_or(&b' ') {
            b'.' => (*posx, *posy) = (nx, ny),
            b'#' => break,
            b' ' => {
                let (nr, nc, d) = connections(*dir, *posx, *posy);

                if map[nc][nr] == b'#' {
                    break;
                }
                (*posx, *posy, *dir) = (nr, nc, d);
            }
            _ => unreachable!(),
        }
    }
    *steps = 0;
}

pub(crate) fn part2(text: &str) -> usize {
    let mut lines = text.lines();
    let mut map = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        map.push(line.as_bytes());
    }
    let instructions = lines.next().unwrap().as_bytes();

    let (mut posx, mut posy) = (50, 0);
    let mut facing = 0;
    let mut steps = 0;
    for c in instructions.iter() {
        match c {
            b'L' => {
                take_steps_on_cube(&map, &mut steps, &mut facing, &mut posx, &mut posy);
                facing = (facing as isize - 1).rem_euclid(4) as usize;
            }
            b'R' => {
                take_steps_on_cube(&map, &mut steps, &mut facing, &mut posx, &mut posy);
                facing = (facing + 1) % 4;
            }
            b => {
                steps = steps * 10 + (b - b'0') as usize;
            }
        }
    }
    take_steps_on_cube(&map, &mut steps, &mut facing, &mut posx, &mut posy);

    1000 * (posy + 1) + 4 * (posx + 1) + facing
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day22.txt";
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
