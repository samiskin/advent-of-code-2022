#[allow(dead_code)]
mod utils;
use std::collections::HashSet;

#[allow(unused_imports)]
use crate::utils::*;

fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------

    let parsed = input
        .trim()
        .split("\n")
        .map(|s| {
            let mut iter = s.split(' ');
            let mut get_num = |n: usize| {
                iter.nth(n)
                    .unwrap()
                    .trim_end_matches(':')
                    .parse::<i32>()
                    .unwrap()
            };
            return (
                get_num(1), // blueprint
                get_num(4), // ore ore
                get_num(5), // clay ore
                get_num(5), // obsidian ore
                get_num(2), // obsidian clay
                get_num(5), // geode ore
                get_num(2), // geode obsidian
            );
        })
        .collect::<Vec<_>>();

    // ----------- Solve -----------

    let mut total_quality = 0;
    let mut mul_max_geodes = 1;
    for (id, oo, co, obo, obc, go, gob) in parsed {
        let max_o = oo.max(co).max(obo).max(go);
        let mut max_geodes_p1 = 0;
        let mut max_geodes = 0;
        let mut cache = HashSet::new();
        let time = if id <= 3 { 32 } else { 24 };
        let start = (0, 0, 0, 0, 1, 0, 0, 0, 0);
        let mut to_visit = vec![start];
        while !to_visit.is_empty() {
            let mut tuple = to_visit.pop().unwrap();
            tuple.4 = tuple.4.min(max_o);
            tuple.5 = tuple.5.min(obc);
            tuple.6 = tuple.6.min(gob);
            if cache.contains(&tuple) {
                continue;
            }
            cache.insert(tuple);

            let (o, c, ob, g, ro, rc, rob, rg, t) = tuple;
            max_geodes = max_geodes.max(g);
            if t <= 24 {
                max_geodes_p1 = max_geodes_p1.max(g);
            }
            if t >= time {
                continue;
            }
            if o >= go && ob >= gob {
                to_visit.push((
                    o + ro - go,
                    c + rc,
                    ob + rob - gob,
                    g + rg,
                    ro,
                    rc,
                    rob,
                    rg + 1,
                    t + 1,
                ));
                continue;
            }
            to_visit.push((o + ro, c + rc, ob + rob, g + rg, ro, rc, rob, rg, t + 1));
            if o >= obo && c >= obc {
                to_visit.push((
                    o + ro - obo,
                    c + rc - obc,
                    ob + rob,
                    g + rg,
                    ro,
                    rc,
                    rob + 1,
                    rg,
                    t + 1,
                ));
                continue;
            }
            if o >= co {
                to_visit.push((
                    o + ro - co,
                    c + rc,
                    ob + rob,
                    g + rg,
                    ro,
                    rc + 1,
                    rob,
                    rg,
                    t + 1,
                ));
            }
            if o >= oo {
                to_visit.push((
                    o + ro - oo,
                    c + rc,
                    ob + rob,
                    g + rg,
                    ro + 1,
                    rc,
                    rob,
                    rg,
                    t + 1,
                ));
            }
        }

        let quality = id * max_geodes_p1;
        total_quality += quality;
        if id <= 3 {
            mul_max_geodes = mul_max_geodes * max_geodes
        }
    }

    // ----------- Print -----------

    println!("Part 1: {}", total_quality);
    println!("Part 2: {}", mul_max_geodes);
}

fn _get_test_input() -> String {
    return "

Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.

"
    .to_string();
}

fn _get_input() -> String {
    return "

Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 15 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 3: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 4: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 4 ore and 15 obsidian.
Blueprint 5: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 6: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 7: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 7 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 9: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 18 obsidian.
Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 11: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 12: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 2 ore and 16 obsidian.
Blueprint 13: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 16 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 13 obsidian.
Blueprint 15: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 2 ore and 20 obsidian.
Blueprint 16: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 4 ore and 10 obsidian.
Blueprint 17: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 17 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 18: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.
Blueprint 19: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 20: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 21: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 22: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 3 ore and 19 obsidian.
Blueprint 23: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 25: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 27: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 16 obsidian.
Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 4 ore and 19 obsidian.
Blueprint 30: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.

"
    .to_string();
}
