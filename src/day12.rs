use crate::utils::Grid;

#[allow(dead_code)]
mod utils;

fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------

    let mut grid = Grid::from_str(&input);

    // ----------- Solve -----------
    let start = grid.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let end = grid.iter().find(|(_, c)| **c == 'E').unwrap().0;
    grid.set(start.0, start.1, 'a');
    grid.set(end.0, end.1, 'z');

    let mut graph = grid.to_graph();

    for ((x, y), c) in grid.iter() {
        for (nx, ny) in grid.neighbors(x, y) {
            let nc = grid.get(nx, ny);
            if *nc as u32 > *c as u32 + 1 {
                graph.remove_edge_uni(((x, y), *c), ((nx, ny), *nc));
            }
        }
    }

    let cost = graph
        .get_shortest_path_cost((start, 'a'), (end, 'z'))
        .unwrap();

    let min_a_cost = (0..grid.height()).fold(u64::MAX, |min, y| {
        let new_start = (start.0, y);
        let cost = graph
            .get_shortest_path_cost((new_start, 'a'), (end, 'z'))
            .unwrap();
        min.min(cost)
    });

    // ----------- Print -----------

    println!("Part 1: {}", cost);
    println!("Part 2: {}", min_a_cost);
}

fn _get_test_input() -> String {
    return "

Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi

"
    .to_string();
}

fn _get_input() -> String {
    return "

abaaaaacaaaacccccccccaaaaaaccccccccccccccccccccccccccccccccccaaaaaa
abaaaaacaaaaccccaaaaaaaaaacccccccccccccccccccccccccccccccccccaaaaaa
abaaacccaaaaccccaaaaaaaaaaacccaacccccccccccaacccccccccccccccccaaaaa
abaaaacccaacccccaaaaaaaaaaaaaaaaacccccccccccacccccccccccccccccccaaa
abacaacccccccccccaaaaaaaaaaaaaaaaccccccccccaacccccccccccccccccccaaa
abcccacccccccccccaaaaaaaccaaaaaaaccccccccccclllcccccacccccccccccaac
abccccccccccccccccaaaaaccccccccccccccccccclllllllcccccccccccccccccc
abaaacccccccccccccaaaaaccccccccccccccccaakklllllllcccccccccaacccccc
abaaacccccccccccacccaaaccccccccccccccccakkklpppllllccddaaacaacccccc
abaaacccaaacccccaacaaaccccccccccccccccckkkkpppppllllcddddaaaacccccc
abaacccaaaacccccaaaaaccccccccccccccccckkkkpppppppllmmddddddaaaacccc
abaaaccaaaaccccccaaaaaacaaacccccccccckkkkpppuuuppplmmmmdddddaaacccc
abaaacccaaaccccaaaaaaaacaaaaccccccckkkkkoppuuuuuppqmmmmmmdddddacccc
abcccccccccccccaaaaaaaacaaaacccccjkkkkkooppuuuuuuqqqmmmmmmmddddcccc
abccccccccccccccccaaccccaaaccccjjjjkoooooouuuxuuuqqqqqqmmmmmddecccc
abacaaccccccccccccaacccccccccccjjjjoooooouuuxxxuvvqqqqqqqmmmeeecccc
abaaaacccccccacccaccccccccccccjjjjoootuuuuuuxxxyvvvvvqqqqmmmeeecccc
abaaaaacccccaaacaaacccccccccccjjjoooottuuuuuxxyyvvvvvvvqqmnneeecccc
abaaaaaccaaaaaaaaaaccccccccaccjjjooottttxxxxxxyyyyyyvvvqqnnneeecccc
abaaaccccaaaaaaaaaacccccccaaccjjjoootttxxxxxxxyyyyyyvvqqqnnneeecccc
SbcaaccccaaaaaaaaaaccccaaaaacajjjnnntttxxxxEzzzyyyyvvvrrqnnneeccccc
abcccccccaaaaaaaaaaacccaaaaaaaajjjnnntttxxxxyyyyyvvvvrrrnnneeeccccc
abcccccccaaaaaaaaaaacccccaaaaccjjjnnnnttttxxyyyyywvvrrrnnneeecccccc
abcccccccccaaaaaaccaccccaaaaaccciiinnnnttxxyyyyyyywwrrnnnneeecccccc
abccccccccccccaaacccccccaacaaaccciiinnnttxxyywwyyywwrrnnnffeccccccc
abccccccccccccaaacccccccaccaaaccciiinnnttwwwwwwwwwwwrrrnnfffccccccc
abccccccccccccccccccccccccccccccciiinnnttwwwwsswwwwwrrrnnfffccccccc
abaaaccaaccccccccccccccccccccccccciinnnttswwwssswwwwrrroofffacccccc
abaaccaaaaaacccccccccccccccccaaacciinnntssssssssssrrrrooofffacccccc
abaccccaaaaacccccccaaacccccccaaaaciinnnssssssmmssssrrrooofffacccccc
abaacaaaaaaacccccccaaaaccccccaaaaciiinmmmssmmmmmoosroooooffaaaacccc
abaaaaaaaaaaaccccccaaaaccccccaaacciiimmmmmmmmmmmoooooooofffaaaacccc
abcaaaaaaaaaaccccccaaaaccccccccccccihhmmmmmmmhggoooooooffffaaaccccc
abcccccaaacaccccccccaaccccccccccccchhhhhhhhhhhggggggggggffaaacccccc
abaccccaacccccccccccaaaccccccccccccchhhhhhhhhhgggggggggggcaaacccccc
abaaaccccaccccccccccaaaacccaacccccccchhhhhhhaaaaaggggggcccccccccccc
abaaaccccaaacaaaccccaaaacaaaacccccccccccccccaaaacccccccccccccccaaac
abaacccccaaaaaaaccccaaaaaaaaacccccccccccccccaaacccccccccccccccccaaa
abaaaccccaaaaaaccccaaaaaaaaccccccccccccccccccaacccccccccccccccccaaa
abccccccaaaaaaaaaaaaaaaaaaacccccccccccccccccaaccccccccccccccccaaaaa
abcccccaaaaaaaaaaaaaaaaaaaaacccccccccccccccccccccccccccccccccaaaaaa

"
    .to_string();
}

