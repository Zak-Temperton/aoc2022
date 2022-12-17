use std::collections::{HashMap, HashSet};

const fn name_to_id(name: &str) -> usize {
    let name = name.as_bytes();
    ((name[0] - b'A') as usize * 26) + (name[1] - b'A') as usize
}

#[derive(Debug, PartialEq, Eq)]
struct Valve {
    id: usize,
    flow_rate: usize,
    tunnels: Vec<(usize, usize)>,
    open: bool,
}

impl From<&str> for Valve {
    fn from(line: &str) -> Self {
        let mut split = line.split("; ");
        let s1 = split.next().unwrap();
        let s2 = split.next().unwrap();
        Valve {
            id: name_to_id(&s1[6..8]),
            flow_rate: s1[23..].parse::<usize>().unwrap(),
            tunnels: s2[22..]
                .split(|b| ((b as u8) < b'A') || ((b as u8) > b'Z'))
                .filter(|s| !s.is_empty())
                .map(|s| (name_to_id(s), 1))
                .collect(),
            open: false,
        }
    }
}

pub(crate) fn part1(text: &str) -> usize {
    let mut valves = HashMap::new();
    let mut working_valves = HashSet::new();
    for line in text.lines() {
        let valve = Valve::from(line);
        if valve.flow_rate > 0 {
            working_valves.insert(valve.id);
        }
        valves.insert(valve.id, valve);
    }

    let mut new_valves = HashMap::new();

    for &from in working_valves.iter().chain([name_to_id("AA")].iter()) {
        let valve = valves.get(&from).unwrap().clone();
        let mut new_valve = Valve {
            flow_rate: valve.flow_rate,
            id: from,
            open: false,
            tunnels: Vec::new(),
        };
        for &to in working_valves.iter() {
            if from != to {
                let len = bfs(from, to, &mut valves, 0);
                new_valve.tunnels.push((to, len))
            }
        }
        new_valves.insert(from, new_valve);
    }
    test(name_to_id("AA"), &mut new_valves, 30)
}

fn test(cur: usize, valves: &mut HashMap<usize, Valve>, time: usize) -> usize {
    if time == 0 {
        return 0;
    }
    let pressure = valves.get(&cur).unwrap().flow_rate * (time);
    let mut best = pressure;
    for (tunnel, len) in valves.get(&cur).unwrap().tunnels.clone() {
        if !valves.get(&tunnel).unwrap().open && time > len + 1 {
            valves.get_mut(&tunnel).unwrap().open = true;
            let result = test(tunnel, valves, (time - 1) - len) + pressure;
            if best < result {
                best = result;
            }
            valves.get_mut(&tunnel).unwrap().open = false;
        }
    }
    best
}

fn bfs(cur: usize, target: usize, valves: &mut HashMap<usize, Valve>, len: usize) -> usize {
    if cur == target {
        return len;
    }
    valves.get_mut(&cur).unwrap().open = true;
    let mut shortest = usize::MAX;
    for (tunnel, l) in valves.get(&cur).unwrap().tunnels.clone() {
        if !valves.get(&tunnel).unwrap().open && l + len < shortest {
            shortest = shortest.min(bfs(tunnel, target, valves, l + len));
        }
    }
    valves.get_mut(&cur).unwrap().open = false;
    shortest
}

pub(crate) fn part2(text: &str) -> usize {
    let mut valves = HashMap::new();
    let mut working_valves = HashSet::new();
    for line in text.lines() {
        let valve = Valve::from(line);
        if valve.flow_rate > 0 {
            working_valves.insert(valve.id);
        }
        valves.insert(valve.id, valve);
    }

    let mut new_valves = HashMap::new();
    for &from in working_valves.iter().chain([name_to_id("AA")].iter()) {
        let valve = valves.get(&from).unwrap().clone();
        let mut new_valve = Valve {
            flow_rate: valve.flow_rate,
            id: from,
            open: false,
            tunnels: Vec::new(),
        };
        for &to in working_valves.iter() {
            if from != to {
                let len = bfs(from, to, &mut valves, 0);
                new_valve.tunnels.push((to, len))
            }
        }
        new_valves.insert(from, new_valve);
    }

    test2(name_to_id("AA"), &mut new_valves, 26)
}

fn test2(me: usize, valves: &mut HashMap<usize, Valve>, time1: usize) -> usize {
    let mut best = 0;
    if time1 > 0 {
        let pressure = valves.get(&me).unwrap().flow_rate * (time1);
        best = pressure + test(0, valves, 26);
        for (tunnel, len) in valves.get(&me).unwrap().tunnels.clone() {
            if !valves.get(&tunnel).unwrap().open && time1 > len + 1 {
                valves.get_mut(&tunnel).unwrap().open = true;
                let result = test2(tunnel, valves, (time1 - 1) - len) + pressure;
                if best < result {
                    best = result;
                }
                valves.get_mut(&tunnel).unwrap().open = false;
            }
        }
    }

    best
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day16.txt";
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        let text = read_to_string(PATH).unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    #[ignore = "too slow"]
    fn part2_bench(b: &mut Bencher) {
        let text = read_to_string(PATH).unwrap();
        b.iter(|| part2(&text));
    }
}
