#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos(usize, usize);

fn init_map(text: &str) -> (Vec<Vec<u8>>, Pos, Pos) {
    let mut map = Vec::new();
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    for (y, line) in text.lines().enumerate() {
        let mut bytes = line.as_bytes().to_vec();
        for (x, b) in bytes.iter_mut().enumerate() {
            if *b == b'S' {
                *b = b'a';
                start = Pos(x, y);
            } else if *b == b'E' {
                *b = b'z';
                end = Pos(x, y);
            }
        }
        map.push(bytes);
    }
    (map, start, end)
}

fn test_cell(
    map: &[Vec<u8>],
    cell: &Pos,
    new_pos: Pos,
    cells: &mut [Vec<bool>],
    end: Pos,
    steps: usize,
    new: &mut Vec<Pos>,
) -> Option<usize> {
    if map[cell.1][cell.0] + 1 >= map[new_pos.1][new_pos.0] && !cells[new_pos.1][new_pos.0] {
        if new_pos == end {
            return Some(steps);
        }
        cells[new_pos.1][new_pos.0] = true;
        new.push(new_pos);
    }
    None
}

fn bfs(map: &[Vec<u8>], mut start: Vec<Pos>, end: Pos, cells: &mut [Vec<bool>]) -> usize {
    let mut steps = 0;
    while !start.is_empty() {
        let mut new = Vec::new();
        steps += 1;
        for i in [-1, 1] {
            for cell in &start {
                if !(cell.0 == 0 && i == -1 || cell.0 == map[0].len() - 1 && i == 1) {
                    let new_pos = Pos((cell.0 as isize + i) as usize, cell.1);
                    if let Some(value) = test_cell(map, cell, new_pos, cells, end, steps, &mut new)
                    {
                        return value;
                    }
                }
                if !(cell.1 == 0 && i == -1 || cell.1 == map.len() - 1 && i == 1) {
                    let new_pos = Pos(cell.0, (cell.1 as isize + i) as usize);
                    if let Some(value) = test_cell(map, cell, new_pos, cells, end, steps, &mut new)
                    {
                        return value;
                    }
                }
            }
        }
        start = new;
    }
    panic!("No path found")
}

fn test_cell_b(
    cells: &mut [Vec<bool>],
    new_pos: Pos,
    map: &[Vec<u8>],
    starts: &mut Vec<Pos>,
    new: &mut Vec<Pos>,
) {
    if !cells[new_pos.1][new_pos.0] {
        cells[new_pos.1][new_pos.0] = true;
        if map[new_pos.1][new_pos.0] == b'b' {
            starts.push(new_pos);
        } else if map[new_pos.1][new_pos.0] == b'a' {
            new.push(new_pos);
        }
    }
}

fn find_b(map: &[Vec<u8>], mut start: Vec<Pos>, cells: &mut [Vec<bool>]) -> Vec<Pos> {
    let mut starts = Vec::new();
    while !start.is_empty() {
        let mut new = Vec::new();
        for i in [-1, 1] {
            for cell in &start {
                if !(cell.0 == 0 && i == -1 || cell.0 == map[0].len() - 1 && i == 1) {
                    let new_pos = Pos((cell.0 as isize + i) as usize, cell.1);
                    test_cell_b(cells, new_pos, map, &mut starts, &mut new);
                }
                if !(cell.1 == 0 && i == -1 || cell.1 == map.len() - 1 && i == 1) {
                    let new_pos = Pos(cell.0, (cell.1 as isize + i) as usize);
                    test_cell_b(cells, new_pos, map, &mut starts, &mut new);
                }
            }
        }
        start = new;
    }
    starts
}

pub fn part1(text: &str) -> usize {
    let (map, start, end) = init_map(text);
    let mut cells = vec![vec![false; map[0].len()]; map.len()];
    bfs(&map, vec![start], end, &mut cells)
}

pub fn part2(text: &str) -> usize {
    let (map, start, end) = init_map(text);
    let mut cells = vec![vec![false; map[0].len()]; map.len()];
    let starts = find_b(&map, vec![start], &mut cells);
    bfs(&map, starts, end, &mut cells) + 1
}

#[cfg(test)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day12.txt";
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
