const input = `
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
`;

const parsed = input
  .trim()
  .split("\n")
  .map((s) =>
    s.split(",").map((h) => h.split("-").map((i) => parseInt(i, 10)))
  );

console.log(
  "Part 1:",
  parsed.filter(
    ([[al, ar], [bl, br]]) => 
      (al <= bl && ar >= br) || // abba
      (bl <= al && br >= ar)    // baab
  ).length
);

console.log(
  "Part 2:",
  parsed.filter(
    ([[al, ar], [bl, br]]) =>
      (al <= bl && ar >= br) || // abba
      (bl <= al && br >= ar) || // baab
      (al <= bl && ar >= bl) || // abab
      (bl <= al && br >= al) // baba
  ).length
);
