use std::collections::VecDeque;

fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------

    #[derive(Clone)]
    struct Monkey {
        items: VecDeque<i64>,
        opt_suffix: String,
        divisor: i64,
        next_true: usize,
        next_false: usize,
    }
    impl Monkey {
        fn opt(&self, val: i64) -> i64 {
            let (opt_str, opt_val_str) = self.opt_suffix.split_once(" ").unwrap();
            let opt_val = match opt_val_str {
                "old" => val,
                _ => opt_val_str.parse::<i64>().unwrap(),
            };
            match opt_str {
                "*" => val * opt_val,
                "+" => val + opt_val,
                _ => 0,
            }
        }

        fn next(&self, worry: i64) -> usize {
            if worry % self.divisor == 0 {
                self.next_true
            } else {
                self.next_false
            }
        }
    }

    let parsed = input
        .trim()
        .split("\n\n")
        .map(|s| {
            let mut lines_iter = s.split('\n').skip(1);
            let mut next_stripped = |prefix: &str| {
                lines_iter
                    .next()
                    .unwrap()
                    .trim()
                    .strip_prefix(prefix)
                    .unwrap()
                    .to_owned()
            };
            let items = next_stripped("Starting items: ")
                .split(", ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            let opt_suffix = next_stripped("Operation: new = old ");

            let divisor = next_stripped("Test: divisible by ").parse::<i64>().unwrap();
            let next_true = next_stripped("If true: throw to monkey ")
                .parse::<usize>()
                .unwrap();
            let next_false = next_stripped("If false: throw to monkey ")
                .parse::<usize>()
                .unwrap();

            return Monkey {
                items,
                opt_suffix,
                divisor,
                next_true,
                next_false,
            };
        })
        .collect::<Vec<Monkey>>();

    // ----------- Solve -----------

    let total_mul = parsed.iter().fold(1, |s, m| s * m.divisor);

    let run_monkeys = |rounds: usize, divisor: i64| {
        let mut monkeys = parsed.clone();
        let mut inspections = monkeys.iter().map(|_| 0).collect::<Vec<u64>>();

        for _ in 0..rounds {
            for i in 0..monkeys.len() {
                while !monkeys[i].items.is_empty() {
                    let item = monkeys[i].items.pop_front().unwrap();
                    let worry = (monkeys[i].opt(item) / divisor) % total_mul;
                    inspections[i] += 1;
                    let next = monkeys[i].next(worry);
                    monkeys[next].items.push_back(worry);
                }
            }
        }

        inspections.sort_by(|a, b| b.cmp(a));
        return inspections[0] * inspections[1];
    };

    // ----------- Print -----------

    println!("Part 1: {}", run_monkeys(20, 3));
    println!("Part 2: {}", run_monkeys(10000, 1));
}

fn _get_test_input() -> String {
    return "

Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1

"
    .to_string();
}

fn _get_input() -> String {
    return "

Monkey 0:
  Starting items: 74, 64, 74, 63, 53
  Operation: new = old * 7
  Test: divisible by 5
    If true: throw to monkey 1
    If false: throw to monkey 6

Monkey 1:
  Starting items: 69, 99, 95, 62
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 5

Monkey 2:
  Starting items: 59, 81
  Operation: new = old + 8
  Test: divisible by 7
    If true: throw to monkey 4
    If false: throw to monkey 3

Monkey 3:
  Starting items: 50, 67, 63, 57, 63, 83, 97
  Operation: new = old + 4
  Test: divisible by 13
    If true: throw to monkey 0
    If false: throw to monkey 7

Monkey 4:
  Starting items: 61, 94, 85, 52, 81, 90, 94, 70
  Operation: new = old + 3
  Test: divisible by 19
    If true: throw to monkey 7
    If false: throw to monkey 3

Monkey 5:
  Starting items: 69
  Operation: new = old + 5
  Test: divisible by 3
    If true: throw to monkey 4
    If false: throw to monkey 2

Monkey 6:
  Starting items: 54, 55, 58
  Operation: new = old + 7
  Test: divisible by 11
    If true: throw to monkey 1
    If false: throw to monkey 5

Monkey 7:
  Starting items: 79, 51, 83, 88, 93, 76
  Operation: new = old * 3
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 6

"
    .to_string();
}
