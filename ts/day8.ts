import { mapValues } from "lodash";
import {
  strToGrid,
  inBounds,
  getDimensions,
  hashToPoint,
  objKeys,
  hash,
} from "./utils";

const input = `
30373
25512
65332
33549
35390
`;

const grid = mapValues(strToGrid(input), (c: string) => parseInt(c, 10));

let [width, height] = getDimensions(grid);
let walkDirs = ([sx, sy]: [number, number]): [number, number][][] => {
  const dirs = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
  ];
  const coords = [];
  for (let [dx, dy] of dirs) {
    const dirCoords = [];
    let [x, y] = [sx + dx, sy + dy];
    while (inBounds([x, y], width, height)) {
      dirCoords.push([x, y]);
      x += dx;
      y += dy;
    }
    coords.push(dirCoords);
  }
  return coords;
};

const visible = ([sx, sy]: [number, number]) => {
  for (let dirCoords of walkDirs([sx, sy])) {
    let collided = false;
    for (let [x, y] of dirCoords) {
      if (grid[hash([x, y])] >= grid[hash([sx, sy])]) {
        collided = true;
        break;
      }
    }
    if (!collided) {
      return true;
    }
  }
  return false;
};

const scenicScore = ([sx, sy]: [number, number]) => {
  let score = 1;
  for (let dirCoords of walkDirs([sx, sy])) {
    let treesSeen = 0;
    for (let coord of dirCoords) {
      if (grid[hash(coord)] < grid[hash([sx, sy])]) {
        treesSeen += 1;
      } else {
        treesSeen += 1;
        break;
      }
    }
    score *= treesSeen;
  }

  return score;
};

console.log(
  "Part 1:",
  objKeys(grid).filter((pos) => visible(hashToPoint(pos))).length
);
console.log(
  "Part 2:",
  objKeys(grid)
    .map((pos) => scenicScore(hashToPoint(pos)))
    .reduce((s, n) => Math.max(s, n), 0)
);
