pub fn part1(text: &str) -> u64 {
    let mut ranges = Vec::new();
    for line in text.lines() {
        let mut nums = line
            .split(|c| !"-0123456789".contains(c))
            .filter(|num| !num.is_empty())
            .flat_map(str::parse::<i64>);
        let sensor = (nums.next().unwrap(), nums.next().unwrap());
        let beakon = (nums.next().unwrap(), nums.next().unwrap());
        let reach = (sensor.0.abs_diff(beakon.0) + sensor.1.abs_diff(beakon.1)) as i64
            - sensor.1.abs_diff(2_000_000) as i64;
        if reach > 0 {
            let left = sensor.0 - reach;
            let right = sensor.0 + reach;
            ranges.push((left, right));
        }
    }
    ranges.sort_unstable_by_key(|r| r.0);
    let mut count = 0;
    let mut max = i64::MIN;
    for range in ranges {
        if range.1 > max {
            if range.0 > max {
                count += range.1.abs_diff(range.0);
            } else {
                count += range.1.abs_diff(max);
            }
            max = range.1;
        }
    }
    count
}

pub fn part2(text: &str) -> i64 {
    let pairs = text
        .lines()
        .map(|line| {
            line.split(|c| !"-0123456789".contains(c))
                .filter(|num| !num.is_empty())
                .flat_map(str::parse::<i64>)
        })
        .map(|mut nums| {
            (
                (nums.next().unwrap(), nums.next().unwrap()),
                (nums.next().unwrap(), nums.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();
    //Beakon must be touching the border of at least one Sensor
    //due to there being only one possible spot in the given range not covered by Sensors
    for &((x, y), (dx, dy)) in &pairs {
        let dist = (x.abs_diff(dx) + y.abs_diff(dy)) as i64;
        for (dir_x, dir_y) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
            for d in 0..dist {
                let bx = x + dir_x * d;
                let by = y + dir_y * (dist + 1 - d);
                if bx < 0 || by < 0 || bx > 4_000_000 || by > 4_000_000 {
                    break;
                }
                let mut found = true;
                for &((x, y), (dx, dy)) in &pairs {
                    let range = x.abs_diff(dx) + y.abs_diff(dy);
                    let distance = x.abs_diff(bx) + y.abs_diff(by);
                    if distance < range {
                        found = false;
                        break;
                    }
                }
                if found {
                    return bx * 4_000_000 + by;
                }
            }
        }
    }
    unreachable!()
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day15.txt";
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
