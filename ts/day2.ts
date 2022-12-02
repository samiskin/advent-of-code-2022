const input = `
A Y
B X
C Z
`;
const parsed = input
  .trim()
  .split("\n")
  .map((s) => s.split(" "));
const map: Record<string, string> = { X: "A", Y: "B", Z: "C" };
const win = { A: "C", B: "A", C: "B" } as Record<string, string>;
const lose = { A: "B", B: "C", C: "A" } as Record<string, string>;
const score = { A: 1, B: 2, C: 3 } as Record<string, number>;

const calcScore = (a: string, b: string) =>
  score[b] + (b == a ? 3 : a == win[b] ? 6 : 0);

const sumP1 = parsed.reduce((sum, [a, b]) => sum + calcScore(a, map[b]), 0);

const sumP2 = parsed.reduce(
  (sum, [a, cmd]) =>
    sum + calcScore(a, cmd == "X" ? win[a] : cmd == "Y" ? a : lose[a]),
  0
);

console.log("Part 1:", sumP1);
console.log("Part 2:", sumP2);
