fn loops(posx: usize, posy: usize, dir: usize) -> (usize, usize) {
    match (posx / 50, posy / 50, dir) {
        (1, 0, 2) => (149, posy),
        (1, 0, 3) => (posx, 149),
        (2, 0, 0) => (50, posy),
        (2, 0, 1) => (posx, 0),
        (2, 0, 3) => (posx, 49),
        (1, 1, 0) => (50, posy),
        (1, 1, 2) => (99, posy),
        (0, 2, 2) => (99, posy),
        (0, 2, 3) => (posx, 199),
        (1, 2, 1) => (posx, 0),
        (1, 2, 0) => (0, posy),
        (0, 3, 0) => (0, posy),
        (0, 3, 1) => (posx, 100),
        (0, 3, 2) => (49, posy),
        _ => unreachable!(),
    }
}

fn take_steps(map: &[&[u8]], steps: &mut usize, dir: usize, posx: &mut usize, posy: &mut usize) {
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
                let (newx, newy) = loops(*posx, *posy, dir);
                if map[newy][newx] == b'#' {
                    break;
                }
                (*posx, *posy) = (newx, newy);
            }
            _ => unreachable!(),
        }
    }
    *steps = 0;
}

pub(crate) fn part1(text: &str) -> usize {
    let mut lines = text.lines();
    let mut map = Vec::new();
    for line in lines.by_ref() {
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
                take_steps(&map, &mut steps, facing, &mut posx, &mut posy);
                facing = (facing as isize - 1).rem_euclid(4) as usize;
            }
            b'R' => {
                take_steps(&map, &mut steps, facing, &mut posx, &mut posy);
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

fn connections(posx: usize, posy: usize, dir: usize) -> (usize, usize, usize) {
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
                let (newx, newy, new_dir) = connections(*posx, *posy, *dir);
                if map[newy][newx] == b'#' {
                    break;
                }
                (*posx, *posy, *dir) = (newx, newy, new_dir);
            }
            _ => unreachable!(),
        }
    }
    *steps = 0;
}

pub(crate) fn part2(text: &str) -> usize {
    let mut lines = text.lines();
    let mut map = Vec::new();
    for line in lines.by_ref() {
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
