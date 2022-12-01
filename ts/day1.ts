const testInput = `
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
`;

const sortedSums = testInput
  .trim()
  .split("\n\n")
  .map((s) => s.split("\n").reduce((sum, s) => sum + parseInt(s, 10), 0))
  .sort((a, b) => b - a);

console.log("Part 1:", sortedSums[0]);
console.log("Part 2:", sortedSums[0] + sortedSums[1] + sortedSums[2]);
