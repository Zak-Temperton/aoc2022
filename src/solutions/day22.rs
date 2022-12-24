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

pub(crate) fn part2(text: &str) -> i64 {
    let mut lines = text.lines();
    let mut map = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        map.push(line.as_bytes());
    }
    let instructions = lines.next().unwrap().as_bytes();
    todo!("Need complete Rework of thinking")
}
