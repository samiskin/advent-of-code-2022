const input = `

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

`;

const parsed = input.trim().split("\n");

const priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

let totalP1 = 0;
for (let line of parsed) {
  let usedLeft = new Set(line.slice(0, line.length / 2));
  let usedRight = new Set(line.slice(line.length / 2));

  for (let c of usedRight) {
    if (usedLeft.has(c)) {
      totalP1 += +priorities.indexOf(c) + 1;
    }
  }
}
console.log(totalP1);

let totalP2 = 0;
for (let i = 0; i < parsed.length; i += 3) {
  let usedA = new Set(parsed[i]);
  let usedB = new Set(parsed[i + 1]);
  let usedC = new Set(parsed[i + 2]);

  for (let c of usedC) {
    if (usedA.has(c) && usedB.has(c)) {
      totalP2 += +priorities.indexOf(c) + 1;
    }
  }
}
console.log(totalP2);
