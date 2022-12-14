use std::cmp::PartialEq;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::ops::Add;

pub struct NumIterator {
    curr: u64,
    step: i64,
    end: u64,
}

impl Iterator for NumIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.curr;
        if (self.step > 0 && val > self.end) || (self.step < 0 && val < self.end) {
            return None;
        }
        self.curr = (self.curr as i64 + self.step) as u64;
        Some(val)
    }
}

pub fn iter_nums(lower: u64, upper: u64) -> NumIterator {
    if lower < upper {
        NumIterator {
            curr: lower,
            step: 1,
            end: upper,
        }
    } else {
        NumIterator {
            curr: lower,
            step: -1,
            end: upper,
        }
    }
}

pub fn digits(mut n: u64) -> Vec<u64> {
    let mut result = vec![];
    for digit in (0..20).rev() {
        if !result.is_empty() || n / 10_u64.pow(digit) >= 1 || digit == 0 {
            result.push(n / 10_u64.pow(digit));
            n = n % 10_u64.pow(digit);
        }
    }
    return result;
}

pub fn from_digits(digits: &Vec<u64>) -> u64 {
    let mut num = 0;
    for digit in digits {
        num *= 10;
        num += digit;
    }
    num
}

pub fn prepend_digit(leading_digit: u64, rest: u64) -> u64 {
    let mut digits = digits(rest);
    digits.insert(0, leading_digit);
    from_digits(&digits)
}

pub fn print_vec_multiline<T>(v: &Vec<T>)
where
    T: std::fmt::Debug,
{
    for e in v {
        println!("{:?}", e);
    }
}

pub fn vec_to_str_multiline<T>(v: &Vec<T>) -> String
where
    T: std::fmt::Debug,
{
    let mut out = "\n".to_owned();
    for e in v {
        out = format!("{}{:?}\n", out, e);
    }
    out
}

