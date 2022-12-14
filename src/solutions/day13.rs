use std::{cmp::Ordering, str::Bytes};

#[derive(Debug, Clone)]
enum Element {
    List(Vec<Element>),
    Val(u32),
    Empty,
}

impl Element {
    pub fn new_list(line: &str) -> Element {
        let mut element = Element::List(vec![Element::Empty]);
        let mut bytes = line.bytes();
        if let Element::List(list) = &mut element {
            while let Some(b) = bytes.next() {
                match b {
                    b'[' => Self::new_element(&mut bytes, list.last_mut().unwrap()),
                    b']' => return element,
                    b',' => list.push(Element::Empty),
                    b if b.is_ascii_digit() => match list.last_mut() {
                        Some(Element::Val(v)) => *v = *v * 10 + (b - b'0') as u32,
                        Some(Element::Empty) => {
                            *list.last_mut().unwrap() = Element::Val((b - b'0') as u32)
                        }
                        _ => panic!(),
                    },
                    _ => {}
                }
            }
        }
        element
    }

    fn new_element(bytes: &mut Bytes, element: &mut Element) {
        *element = Element::List(vec![Element::Empty]);

        if let Element::List(list) = element {
            while let Some(b) = bytes.next() {
                match b {
                    b'[' => Self::new_element(bytes, list.last_mut().unwrap()),
                    b']' => return,
                    b',' => list.push(Element::Empty),
                    b if b.is_ascii_digit() => match list.last_mut() {
                        Some(Element::Val(v)) => *v = *v * 10 + (b - b'0') as u32,
                        Some(Element::Empty) => {
                            *list.last_mut().unwrap() = Element::Val((b - b'0') as u32)
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
            Element::List(l) => l[0].first_val(),
            Element::Val(v) => Some(*v),
            Element::Empty => None,
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
            (Element::List(l), Element::List(r)) => {
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
            (Element::List(_), Element::Val(_)) => self.cmp(&Element::List(vec![other.clone()])),
            (Element::Val(_), Element::List(_)) => Element::List(vec![self.clone()]).cmp(other),
            (Element::Val(l), Element::Val(r)) => l.cmp(r),
            (Element::Val(_), Element::Empty) | (Element::List(_), Element::Empty) => {
                Ordering::Greater
            }
            (Element::Empty, Element::List(_)) | (Element::Empty, Element::Val(_)) => {
                Ordering::Less
            }
            (Element::Empty, Element::Empty) => Ordering::Equal,
        })
    }
}

pub(crate) fn part1(text: &str) -> usize {
    let mut sum = 0;
    for (i, lines) in text.lines().array_chunks::<3>().enumerate() {
        let left = Element::new_list(lines[0]);
        let right = Element::new_list(lines[1]);
        if left <= right {
            sum += i + 1;
        }
    }
    sum
}

pub(crate) fn part2(text: &str) -> usize {
    let low = 2;
    let high = 6;
    let mut low_index = 1;
    let mut high_index = 2;
    for lines in text.lines().array_chunks::<3>() {
        let left = Element::new_list(lines[0]);
        let right = Element::new_list(lines[1]);
        match left.first_val() {
            None => {
                low_index += 1;
                high_index += 1
            }
            Some(v) if high > v => {
                high_index += 1;
                if low > v {
                    low_index += 1;
                }
            }
            _ => {}
        }
        match right.first_val() {
            None => {
                low_index += 1;
                high_index += 1
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

#[allow(soft_unstable, unused_imports, dead_code)]
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
