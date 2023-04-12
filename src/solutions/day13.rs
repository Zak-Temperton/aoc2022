use std::{cmp::Ordering, str::Bytes};

#[derive(Debug, Clone)]
enum Element {
    List(Vec<Element>),
    Val(u32),
    Empty,
}

impl Element {
    fn new_element(&mut self, bytes: &mut Bytes) {
        *self = Self::List(vec![Self::Empty]);

        if let Self::List(list) = self {
            while let Some(b) = bytes.next() {
                match b {
                    b'[' => list.last_mut().unwrap().new_element(bytes),
                    b']' => return,
                    b',' => list.push(Self::Empty),
                    b if b.is_ascii_digit() => match list.last_mut() {
                        Some(Self::Val(v)) => *v = *v * 10 + (b - b'0') as u32,
                        Some(Self::Empty) => {
                            *list.last_mut().unwrap() = Self::Val((b - b'0') as u32);
                        }
                        _ => panic!(),
                    },
                    _ => {}
                }
            }
        }
    }

    fn first_val(&self) -> Option<u32> {
        match self {
            Self::List(l) => l[0].first_val(),
            Self::Val(v) => Some(*v),
            Self::Empty => None,
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Val(l0), Self::Val(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Element {}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Self::List(l), Self::List(r)) => {
                let mut l = l.iter();
                let mut r = r.iter();
                loop {
                    let m = (l.next(), r.next());
                    match m {
                        (None, None) => {
                            return Some(Ordering::Equal);
                        }
                        (None, _) => return Some(Ordering::Less),
                        (_, None) => return Some(Ordering::Greater),
                        (Some(l), Some(r)) if l > r => return Some(Ordering::Greater),
                        (Some(l), Some(r)) if l < r => return Some(Ordering::Less),
                        _ => {}
                    }
                }
            }
            (Self::List(_), Self::Val(_)) => self.cmp(&Self::List(vec![other.clone()])),
            (Self::Val(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::Val(l), Self::Val(r)) => l.cmp(r),
            (Self::Val(_) | Self::List(_), Self::Empty) => Ordering::Greater,
            (Self::Empty, Self::List(_) | Self::Val(_)) => Ordering::Less,
            (Self::Empty, Self::Empty) => Ordering::Equal,
        })
    }
}

pub fn part1(text: &str) -> usize {
    let mut sum = 0;
    for (i, lines) in text.lines().array_chunks::<3>().enumerate() {
        let mut left = Element::Empty;
        let mut right = Element::Empty;
        left.new_element(&mut lines[0].bytes());
        right.new_element(&mut lines[1].bytes());
        if left <= right {
            sum += i + 1;
        }
    }
    sum
}

pub fn part2(text: &str) -> usize {
    let low = 2;
    let high = 6;
    let mut low_index = 1;
    let mut high_index = 2;
    for line in text.split_whitespace() {
        let mut element = Element::Empty;
        element.new_element(&mut line.bytes());
        match element.first_val() {
            None => {
                low_index += 1;
                high_index += 1;
            }
            Some(v) if high > v => {
                high_index += 1;
                if low > v {
                    low_index += 1;
                }
            }
            _ => {}
        }
    }
    low_index * high_index
}

#[cfg(test)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day13.txt";
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
