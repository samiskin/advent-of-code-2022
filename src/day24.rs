#[allow(dead_code)]
mod utils;
use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use crate::utils::*;

fn main() {
    // let input = _get_test_input();
    let input = _get_input();

    // ----------- Parse Input -----------

    let grid = Grid::from_str(input.trim());
    let (width, height) = (grid.width() as isize, grid.height() as isize);
    let mut blizzards = HashMap::new();
    let mut walls = HashSet::new();
    for ((x, y), c) in grid.iter() {
        if *c != '#' && *c != '.' {
            blizzards.insert((x as isize, y as isize), vec![c]);
        }
        if *c == '#' {
            walls.insert((x as isize, y as isize));
        }
    }

    // ----------- Solve -----------

    let start = (1, 0);
    let end = (width - 2, height - 1);

    let mut positions = HashSet::new();
    positions.insert(start);

    let steps = [(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)];

    let mut lap = 0;
    let mut dist = 0;
    let mut p1_dist = 0;
    loop {
        let mut next_positions = HashSet::new();
        for (x, y) in positions.iter() {
            for (dx, dy) in steps {
                let (nx, ny) = (x + dx, y + dy);

                if ny < 0 || ny >= height || walls.contains(&(nx, ny)) {
                    continue;
                }
                if !blizzards.contains_key(&(nx, ny)) {
                    next_positions.insert((nx, ny));
                }
            }
        }

        if lap % 2 == 0 && next_positions.contains(&end) {
            lap += 1;
            next_positions.clear();
            next_positions.insert(end);
        }
        if lap % 2 == 1 && next_positions.contains(&start) {
            lap += 1;
            next_positions.clear();
            next_positions.insert(start);
        }
        if lap == 1 && p1_dist == 0 {
            p1_dist = dist;
        }
        if lap == 3 {
            break;
        }

        let mut next_blizzards = HashMap::<(isize, isize), Vec<&char>>::new();
        for ((x, y), cs) in blizzards {
            for c in cs {
                let (mut nx, mut ny) = (x, y);
                match c {
                    '>' => nx += 1,
                    '<' => nx -= 1,
                    '^' => ny -= 1,
                    'v' => ny += 1,
                    _ => (),
                }
                if nx <= 0 {
                    nx = width - 2;
                } else if nx >= width - 1 {
                    nx = 1;
                }
                if ny <= 0 {
                    ny = height - 2;
                } else if ny >= height - 1 {
                    ny = 1;
                }
                if let Some(v) = next_blizzards.get_mut(&(nx, ny)) {
                    v.push(c);
                } else {
                    next_blizzards.insert((nx, ny), vec![c]);
                }
            }
        }

        positions = next_positions;
        blizzards = next_blizzards;
        dist += 1;
    }

    // ----------- Print -----------

    println!("Part 1: {}", p1_dist);
    println!("Part 2: {}", dist);
}

fn _get_test_input_small() -> String {
    return "

#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#

"
    .to_string();
}

fn _get_test_input() -> String {
    return "

#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#

"
    .to_string();
}

