#[allow(dead_code)]
mod utils;
#[allow(unused_imports)]
use crate::utils::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------

    let parsed = input
        .trim()
        .split("\n")
        .map(|s| {
            let mut words_iter = s.split(' ');
            let valve = words_iter.nth(1).unwrap();
            let rate_eq = words_iter.nth(2).unwrap();
            let rate = rate_eq.split_once('=').unwrap().1.trim_end_matches(';');
            let tunnels = words_iter
                .skip(4)
                .map(|v| v.trim_end_matches(','))
                .collect::<Vec<_>>();
            return (valve, rate.parse::<i64>().unwrap(), tunnels);
        })
        .collect::<Vec<_>>();

    // ----------- Solve -----------

    let mut graph = Graph::new();
    let mut flows = HashMap::new();

    for (valve, rate, tunnels) in parsed.iter() {
        flows.insert(*valve, *rate);
        graph.add_node(*valve);
        for tunnel in tunnels {
            graph.add_edge_uni(*valve, tunnel, 1)
        }
    }

    let mut shortcuts = Graph::new();
    for (start, _) in graph.nodes.iter() {
        if *start != "AA" && *flows.get(*start).unwrap() == 0 {
            continue;
        }
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_front((*start, 0));
        while !to_visit.is_empty() {
            let (node, dist) = to_visit.pop_front().unwrap();
            if visited.contains(node) {
                continue;
            }
            visited.insert(node);
            let flow = *flows.get(node).unwrap();
            if flow > 0 && node != *start {
                shortcuts.add_edge_uni(*start, node, dist)
            }
            for neighbor in graph.neighbors(&node).unwrap() {
                to_visit.push_back((*neighbor, dist + 1));
            }
        }
    }

    fn get_pressure<'a>(
        node: &'a str,
        opened: Vec<&'a str>,
        days_left: i64,
        with_ele: bool,
        cache: &mut HashMap<(&'a str, Vec<&'a str>, i64, bool), i64>,
        shortcuts: &'a Graph<&str>,
        flows: &HashMap<&str, i64>,
    ) -> i64 {
        if days_left <= 0 {
            return 0;
        }
        let cache_tuple = (node, opened.clone(), days_left, with_ele);
        if let Some(val) = cache.get(&cache_tuple) {
            return *val;
        }

        let mut max = 0;
        for neighbor in shortcuts.neighbors(&node).unwrap() {
            let cost = shortcuts.edges.get(&(node, *neighbor)).unwrap();
            let mut new_opened = opened.clone();
            if new_opened.binary_search(neighbor).is_ok() || (days_left - cost < 0) {
                continue;
            }
            let index = new_opened.binary_search(neighbor).unwrap_err();
            new_opened.insert(index, neighbor);
            let flow = flows.get(neighbor).unwrap();
            let new_days_left = days_left - (cost + 1);
            max = max.max(
                flow * new_days_left
                    + get_pressure(
                        neighbor,
                        new_opened,
                        new_days_left,
                        with_ele,
                        cache,
                        shortcuts,
                        flows,
                    ),
            );
        }

        if with_ele {
            max = max.max(get_pressure(
                "AA", opened, 26, false, cache, shortcuts, flows,
            ));
        }

        cache.insert(cache_tuple, max);
        return max;
    }

    let mut cache = HashMap::<(&str, Vec<&str>, i64, bool), i64>::new();
    let p1 = get_pressure("AA", Vec::new(), 30, false, &mut cache, &shortcuts, &flows);
    let p2 = get_pressure("AA", Vec::new(), 26, true, &mut cache, &shortcuts, &flows);

    // ----------- Print -----------

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn _get_test_input() -> String {
    return "

Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II


"
    .to_string();
}

