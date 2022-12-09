const { strToGrid, printGrid } = require('./utils');

const input = `
30373
25512
65332
33549
35390
`;

const parsed = strToGrid(input);

printGrid(parsed)