pub fn bit_vec_to_num<'a>(bitvec: &'a [u8]) -> u64 {
    bitvec
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, n)| acc + ((*n as u64) << i))
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash, Debug)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub fn to_arr(&self) -> [i32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn sub(&self, rhs: &Point3) -> Point3 {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    pub fn add(&self, rhs: &Point3) -> Point3 {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    pub fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

pub fn transform_point(p: Point3, mat: [[i32; 3]; 3]) -> Point3 {
    let coords = mat
        .iter()
        .map(|row| p.x * row[0] + p.y * row[1] + p.z * row[2])
        .collect::<Vec<i32>>();

    Point3 {
        x: coords[0],
        y: coords[1],
        z: coords[2],
    }
}

pub type Matrix = [[i32; 3]; 3];

pub fn mat_transpose(mat: Matrix) -> Matrix {
    [
        [mat[0][0], mat[1][0], mat[2][0]],
        [mat[0][1], mat[1][1], mat[2][1]],
        [mat[0][2], mat[1][2], mat[2][2]],
    ]
}
pub fn vec_dot(a: [i32; 3], b: [i32; 3]) -> i32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub fn mat_mul(a: Matrix, b_orig: Matrix) -> Matrix {
    let b = mat_transpose(b_orig);
    [
        [
            vec_dot(a[0], b[0]),
            vec_dot(a[0], b[1]),
            vec_dot(a[0], b[2]),
        ],
        [
            vec_dot(a[1], b[0]),
            vec_dot(a[1], b[1]),
            vec_dot(a[1], b[2]),
        ],
        [
            vec_dot(a[2], b[0]),
            vec_dot(a[2], b[1]),
            vec_dot(a[2], b[2]),
        ],
    ]
}

pub fn mat_pow(a: Matrix, n: usize) -> Matrix {
    (1..n).fold(a, |acc, _| mat_mul(a, acc))
}

pub static ID_MAT: Matrix = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
pub static ROT_90_X: Matrix = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];
pub static ROT_90_Y: Matrix = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
pub static ROT_90_Z: Matrix = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];

#[derive(Copy, Clone, Eq, PartialEq, Ord)]
pub struct PqState<T>
where
    T: Ord,
{
    pub cost: T,
    pub pos: (usize, usize),
}

impl<T> PartialOrd for PqState<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

// Note: Does not include cost of the starting node
pub fn get_shortest_path_grid<T: Add<Output = T>>(
    grid: &Grid<T>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<T>
where
    T: Default + Ord + Add + Copy,
{
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push(PqState {
        cost: T::default(),
        pos: start,
    });
    seen.insert(start);

    while let Some(PqState { cost, pos }) = heap.pop() {
        if pos == end {
            return Some(cost);
        }

        for n in grid.neighbors(pos.0, pos.1) {
            if !seen.contains(&n) {
                seen.insert(n);
                heap.push(PqState {
                    cost: cost + *grid.get(n.0, n.1),
                    pos: n,
                });
            }
        }
    }

    None
}

impl<T> Grid<T>
where
    T: Eq + Hash + Copy,
{
    pub fn to_graph(&self) -> Graph<((usize, usize), T)> {
        let mut graph = Graph::new();
        for ((x, y), t) in self.iter() {
            let node = ((x, y), *t);
            graph.add_node(node);
            for (nx, ny) in self.neighbors(x, y) {
                let nt = self.get(nx, ny);
                graph.add_edge(node, ((nx, ny), *nt), 1);
            }
        }
        graph
    }
}

pub struct Graph<T>
where
    T: Eq + Hash + Copy,
{
    nodes: HashMap<T, HashSet<T>>,
    edges: HashMap<(T, T), u64>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DijkstraState<'a, T>
where
    T: std::cmp::Eq,
{
    pub cost: u64,
    pub node: &'a T,
}

impl<'a, T> PartialOrd for DijkstraState<'a, T>
where
    T: std::cmp::Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl<'a, T> Ord for DijkstraState<'a, T>
where
    T: std::cmp::Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T> Graph<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Graph<T> {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, key: T) {
        self.nodes.insert(key, HashSet::new());
    }

    pub fn add_edge(&mut self, a: T, b: T, cost: u64) {
        self.add_edge_uni(a, b, cost);
        self.add_edge_uni(b, a, cost);
    }

    pub fn add_edge_uni(&mut self, a: T, b: T, cost: u64) {
        let nodes_a = self.nodes.entry(a).or_insert(HashSet::new());
        nodes_a.insert(b);
        let edges_a = self.edges.entry((a, b)).or_insert(cost);
        *edges_a = cost;
    }

    pub fn remove_edge(&mut self, a: T, b: T) {
        self.remove_edge_uni(a, b);
        self.remove_edge_uni(b, a);
    }

    pub fn remove_edge_uni(&mut self, a: T, b: T) {
        if let Some(nodes_a) = self.nodes.get_mut(&a) {
            nodes_a.remove(&b);
        };
        self.edges.remove(&(a, b));
    }

    pub fn neighbors(&self, key: &T) -> Option<&HashSet<T>> {
        if !self.nodes.contains_key(key) {
            return None;
        }
        return Some(&self.nodes.get(key).unwrap());
    }

    pub fn get_shortest_path_cost(&self, start: T, end: T) -> Option<u64> {
        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        heap.push(DijkstraState {
            cost: 0,
            node: &start,
        });
        seen.insert(start);

        while let Some(DijkstraState { cost, node }) = heap.pop() {
            if *node == end {
                return Some(cost);
            }

            for n in self.neighbors(node).unwrap() {
                if !seen.contains(n) {
                    seen.insert(*n);
                    let edge_cost = self.edges.get(&(*node, *n)).unwrap();
                    heap.push(DijkstraState {
                        cost: cost + edge_cost,
                        node: &n,
                    });
                }
            }
        }

        None
    }
}

impl<T> fmt::Debug for Graph<T>
where
    T: Eq + Hash + Copy + fmt::Debug + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_keys = Vec::from_iter(self.nodes.keys().map(|k| *k));
        sorted_keys.sort();
        for key in sorted_keys {
            let mut sorted_edges =
                Vec::from_iter(self.nodes.get(&key).unwrap().into_iter().map(|t| *t));
            sorted_edges.sort();
            write!(f, "\n\x1b[31m{:?}\x1b[0m -> {:?}", key, sorted_edges).unwrap();
        }
        Ok(())
    }
}

// -----------------------------

#[derive(Clone)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn from_points(points: Vec<((i32, i32), T)>, default: T) -> Grid<T> {
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        for (point, _) in points.iter() {
            max_x = i32::max(max_x, point.0);
            max_y = i32::max(max_y, point.1);
            min_x = i32::min(min_x, point.0);
            min_y = i32::min(min_y, point.1);
        }

        let mut grid = Grid {
            grid: vec![vec![default; (max_x - min_x) as usize + 1]; (max_y - min_y) as usize + 1],
        };
        for (point, val) in points.iter() {
            grid.set((point.0 - min_x) as usize, (point.1 - min_y) as usize, *val);
        }

        grid
    }
    pub fn set_all(&mut self, val: T) {
        self.grid = vec![vec![val; self.width()]; self.height()];
    }
}

impl Grid<char> {
    pub fn from_str(grid_str: &String) -> Grid<char> {
        let parsed = grid_str
            .trim()
            .split("\n")
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>();
        Grid { grid: parsed }
    }
}

