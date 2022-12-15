#[allow(dead_code)]
mod utils;
use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use crate::utils::*;

fn main() {
    let input = _get_test_input();
    let p1_y = 10;
    let max_coord = 20;
    // let input = _get_input();
    // let p1_y = 2000000;
    // let max_coord = 4000000;

    // ----------- Parse Input -----------

    let parsed = input
        .trim()
        .split("\n")
        .map(|s| {
            let mut split = s.split(' ');
            let sxs = split.nth(2).unwrap();
            let sys = split.nth(0).unwrap();
            let bxs = split.nth(4).unwrap();
            let bys = split.nth(0).unwrap();

            let sx = sxs.strip_prefix("x=").unwrap().trim_matches(',');
            let sy = sys.strip_prefix("y=").unwrap().trim_matches(':');
            let bx = bxs.strip_prefix("x=").unwrap().trim_matches(',');
            let by = bys.strip_prefix("y=").unwrap();

            return (
                sx.parse::<i64>().unwrap(),
                sy.parse::<i64>().unwrap(),
                bx.parse::<i64>().unwrap(),
                by.parse::<i64>().unwrap(),
            );
        })
        .collect::<Vec<_>>();

    // ----------- Solve -----------

    let mut occupied = HashSet::<(i64, i64)>::new();
    for (_, _, bx, by) in parsed.iter() {
        occupied.insert((*bx, *by));
    }

    let mut p1_y_occupied = HashSet::<i64>::new();
    let mut p2_intervals = HashMap::<i64, Vec<(i64, i64)>>::new();

    for (sx, sy, bx, by) in parsed {
        let radius = (bx - sx).abs() + (by - sy).abs();
        for iy in (sy - radius)..=(sy + radius) {
            let dy = (iy - sy).abs();
            let x_radius = radius - dy;

            // P1
            if iy == p1_y {
                for ix in (sx - x_radius)..=(sx + x_radius) {
                    if !occupied.contains(&(ix, iy)) {
                        p1_y_occupied.insert(ix);
                    }
                }
            }

            // P2
            if iy < 0 || iy > max_coord {
                continue;
            }
            let clamped_interval = ((sx - x_radius).max(0), (sx + x_radius).min(max_coord));
            let old_intervals = p2_intervals.remove(&iy).unwrap_or(vec![]);
            p2_intervals.insert(iy, add_interval(old_intervals, clamped_interval));
        }
    }

    let p2_gap_interval = p2_intervals.iter().find(|e| e.1.len() > 1).unwrap();

    // ----------- Print -----------

    println!("Part 1: {}", p1_y_occupied.len());
    println!(
        "Part 2: {}",
        (p2_gap_interval.1[0].1 + 1) * 4000000 + p2_gap_interval.0
    );
}

fn _get_test_input() -> String {
    return "

Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3

"
    .to_string();
}

fn _get_input() -> String {
    return "

Sensor at x=2150774, y=3136587: closest beacon is at x=2561642, y=2914773
Sensor at x=3983829, y=2469869: closest beacon is at x=3665790, y=2180751
Sensor at x=2237598, y=3361: closest beacon is at x=1780972, y=230594
Sensor at x=1872170, y=78941: closest beacon is at x=1780972, y=230594
Sensor at x=3444410, y=3965835: closest beacon is at x=3516124, y=3802509
Sensor at x=3231566, y=690357: closest beacon is at x=2765025, y=1851710
Sensor at x=3277640, y=2292194: closest beacon is at x=3665790, y=2180751
Sensor at x=135769, y=50772: closest beacon is at x=1780972, y=230594
Sensor at x=29576, y=1865177: closest beacon is at x=255250, y=2000000
Sensor at x=3567617, y=3020368: closest beacon is at x=3516124, y=3802509
Sensor at x=1774477, y=148095: closest beacon is at x=1780972, y=230594
Sensor at x=1807041, y=359900: closest beacon is at x=1780972, y=230594
Sensor at x=1699781, y=420687: closest beacon is at x=1780972, y=230594
Sensor at x=2867703, y=3669544: closest beacon is at x=3516124, y=3802509
Sensor at x=1448060, y=201395: closest beacon is at x=1780972, y=230594
Sensor at x=3692914, y=3987880: closest beacon is at x=3516124, y=3802509
Sensor at x=3536880, y=3916422: closest beacon is at x=3516124, y=3802509
Sensor at x=2348489, y=2489095: closest beacon is at x=2561642, y=2914773
Sensor at x=990761, y=2771300: closest beacon is at x=255250, y=2000000
Sensor at x=1608040, y=280476: closest beacon is at x=1780972, y=230594
Sensor at x=2206669, y=1386195: closest beacon is at x=2765025, y=1851710
Sensor at x=3932320, y=3765626: closest beacon is at x=3516124, y=3802509
Sensor at x=777553, y=1030378: closest beacon is at x=255250, y=2000000
Sensor at x=1844904, y=279512: closest beacon is at x=1780972, y=230594
Sensor at x=2003315, y=204713: closest beacon is at x=1780972, y=230594
Sensor at x=2858315, y=2327227: closest beacon is at x=2765025, y=1851710
Sensor at x=3924483, y=1797070: closest beacon is at x=3665790, y=2180751
Sensor at x=1572227, y=3984898: closest beacon is at x=1566446, y=4774401
Sensor at x=1511706, y=1797308: closest beacon is at x=2765025, y=1851710
Sensor at x=79663, y=2162372: closest beacon is at x=255250, y=2000000
Sensor at x=3791701, y=2077777: closest beacon is at x=3665790, y=2180751
Sensor at x=2172093, y=3779847: closest beacon is at x=2561642, y=2914773
Sensor at x=2950352, y=2883992: closest beacon is at x=2561642, y=2914773
Sensor at x=3629602, y=3854760: closest beacon is at x=3516124, y=3802509
Sensor at x=474030, y=3469506: closest beacon is at x=-452614, y=3558516

"
    .to_string();
}
