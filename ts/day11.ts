import { cloneDeep } from "lodash";

const input = `

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

`

type Monkey = {
  id: number,
  items: number[],
  divisor: number,
  opt: (old: number) => number,
  next: (val: number) => number,
  inspections: number,
}

const parsed: Monkey[] = input.trim().split('\n\n').map((ms) => {
  const [monkeyStr, startStr, optStr, testStr, tr, fl] = ms.split('\n').map((s) => s.trim());

  const id = parseInt(monkeyStr.split(' ')[1][0], 10);
  const items = startStr.substring(16).split(', ').map((s) => parseInt(s, 10));
  const [, , , , optChar, valStr] = optStr.split(' ');
  const opt = (old: number) => {
    const val = valStr == "old" ? old : parseInt(valStr, 10);
    switch (optChar) {
      case '*': return old * val;
      case '+': return old + val;
    }
  }

  const divisor = parseInt(testStr.split(' ').pop(), 10);
  const targetTrue = parseInt(tr.split(' ').pop(), 10);
  const targetFalse = parseInt(fl.split(' ').pop(), 10);
  const next = (val: number) => val % divisor == 0 ? targetTrue : targetFalse;

  return {
    id, items, divisor, opt, next, inspections: 0,
  }
});

const runMonkeys = (rounds: number, divisor: number = 1) => {
  const monkeys = cloneDeep(parsed);
  const divisorMults = monkeys.reduce((s, { divisor }) => s * divisor, 1)

  for (let round = 1; round <= rounds; round++) {
    for (let monkey of monkeys) {
      while (monkey.items.length > 0) {
        const item = monkey.items.shift();
        const worry = Math.floor(monkey.opt(item) / divisor) % divisorMults;
        monkey.inspections += 1;
        const next = monkey.next(worry);
        monkeys[next].items.push(worry);
      }
    }
  }
  const max = monkeys.map(({ inspections }) => inspections).sort((a, b) => b - a).slice(0, 2).reduce((s, n) => s * n);
  return max;
}

console.log("Part 1:", runMonkeys(20, 3));
console.log("Part 2:", runMonkeys(10000));