impl<T> Grid<T>
where
    T: Default + Copy,
{
    pub fn rotated(&self) -> Grid<T> {
        Grid::from_points(
            self.iter()
                .map(|((x, y), t)| ((y as i32, x as i32), *t))
                .collect(),
            T::default(),
        )
    }

    pub fn flip_x(&self) -> Grid<T> {
        Grid::from_points(
            self.iter()
                .map(|((x, y), t)| ((-(x as i32), y as i32), *t))
                .collect(),
            T::default(),
        )
    }

    pub fn flip_y(&self) -> Grid<T> {
        Grid::from_points(
            self.iter()
                .map(|((x, y), t)| ((x as i32, -(y as i32)), *t))
                .collect(),
            T::default(),
        )
    }
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Grid<T> {
        Grid { grid }
    }
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.grid[y][x]
    }
    pub fn get_opt(&self, x: isize, y: isize) -> Option<&T> {
        if self.contains(x, y) {
            Some(&self.grid[y as usize][x as usize])
        } else {
            None
        }
    }
    pub fn contains(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width() as isize && y >= 0 && y < self.height() as isize
    }
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.grid[y][x] = val;
    }
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }
    pub fn height(&self) -> usize {
        self.grid.len()
    }
    pub fn size(&self) -> usize {
        self.width() * self.height()
    }
    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y))
        }
        if y > 0 {
            neighbors.push((x, y - 1))
        }
        if x < self.width() - 1 {
            neighbors.push((x + 1, y))
        }
        if y < self.height() - 1 {
            neighbors.push((x, y + 1))
        }
        return neighbors;
    }
    pub fn neighbors_walk(&self, sx: usize, sy: usize) -> [Vec<(usize, usize)>; 4] {
        let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut neighbors = [vec![], vec![], vec![], vec![]];
        for (i, (dx, dy)) in dirs.iter().enumerate() {
            let (mut x, mut y) = (sx as isize + dx, sy as isize + dy);
            while self.contains(x, y) {
                neighbors[i].push((x as usize, y as usize));
                x += dx;
                y += dy;
            }
        }
        return neighbors;
    }
    pub fn neighbors_diag(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = self.neighbors(x, y);
        if x > 0 && y > 0 {
            neighbors.push((x - 1, y - 1))
        }
        if x < self.width() - 1 && y > 0 {
            neighbors.push((x + 1, y - 1))
        }
        if x < self.width() - 1 && y < self.height() - 1 {
            neighbors.push((x + 1, y + 1))
        }
        if x > 0 && y < self.height() - 1 {
            neighbors.push((x - 1, y + 1))
        }
        return neighbors;
    }
    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter { grid: self, pos: 0 }
    }

    pub fn bfs_iter(
        &self,
        pos: (usize, usize),
        validator: fn((usize, usize), &T) -> bool,
    ) -> GridBfsIter<'_, T> {
        GridBfsIter {
            grid: self,
            to_visit: vec![pos],
            seen: HashSet::new(),
            validator,
        }
    }
}

impl<T> fmt::Debug for Grid<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            write!(f, "\n").unwrap();
            for t in row {
                write!(f, "{:?} ", t).unwrap();
            }
        }
        Ok(())
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            write!(f, "\n").unwrap();
            for t in row {
                write!(f, "{}", t).unwrap();
            }
        }
        Ok(())
    }
}

impl<T> Eq for Grid<T> where T: PartialEq + fmt::Debug {}
impl<T> PartialEq for Grid<T>
where
    T: PartialEq + fmt::Debug,
{
    fn eq(&self, other: &Self) -> bool {
        if other.width() != self.width() || other.height() != self.height() {
            return false;
        }
        for ((x, y), c) in self.iter() {
            if *other.get(x, y) != *c {
                return false;
            }
        }
        return true;
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    pos: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((usize, usize), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.grid.width() * self.grid.height() {
            return None;
        };
        let coord = (self.pos % self.grid.width(), self.pos / self.grid.width());
        self.pos += 1;
        return Some((coord, self.grid.get(coord.0, coord.1)));
    }
}

pub struct GridBfsIter<'a, T> {
    grid: &'a Grid<T>,
    validator: fn((usize, usize), &T) -> bool,
    to_visit: Vec<(usize, usize)>,
    seen: HashSet<(usize, usize)>,
}

impl<'a, T> Iterator for GridBfsIter<'a, T>
where
    T: fmt::Debug,
{
    type Item = ((usize, usize), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.to_visit.is_empty() {
            return None;
        }
        let pos = self.to_visit.pop().unwrap();
        self.seen.insert(pos);
        for (nx, ny) in self.grid.neighbors(pos.0, pos.1) {
            if !self.seen.contains(&(nx, ny)) && (self.validator)(pos, self.grid.get(nx, ny)) {
                self.to_visit.insert(0, (nx, ny));
                self.seen.insert((nx, ny));
            }
        }

        return Some((pos, self.grid.get(pos.0, pos.1)));
    }
}

// -----------------------------

#[derive(Clone, Copy)]
pub struct BinaryStreamIterator<'a> {
    digits: &'a Vec<u8>,
    pub index: usize,
}

impl<'a> BinaryStreamIterator<'a> {
    pub fn new(digits: &'a Vec<u8>) -> BinaryStreamIterator<'a> {
        BinaryStreamIterator { digits, index: 0 }
    }

    pub fn take(&mut self, num_digits: usize) -> &[u8] {
        let old_index = self.index;
        self.index += num_digits;
        &self.digits[old_index..self.index]
    }

    pub fn next(&mut self) -> u8 {
        self.take(1)[0]
    }
}