fn _get_input() -> String {
    return "

#.####################################################################################################
#>>^<^v>v<>^>>v<<.<<<^<vv^<^>v><^<v>><^>.<><^^>v<^>v>^^>v><v.^<.^>>>^>v^^^<vv^<><vvv>v>^vv<<>v^^<v><>#
#>^>^><^v<>.v><><<<>><v><>vv^.><v^<^v^>v<<<v..<v.v^v.v<<v<>>^^<v^v.<^.^v><^v^.^^<v>..v<^>^<<^>.v.>v.>#
#.>^>v<v^><v<v^><>^v>><>v<>^><>v^<^v<>v<>^<v<><^^v.v.>^^.v<<^v<>>.v>>v<^^.v<v^<^>>..>><<v.^>.<<<^v.^>#
#<>^^^..v^^v^<<><<<^<>.>.vvv.v>^^.><.^>>>v^^>^^vv>v<^v>>^^<v>v^<<^>..^v^<>>>>>.>>><v^^^vv<>v^v<>^^.<>#
#>v^vvv><>>v^v.v><v<<^.>.>>^<.^<^<>><^<><^>v.>v<>v><<>>><^v<.>v<^v<v^..<vv><><.^^^^<^.<v<<^^>>v^>^<><#
#>^>^>^<<>^><<v^^^^^^^v^<<>>v^^>v>v<.><><^v^<v>>>>^<..>v<.^v><^<^v>v^^vvv^v^<v>>.^<<<.^^vv>vvv>^..<v>#
#><vv.vv^v.>^>>v<<<v^>>^^>><v<<.^>>v>v^.<v<vv<<<.vv^<>^.<v>vv<.>^^v><><^<v<^><.^vv^^<><<.vv^<v<<v.v^>#
#<^<v>.>^v^<v^^><vv.>>.<^>.^..><v^<<v^<^<<.<v<^v^v<^<.<<v<^v>^<<^v<>^<^v>vv^>vv>^<v.>^><v..>><v^<^^^>#
#>^><v><<<>><^v^>v<><>v^><.v>>.>^v<.^><vvv<.>v<v>^>>v><^^^v^.v>^.^^>^v>>v^v<^v>.^.<^>^^v<.<<v^..<<.v>#
#>vv>^.<<>><^>.<^><v<^<>>v><v>^.>v^<><.v.>^>>^><<v>^v..<^^^<vv^..^^<<<.v<<v<vv^v^<^><><v>.<<><^<v<^<<#
#>^>v<^v^.<<v<<^^>^<>vvv<v^>^<v<.^v^>><>vv<>v<.^^>^<>>><>v<<>.>^<<>vvv^v.v<><<>^.v.>^><^^>>^>>><>v.^<#
#>^<.^.<>v<.<^v>v<^^<>^>vvv..v.^><<^<.><...<.^>^>^vv<>v^^><<v>v^^<^^><<^>>vvv^<^^v>>vv<v<><v^<^v<^v^>#
#.>>^.v<>v>><<..vv>^.>>^>^^.v>v.<^><>v<v^^><<v<.>>^<v<<><>.^<<.>>^<v<>v^>><v>^>..v<^<^.vv<>^^>>>v^>v<#
#<>^^^<v><^v^vvv>^v^v><<>>^<vv^><<v.>>v>.v>^>>v<^vv.<><<>><<v<>>^.<<>v.v^^vv<.>^<^v.<^<vvv^v.vv><vv.>#
#>vv.vv^><v>v>.<v^><v<vv^v<.v<<>^<^>^><v^>v>^<v><>v>>v>>>>v^.<v<vv>^v<^.^.^>>><>^<^<^.<v<.><v.^<^.><.#
#>^vvv<>^v^^>^<^v<^>v<<^<vv.vvv<<<v<v^..^.<<v<<<>>^>v^v^>^><v.^<v<<v>^^v<v.>>>v^<v^<v>^<^^.>v<^<>^.v<#
#<>^<<<<v<>^v>v^<vv<^v>v^^<>.^>^^<.>>v>v<<^<<<>v<v^v><>>.<<.^<>><<<v>v^>v^^<v^^.><vv>^^<v.v>^<v<.vv><#
#<>vvv<^^.v^^.v.<^<^>.^^v>^v>>.vv.>.v<<^<^^>vvv^<<<vv^<vv<>>v^^<^.<^.>v.^>v><^^>>^>^<>.>.^<<<.v.<>^>>#
#><>v<>>>><.^v>v><v>^<<><>^vv.^.<.>>vvvv><>^^><>v>v.^^<^<^v>v<^<.<^<>^^^^vvv>.>>><^vv<^^^>.^<<v^>>><>#
#<<><^>>^>^^>><.v^^^.v^^^>.^v<vvv.^^<^^>^<>>><<>>><<.v<^>><<v<v^>vv^<>vv.<<>v<v^><<vv^^^>vvvv<<v<><<<#
#<^<><>><<>^^>v<<^v<v>^^^>^^vvvv<^^vv<>><.<^^.vv^>>.<>^.v<><^^vv>^^>>^<^^v>.v<>>^v.v^>.^v<.<v.<>>>>v<#
#<^<<^^^vv><^<>^^>^v<v^v^.<v.>v>><<^^>v^>v>><v<<><>^^.vv^><.^>vv^v<<>^><^>^>^v^<><>^.^<.^.^>>^<.>^><<#
#>^.<<v>^.<<<^>v>>>^<.^>>>.^^<<^.v^<<>v^<<<^<><>^vvvv^>vv^^>.>>^>><>^^^^>v<^^>^^<^vv.v.^<<^<<^>v><.>>#
#>vv^.^vv>^<>^v.^^<v>v>><^^^v<<>>^>>^^>^<^<<<^.v><^>v<<v^<^^^>vv><<v<.<v><^vv>^>^v^<^v>^<^vv<<^v<.^^>#
#<v>vv<^>v^^<<^^<<<v>v><<<.v^>^<><vvv^>.^.v>v<^^vvvv>>^v^^v^^^<>>>>v>>^>v^.<>.>.^vvv^<^<.<^>^<.v<><.>#
#.<.<^v<v><^>.^.v<>>^<>.<<^.vv^>v^v<..>^.^^vv^><.<>^<><^vv>^<.<>v..<^^<v^^v<<<v<><<>>vv<>^^<>^vvvvv><#
#<.^v^>vvv<.v<.vv><^vv^>>.^>><><<v>v<^<<vv.<>v<vv^v>^>^^<v>^>^<<>^<<^>.v^^^><>.^^v>^...^<^>>>.>v^^vv<#
#.<^v>^>^^<^v.v>.^<><<<>>vv>.^v<>.<<>>v^v<v>>^.^<^^<.v^<><vvv>><^.>.vv<.vv^>^><<.<.>><^>vv^^>>v^>^>.<#
#.>^<>><v<.>vv<>>^<v.>^.<v^v.v<><>^<v^<v.<<^v<>v<<v>v^>^>v.>>v^.><^^.>v<v.v^.>^^<>^<.^<v^^><<>^vvv>><#
#.^.^vv^..^>.v.<<v>^^^>^<^v<^>>v<>v>.<><>>.<v^<^>.<v.^^v^<v><>^>v.<^<<><>v.<<<^>.<<.^>^>^<^.>^v>>>.>>#
#.^v>><>v<<.<^>^^<<>^>v>vv<>>^>>^.v^>.>^<v^v>>v^^><.v.<v>.>vv<<^v<v<v^v>v<v<.^<.^<<v>^^v.^v.<^>.v>^>>#
#>^v>^<<>>vv>^<.<v^v<^>>^v>^v^<<><<^^^^v>^^^^^^^vv<^..<vvv><^.<..^>^^<^>v^>>>>vv>v<<<<>.>.>^.^<^.v<v>#
#.^v.^>><.><<v^>^^.>><^v<..>..><.><><<v>>>v<>^.>v<vv^^<<.^.^^v<v..^<v^>.<vvv>^>v>^<^><>>>>v<>^>>vv^.>#
#<v<vv^vvv><v^<^>>.v^v^>><.>><>>v^^<v>>>>.v^.^.<vv.>>>^v>^<>v^.^>v<vv.>>vvv^v^<vv<>v<^<^.<<^..v^.^v<<#
#<.<v>^>v<^<.<><>.v<<^v<^<>^.vv<v<<v>v<^<^v>v<v^^vv<><..v<<<<.^^v<>>^<v>.^>>v<^>v>^><vvv<^>>.^v>^<.><#
####################################################################################################.#

"
    .to_string();
}