fn _get_input() -> String {
    return "

Valve GJ has flow rate=14; tunnels lead to valves UV, AO, MM, UD, GM
Valve HE has flow rate=0; tunnels lead to valves QE, SV
Valve ET has flow rate=0; tunnels lead to valves LU, SB
Valve SG has flow rate=0; tunnels lead to valves FF, SB
Valve LC has flow rate=0; tunnels lead to valves QJ, GM
Valve EE has flow rate=13; tunnels lead to valves RE, BR
Valve AA has flow rate=0; tunnels lead to valves QC, ZR, NT, JG, FO
Valve TF has flow rate=0; tunnels lead to valves LU, MM
Valve GO has flow rate=0; tunnels lead to valves LB, AH
Valve QE has flow rate=24; tunnels lead to valves LG, HE
Valve MI has flow rate=0; tunnels lead to valves KU, FF
Valve BR has flow rate=0; tunnels lead to valves HY, EE
Valve UV has flow rate=0; tunnels lead to valves GP, GJ
Valve EH has flow rate=0; tunnels lead to valves UU, FF
Valve WK has flow rate=0; tunnels lead to valves HY, EL
Valve NT has flow rate=0; tunnels lead to valves FF, AA
Valve KI has flow rate=0; tunnels lead to valves OQ, AO
Valve AH has flow rate=22; tunnels lead to valves GO, RE
Valve EL has flow rate=0; tunnels lead to valves WK, SQ
Valve GP has flow rate=0; tunnels lead to valves SB, UV
Valve GM has flow rate=0; tunnels lead to valves LC, GJ
Valve LU has flow rate=9; tunnels lead to valves UU, DW, TF, ET, ML
Valve LB has flow rate=0; tunnels lead to valves GO, VI
Valve QC has flow rate=0; tunnels lead to valves ML, AA
Valve JJ has flow rate=0; tunnels lead to valves QJ, DV
Valve MM has flow rate=0; tunnels lead to valves TF, GJ
Valve VI has flow rate=18; tunnel leads to valve LB
Valve NV has flow rate=0; tunnels lead to valves SB, KU
Valve VT has flow rate=0; tunnels lead to valves HY, JG
Valve RE has flow rate=0; tunnels lead to valves AH, EE
Valve FO has flow rate=0; tunnels lead to valves SB, AA
Valve DV has flow rate=10; tunnels lead to valves JH, UD, JJ
Valve SQ has flow rate=12; tunnels lead to valves EL, QA
Valve OQ has flow rate=23; tunnels lead to valves KI, IV, JS
Valve FF has flow rate=3; tunnels lead to valves EU, NT, SG, MI, EH
Valve IV has flow rate=0; tunnels lead to valves LG, OQ
Valve HY has flow rate=8; tunnels lead to valves VT, BR, WK
Valve ML has flow rate=0; tunnels lead to valves LU, QC
Valve JS has flow rate=0; tunnels lead to valves EM, OQ
Valve KU has flow rate=5; tunnels lead to valves MI, VL, NV, HU, DW
Valve QA has flow rate=0; tunnels lead to valves OS, SQ
Valve EU has flow rate=0; tunnels lead to valves FF, OS
Valve SV has flow rate=0; tunnels lead to valves QJ, HE
Valve JG has flow rate=0; tunnels lead to valves AA, VT
Valve DW has flow rate=0; tunnels lead to valves LU, KU
Valve UD has flow rate=0; tunnels lead to valves DV, GJ
Valve QJ has flow rate=17; tunnels lead to valves JJ, SV, LC, EM, YA
Valve HU has flow rate=0; tunnels lead to valves JH, KU
Valve ZR has flow rate=0; tunnels lead to valves AA, VL
Valve YA has flow rate=0; tunnels lead to valves QJ, OS
Valve JH has flow rate=0; tunnels lead to valves HU, DV
Valve OS has flow rate=15; tunnels lead to valves EU, YA, QA
Valve LG has flow rate=0; tunnels lead to valves QE, IV
Valve SB has flow rate=4; tunnels lead to valves FO, SG, NV, GP, ET
Valve UU has flow rate=0; tunnels lead to valves EH, LU
Valve VL has flow rate=0; tunnels lead to valves ZR, KU
Valve AO has flow rate=0; tunnels lead to valves GJ, KI
Valve EM has flow rate=0; tunnels lead to valves QJ, JS

"
    .to_string();
}
