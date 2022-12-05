const input = `

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

`;

const parsed = input
  .split("\n\n")
  .filter((s) => s != "")
  .map((s) => s.split("\n"));

const [stacks, directions] = parsed;

const cols = stacks.pop()!;
const stackMap: Record<string, string[]> = {};

for (let i = 0; i < cols.length; i++) {
  if (cols[i] == " ") continue;
  stackMap[cols[i]] = [];
  for (let r = stacks.length - 1; r >= 0; r--) {
    if (stacks[r][i] != " ") {
      stackMap[cols[i]].push(stacks[r][i]);
    }
  }
}

const moves = directions.map((s) => {
  const line = s.split(" ");
  const [n, from, to] = [line[1], line[3], line[5]].map((n) => parseInt(n, 10));
  return { n, from, to };
});

const p1Stacks = JSON.parse(JSON.stringify(stackMap));
for (let { n, from, to } of moves) {
  for (let i = 0; i < n; i++) {
    let c = p1Stacks[from].pop();
    p1Stacks[to].push(c);
  }
}

const p2Stacks = JSON.parse(JSON.stringify(stackMap));
for (let { n, from, to } of moves) {
  const tailN = p2Stacks[from].splice(-n, n);
  p2Stacks[to].push(...tailN);
}

const stackTopsStr = (map: Record<string, string[]>) =>
  Object.keys(map)
    .map((k) => map[k][map[k].length - 1])
    .reduce((s, c) => s + c, "");

console.log("Part 1:", stackTopsStr(p1Stacks));
console.log("Part 2:", stackTopsStr(p2Stacks));
