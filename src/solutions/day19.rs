use std::collections::{HashSet, VecDeque};

pub(crate) fn part1(text: &str) -> u32 {
    let mut sum = 0;
    for line in text.lines() {
        let mut split = line
            .split(|c| !"-0123456789".contains(c))
            .filter(|num| !num.is_empty())
            .flat_map(|num| num.parse::<u32>());
        let blueprint = split.next().unwrap();
        let ore_cost = split.next().unwrap();
        let clay_cost = split.next().unwrap();
        let obsidian_cost = (split.next().unwrap(), split.next().unwrap());
        let geode_cost = (split.next().unwrap(), split.next().unwrap());

        let quality = max_geodes_processed(ore_cost, clay_cost, obsidian_cost, geode_cost, 24);
        sum += quality * blueprint;
    }
    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    resources: [u32; 4],
    robots: [u32; 4],
    time: u32,
}

impl State {
    pub fn new(time: u32) -> Self {
        State {
            resources: [0; 4],
            robots: [1, 0, 0, 0],
            time,
        }
    }

    pub fn earn(&mut self) {
        for i in 0..4 {
            self.resources[i] += self.robots[i];
        }
    }
}

fn max_geodes_processed(
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost: (u32, u32),
    geode_cost: (u32, u32),
    time: u32,
) -> u32 {
    let mut states = VecDeque::new();
    states.push_back(State::new(time));
    let mut seen = HashSet::new();
    let max_ore_cost = *[ore_cost, clay_cost, obsidian_cost.0, geode_cost.0]
        .iter()
        .max()
        .unwrap();
    let mut res = 0;
    let mut next_time = time;
    while let Some(mut state) = states.pop_front() {
        if state.time == 0 {
            res = res.max(state.resources[3]);
            continue;
        }
        if seen.contains(&state) {
            continue;
        }
        //clear room in RAM
        //with a queue all states are given in chronological order
        if state.time < next_time {
            next_time = state.time;
            seen.clear();
        }
        seen.insert(state);
        state.time -= 1;

        //if you can afford a new geode robot each turn always make a geode robot
        if state.robots[0] < geode_cost.0 || state.robots[2] < geode_cost.1 {
            if state.robots[0] < max_ore_cost && state.resources[0] >= ore_cost {
                let mut new_state = state.clone();
                new_state.earn();
                new_state.robots[0] += 1;
                new_state.resources[0] -= ore_cost;
                states.push_back(new_state);
            }
            if state.robots[1] < obsidian_cost.1 && state.resources[0] >= clay_cost {
                let mut new_state = state.clone();
                new_state.earn();
                new_state.robots[1] += 1;
                new_state.resources[0] -= clay_cost;
                states.push_back(new_state);
            }
            if state.robots[2] < geode_cost.1
                && state.resources[0] >= obsidian_cost.0
                && state.resources[1] >= obsidian_cost.1
            {
                let mut new_state = state.clone();
                new_state.earn();
                new_state.robots[2] += 1;
                new_state.resources[0] -= obsidian_cost.0;
                new_state.resources[1] -= obsidian_cost.1;
                states.push_back(new_state);
            }
        }
        if state.resources[0] >= geode_cost.0 && state.resources[2] >= geode_cost.1 {
            let mut new_state = state.clone();
            new_state.earn();
            new_state.robots[3] += 1;
            new_state.resources[0] -= geode_cost.0;
            new_state.resources[2] -= geode_cost.1;
            states.push_back(new_state);
        } else {
            state.earn();
            states.push_back(state);
        }
    }
    res
}

pub(crate) fn part2(text: &str) -> u32 {
    let mut product = 1;
    for line in text.lines().take(3) {
        let mut split = line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|num| !num.is_empty())
            .skip(1)
            .flat_map(|num| num.parse::<u32>());
        let ore_cost = split.next().unwrap();
        let clay_cost = split.next().unwrap();
        let obsidian_cost = (split.next().unwrap(), split.next().unwrap());
        let geode_cost = (split.next().unwrap(), split.next().unwrap());

        let quality = max_geodes_processed(ore_cost, clay_cost, obsidian_cost, geode_cost, 32);
        product *= quality;
    }
    product
}

#[allow(soft_unstable, unused_imports, dead_code)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day19.txt";
    #[bench]
    #[ignore = "too slow"]

    fn part1_bench(b: &mut Bencher) {
        let text = read_to_string(PATH).unwrap();
        b.iter(|| part1(&text));
    }
    #[bench]
    #[ignore = "WAY too slow"]
    fn part2_bench(b: &mut Bencher) {
        let text = read_to_string(PATH).unwrap();
        b.iter(|| part2(&text));
    }
}
