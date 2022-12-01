pub(crate) fn part1(text: &str) -> i32 {
    let mut max = 0;
    let mut cur = 0;
    for line in text.lines() {
        if line.is_empty() {
            if cur > max {
                max = cur;
            }
            cur = 0;
        } else {
            cur += line.parse::<i32>().unwrap();
        }
    }
    max
}

pub(crate) fn part2(text: &str) -> i32 {
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut cur = 0;
    for line in text.lines() {
        if line.is_empty() {
            if cur >= first {
                third = second;
                second = first;
                first = cur;
            } else if cur >= second {
                third = second;
                second = cur;
            } else if cur > third {
                third = cur;
            }
            cur = 0;
        } else {
            cur += line.parse::<i32>().unwrap();
        }
    }
    first + second + third
}
