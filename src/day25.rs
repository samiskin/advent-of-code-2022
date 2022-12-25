#[allow(dead_code)]
mod utils;
use std::collections::VecDeque;

#[allow(unused_imports)]
use crate::utils::*;

fn main() {
    // let input = _get_test_input();
    let input = _get_input();

    // ----------- Parse Input -----------

    let parsed = input.trim().split("\n").collect::<Vec<_>>();

    // ----------- Solve -----------
    let mut full_sum = 0 as i64;
    for s in parsed {
        let chars = s.chars().collect::<Vec<_>>();
        let mut mul = 1;
        let mut sum = 0;
        for i in (0..chars.len()).rev() {
            let val = match chars[i] {
                '1' => 1,
                '2' => 2,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            };
            sum += mul * val;
            mul *= 5;
        }

        full_sum += sum;
    }

    const BASE: i64 = 5;

    fn to_quinary_inner(val: i64, pow: u32, maxes: [i64; 21]) -> VecDeque<i64> {
        if val <= 2 {
            return VecDeque::from([val]);
        }

        let range = maxes[pow as usize - 1];
        let remaining;
        let mut digit_val = 0;
        loop {
            let center = BASE.pow(pow) * digit_val;
            if center + range >= val && center - range <= val {
                remaining = val - center;
                break;
            }
            digit_val += 1;
        }

        let mut rest = if remaining > 0 {
            to_quinary_inner(remaining, pow - 1, maxes)
        } else {
            let mut rest = to_quinary_inner(-remaining, pow - 1, maxes);
            for v in rest.iter_mut() {
                *v = -*v;
            }
            rest
        };
        rest.push_front(digit_val);
        return rest;
    }

    fn to_quinary(val: i64) -> String {
        let mut maxes = [0; 21];
        let mut sum = 0;
        for i in 0..maxes.len() {
            sum += 2 * BASE.pow(i as u32);
            maxes[i] = sum;
        }
        return to_quinary_inner(val, 21, maxes)
            .iter()
            .map(|v| match v {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => panic!(),
            })
            .collect::<String>()
            .trim_start_matches("0")
            .to_string();
    }

    // ----------- Print -----------

    println!("Part 1: {}", to_quinary(full_sum));
    println!("Part 2: {}", 1);
}

fn _get_test_input() -> String {
    return "
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122

"
    .to_string();
}

fn _get_input() -> String {
    return "

100=
110
1==2-
11=
12-0-0-=
1==01==2
111-2=2==-21
10-=-022
21=2=2-0=
22
11=01=0011-00-2
1=0-12
1=-2
12120=20=-120-=100
1-220-10120--=01=
10=000=-122--0=
1-001--2200
201-0-1--=2
1202-1
102101-0-2=0=0=1=2
120=0
2=--1=0012-=
100=12100220220
1-11211
1201=---
1=1=-
11==0200=
1-1--120220000
200==
1==-02-1=2-
1=-2=
1=-1==22-11--=1
12=1120=10=-1=10
2021=--12012-
122=0-122=02-2=
220====0--
1-1=1011-022=-=01
11=200-0100
2=-1==1=0100
100-2012-=10-00-21
210122011
1===210-=110-
1002201==0-
1--1211-1==0==-0
20==02-2120
1==
12=22201-=0
100101
12120-=002
10-0=--=220=-000
1=-1012=22=2-20-101
1==0=-20220-==
1=0
1=20110-==02000
2-11200
1=-0012
12-1121=2112=2-
1=-=2-21-02-==0-2-
1-=1==-=
20==
1220
21010==-=000-01
1-20=-==-0==-1=01=1=
2=22101
1-=12-0-11-102
2-0=11=2-=0=2-0
11=2=1-1020--0=
122102-1222
1-0020==0=-2-1=12
1=020-=0-
1-
10=-==1=-=-=22
20-12==0=-=-12
11111==
2=2--11-12
1=2--
11=00-10220-=
1=01--0
1-0
10=12==122
2-020
1=12=1=-222002-
11
202=-===0==01
1-0221=211
21100
10-2-==01
11=020-=10-1110-11=
1--0
1-0-12=0=2
1-1212012--=1-20--
1=-121200020100
2-=0-=-21-10-
221
1=0-=-
11-2-
1=-=200
210=1020-11022=1=0
110==--2-1-121==2=1
1200
20-==1=0-12
11=02=2
1-=2
22-0201=-2=-
1=0201122==2002-1
1-1=21002=1
1-=20=12-21121
12102
1--=211200
20=-===2-=1-1-1
2=0-1022=21---2
2=1
1-01111=11-
10210102=0121
11002=-1-1202=21=

"
    .to_string();
}
