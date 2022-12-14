import { hash, printGrid } from './utils';
let input = `

498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9

`;

const parsed = input.trim().split('\n').map((l) => {
  return l.split(' -> ').map((s) => s.split(',').map((i) => parseInt(i, 10)));
});

const grid = {};

let max_y = 0;
for (let segments of parsed) {
  let [x, y] = segments.pop();
  max_y = Math.max(max_y, y);
  while (segments.length > 0) {
    let [nx, ny] = segments.pop();
    max_y = Math.max(max_y, ny);

    if (x != nx) {
      let [lx, rx] = [Math.min(x, nx), Math.max(x, nx)];
      for (let ix = lx; ix <= rx; ix++) {
        grid[hash([ix, y])] = '#';
      }
    } else if (y != ny) {
      let [ty, by] = [Math.min(y, ny), Math.max(y, ny)];
      for (let iy = ty; iy <= by; iy++) {
        grid[hash([x, iy])] = '#';
      }
    }
    [x, y] = [nx, ny];
  }
}

let floor = max_y + 2;


let sand = 0;
let sand_p1 = 0;
while (true) {
  sand += 1;
  let [sx, sy] = [500, 0];
  while (true) {
    if (sy + 1 >= floor) {
      if (sand_p1 == 0) {
        sand_p1 = sand - 1;
      }
      grid[hash([sx, sy])] = '#'
      break;
    } else if (grid[hash([sx, sy + 1])] != '#') {
      sy += 1;
    } else if (grid[hash([sx - 1, sy + 1])] != '#') {
      sx -= 1;
      sy += 1;
    } else if (grid[hash([sx + 1, sy + 1])] != '#') {
      sx += 1;
      sy += 1;
    } else {
      grid[hash([sx, sy])] = '#'
      break;
    }
  }
  if (sx == 500 && sy == 0) {
    break;
  }
}

console.log("Part 1:", sand_p1);
console.log("Part 2:", sand);
