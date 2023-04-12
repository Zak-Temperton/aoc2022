pub fn part1(text: &str) -> i32 {
    let mut cycle = 1;
    let mut x = 1;
    let mut test_cycle = 20;
    let mut sum = 0;
    for line in text.lines() {
        match &line[..1] {
            "n" => {
                cycle += 1;
                if cycle == test_cycle {
                    sum += test_cycle * x;
                    test_cycle += 40;
                }
            }
            "a" => {
                cycle += 2;
                if cycle >= test_cycle {
                    sum += test_cycle * x;
                    test_cycle += 40;
                }
                x += line[5..].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    sum
}

fn draw(sprite: &mut [Vec<u8>], cycle: &mut usize, x: i32, y: &mut usize) {
    if (*cycle as i32).abs_diff(x) <= 1 {
        sprite[*y][*cycle] = b'#';
    }
    *cycle += 1;
    if *cycle == 40 {
        *cycle = 0;
        *y += 1;
    }
}

pub fn part2(text: &str) -> Vec<Vec<u8>> {
    let mut sprite = vec![vec![b'.'; 40]; 6];
    let mut cycle = 0;
    let mut x = 1;
    let mut y = 0;

    for line in text.split_whitespace() {
        match &line[..1] {
            "n" | "a" => {
                draw(&mut sprite, &mut cycle, x, &mut y);
            }
            _ => {
                draw(&mut sprite, &mut cycle, x, &mut y);
                x += line.parse::<i32>().unwrap();
            }
        }
    }
    sprite
}

#[cfg(test)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day10.txt";
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
