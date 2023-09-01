use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use pattern_parse::parse_fn;
use rayon::prelude::*;
use common::input::Linewise;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Add for Resources {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl Sub for Resources {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl Resources {
    fn enough_for(&self, other: &Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }
}

#[derive(Debug)]
pub struct Blueprint {
    id: u8,
    ore_cost: Resources,
    clay_cost: Resources,
    obsidian_cost: Resources,
    geode_cost: Resources,
}
parse_fn!(
    parse_blueprint_line,
    "Blueprint {u8}: Each ore robot costs {usize} ore. Each clay robot costs {usize} ore. Each obsidian robot costs {usize} ore and {usize} clay. Each geode robot costs {usize} ore and {usize} obsidian."
);

impl FromStr for Blueprint {
    type Err = pattern_parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) =
            parse_blueprint_line(s)?;

        Ok(Blueprint {
            id,
            ore_cost: Resources {
                ore: ore_ore,
                ..Default::default()
            },
            clay_cost: Resources {
                ore: clay_ore,
                ..Default::default()
            },
            obsidian_cost: Resources {
                ore: obsidian_ore,
                clay: obsidian_clay,
                ..Default::default()
            },
            geode_cost: Resources {
                ore: geode_ore,
                obsidian: geode_obsidian,
                ..Default::default()
            },
        })
    }
}

static NONE: Resources = Resources {
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 0,
};
static ORE: Resources = Resources {
    ore: 1,
    clay: 0,
    obsidian: 0,
    geode: 0,
};
static CLAY: Resources = Resources {
    ore: 0,
    clay: 1,
    obsidian: 0,
    geode: 0,
};
static OBSIDIAN: Resources = Resources {
    ore: 0,
    clay: 0,
    obsidian: 1,
    geode: 0,
};
static GEODE: Resources = Resources {
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 1,
};

fn try_step(
    blueprint: &Blueprint,
    robots: Resources,
    resources: Resources,
    build: Resources,
    cost: &Resources,
    remaining: usize,
    current_max: &mut usize,
) {
    if resources.enough_for(&cost) {
        let new_bots = robots + build;
        let resources = resources + robots - *cost;
        step(blueprint, new_bots, resources, remaining, current_max);
    }
}

fn step(
    blueprint: &Blueprint,
    robots: Resources,
    resources: Resources,
    remaining: usize,
    current_max: &mut usize,
) {
    let theoretical_max =
        resources.geode + robots.geode * remaining + (1..remaining).sum::<usize>();
    if theoretical_max < *current_max {
        return;
    }

    if remaining == 0 {
        *current_max = usize::max(*current_max, resources.geode);
        return;
    }

    try_step(
        blueprint,
        robots,
        resources,
        GEODE,
        &blueprint.geode_cost,
        remaining - 1,
        current_max,
    );
    try_step(
        blueprint,
        robots,
        resources,
        OBSIDIAN,
        &blueprint.obsidian_cost,
        remaining - 1,
        current_max,
    );
    try_step(
        blueprint,
        robots,
        resources,
        CLAY,
        &blueprint.clay_cost,
        remaining - 1,
        current_max,
    );
    try_step(
        blueprint,
        robots,
        resources,
        ORE,
        &blueprint.ore_cost,
        remaining - 1,
        current_max,
    );
    try_step(
        blueprint,
        robots,
        resources,
        NONE,
        &NONE,
        remaining - 1,
        current_max,
    );
}

fn run_blueprint(blueprint: &Blueprint, steps: usize) -> usize {
    let robots = Resources {
        ore: 1,
        ..Default::default()
    };

    let mut max = 0;
    step(blueprint, robots, Resources::default(), steps, &mut max);
    max
}

fn run(bps: Vec<Blueprint>, steps: usize) -> usize {
    bps.into_par_iter()
        .map(|blueprint| {
            let score = run_blueprint(&blueprint, steps);
            let qs = score * blueprint.id as usize;
            qs
        })
        .sum::<usize>()
}

pub fn task1(input: Linewise<Blueprint>) -> Result<usize, pattern_parse::ParseError> {
    let mut bps = Vec::new();
    common::for_input!(input, |bp| { bps.push(bp) });
    let result = run(bps, 24);
    Ok(result)
}

pub fn task2(input: Linewise<Blueprint>) -> Result<usize, pattern_parse::ParseError> {
    let mut bps = Vec::new();
    common::for_input!(input, |bp| { bps.push(bp) });
    let result = run(bps, 32);
    Ok(result)
}
