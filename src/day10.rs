fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------

    enum Instruction {
        AddX(i32),
        Noop,
    }

    let parsed = input
        .trim()
        .split("\n")
        .map(|s| {
            if s.starts_with("noop") {
                return Instruction::Noop;
            }

            let (_, val_str) = s.split_once(' ').unwrap();
            return Instruction::AddX(val_str.parse::<i32>().unwrap());
        })
        .collect::<Vec<Instruction>>();

    // ----------- Solve -----------
    let mut x = 1 as i32;
    let mut cycle = 1;
    let mut total_strength = 0;
    let mut crt = [' '; 240];
    for instr in parsed {
        let cycle_wait = match instr {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        };

        for _ in 0..cycle_wait {
            let crt_pos = cycle - 1;
            if (x as i32 - (crt_pos % 40) as i32).abs() <= 1 {
                crt[crt_pos] = '█';
            }
            if cycle % 40 == 20 {
                total_strength += x * cycle as i32;
            }
            cycle += 1;
        }

        if let Instruction::AddX(amt) = instr {
            x += amt;
        }
    }

    // ----------- Print -----------

    let crt_grid = crt.chunks(40).fold(String::from(""), |display, line| {
        return format!("{}\n{}", display, String::from_iter(line));
    });

    println!("Part 1: {}", total_strength);
    println!("Part 2: {}", crt_grid);
}

fn _get_test_input() -> String {
    return "

addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop

"
    .to_string();
}

fn _get_input() -> String {
    return "

addx 1
addx 4
addx 1
noop
addx 4
noop
noop
noop
noop
addx 4
addx 1
addx 5
noop
noop
addx 5
addx -1
addx 3
addx 3
addx 1
noop
noop
addx 4
addx 1
noop
addx -38
addx 10
noop
noop
noop
noop
noop
addx 2
addx 3
addx -2
addx 2
addx 5
addx 2
addx -13
addx 14
addx 2
noop
noop
addx -9
addx 19
addx -2
addx 2
addx -9
addx -24
addx 1
addx 6
noop
noop
addx -2
addx 5
noop
noop
addx -12
addx 15
noop
addx 3
addx 3
addx 1
addx 5
noop
noop
noop
noop
addx -24
addx 29
addx 5
noop
noop
addx -37
noop
addx 26
noop
noop
addx -18
addx 28
addx -24
addx 17
addx -16
addx 4
noop
addx 5
addx -2
addx 5
addx 2
addx -18
addx 24
noop
addx -2
addx 10
addx -6
addx -12
addx -23
noop
addx 41
addx -34
addx 30
addx -25
noop
addx 16
addx -15
addx 2
addx -12
addx 19
addx 3
noop
addx 2
addx -27
addx 36
addx -6
noop
noop
addx 7
addx -33
addx -4
noop
addx 24
noop
addx -17
addx 1
noop
addx 4
addx 1
addx 14
addx -12
addx -14
addx 21
noop
noop
noop
addx 5
addx -17
addx 1
addx 20
addx 2
noop
addx 2
noop
noop
noop
noop
noop

"
    .to_string();
}
