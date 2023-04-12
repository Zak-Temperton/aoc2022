use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    resources: [u32; 4],
    robots: [u32; 4],
    time: u32,
}

impl State {
    pub const fn new(time: u32) -> Self {
        Self {
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

    pub fn hash(&mut self) -> u64 {
        let mut hash = 0;
        for i in 0..4 {
            hash *= 120;
            hash += self.resources[i] as u64;
            hash *= 20;
            hash += self.robots[i] as u64;
        }
        hash *= 32;
        hash += self.time as u64;
        hash
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
    let mut res = 0;
    let mut next_time = time;
    while let Some(mut state) = states.pop_front() {
        if state.time == 0 {
            res = res.max(state.resources[3]);
            continue;
        }
        if seen.contains(&state.hash()) {
            continue;
        }
        //with a queue all states are given in chronological order
        if state.time < next_time {
            next_time = state.time;
            seen.clear();
        }
        //Store unique Hash and not entire state
        seen.insert(state.hash());
        state.time -= 1;

        //Always buy geode robot as soon as possible
        if state.resources[0] >= geode_cost.0 && state.resources[2] >= geode_cost.1 {
            let mut new_state = state;
            new_state.earn();
            new_state.robots[3] += 1;
            new_state.resources[0] -= geode_cost.0;
            new_state.resources[2] -= geode_cost.1;
            states.push_back(new_state);
            continue;
        }
        //Always buy obsidian robot as it helps you buy geode robots ASAP
        if state.robots[2] < geode_cost.1
            && state.resources[0] >= obsidian_cost.0
            && state.resources[1] >= obsidian_cost.1
        {
            let mut new_state = state;
            new_state.earn();
            new_state.robots[2] += 1;
            new_state.resources[0] -= obsidian_cost.0;
            new_state.resources[1] -= obsidian_cost.1;
            states.push_back(new_state);
            continue;
        }

        if (state.robots[2] < geode_cost.1 && state.robots[0] < obsidian_cost.0
            || state.robots[1] < obsidian_cost.1 && state.robots[0] < clay_cost
            || state.robots[0] < geode_cost.0)
            && state.resources[0] >= ore_cost
        {
            let mut new_state = state;
            new_state.earn();
            new_state.robots[0] += 1;
            new_state.resources[0] -= ore_cost;
            states.push_back(new_state);
        }

        if state.robots[2] < geode_cost.1
            && state.robots[1] < obsidian_cost.1
            && state.resources[0] >= clay_cost
        {
            let mut new_state = state;
            new_state.earn();
            new_state.robots[1] += 1;
            new_state.resources[0] -= clay_cost;
            states.push_back(new_state);
        }

        state.earn();
        states.push_back(state);
    }
    res
}

pub fn part1(text: &str) -> u32 {
    let mut sum = 0;
    for line in text.lines() {
        let mut split = line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|num| !num.is_empty())
            .flat_map(str::parse);
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

pub fn part2(text: &str) -> u32 {
    let mut product = 1;
    for line in text.lines().take(3) {
        let mut split = line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|num| !num.is_empty())
            .skip(1)
            .flat_map(str::parse);
        let ore_cost = split.next().unwrap();
        let clay_cost = split.next().unwrap();
        let obsidian_cost = (split.next().unwrap(), split.next().unwrap());
        let geode_cost = (split.next().unwrap(), split.next().unwrap());

        let quality = max_geodes_processed(ore_cost, clay_cost, obsidian_cost, geode_cost, 32);
        product *= quality;
    }
    product
}

#[cfg(test)]
mod bench {
    use super::*;
    use std::fs::read_to_string;
    use test::Bencher;
    const PATH: &str = "res/day19.txt";
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
