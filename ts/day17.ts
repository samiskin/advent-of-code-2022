import { hash, getDimensions, hashToPoint, strToGrid } from './utils';

let input = `

>>><<<<><<><<<>>>><>>>><<><<<<>>>><<<<>><<>><<<>><<>>>><<<>>>><<<<>>><>><<<<>>><<<<>>>><<<>>>><<<>><<><><<>>><<<<>>>><<<><<<<>>><<<<>>>><><<<<><<<>>>><<<<>>><<><<<<>>>><<<>>>><<<><<<<>>><<<<>>><<<<>><<<<>>>><<><<<>><<<><<<>>>><<<<>><>><<><<<<><<<>>>><<<>><<<>>>><>><<<>>>><<>>><>>>><>>><<<<><<<>>><<<>>><<<<>>>><<<><<<<><>><<<>>>><>><>><>><>><<>><<<>>>><<><>><<<<>><>><<>><<<<>>><<<>><>>><>><<>>><>><<<<>><<<<><<<<>><<<>><<<>><<>><>>><<<<><<<>>><>>><>>>><<>><<<<><<<<>><<<>>>><<<<><<<>>><<<>>><<<>>>><<><<<>>>><>>><>>><<<<>>><<<<>>>><>>><<<<>><<>><<<<><><<<><>>><<<<>>><<><><<<<>>><<>>>><>>><>><<>>>><<>><<>>><>><<<><>>><<>>>><<><><<<>><<<<>><><<<<>><<>>>><<>>><>>><>>><>><<<>>>><<<<>><<><<<>><>>><<<>>><<<>>><<<<>>>><<<>>><<<<>>><<<><<>>>><<<<>>><<<<>><<<>>><<>>>><<<<><<<<>><<><<<<>>><<<<><<<><<<>><>>><<<>>><>><<<><<<<>><<<>>>><>>>><<<><<<>>><>>><<<>>><<<><>>><>><<<<>>><<>>><<>>>><><<<<>>><>><>>>><>>><<<<><<<>>><<>>>><>><<<>>>><<<>><><<<><<<>><<<><>>><><<<<>><<>>>><><<<>>>><>>>><<<>>>><<<<>>>><<<<>>><<<><<<>>>><>>>><<<<>><<<<>>>><<>><>><<<<>><<><<<>><<<<>>>><>><<<<><<<>><>>><<<<>><>>>><>>>><><<><<<<>><<<>><<>>><<<>>><<<<>>>><<<>>>><<<>><<<<>><<<<><>>><><<<<><<><<<><<>>>><<<><<><<>>>><<>>>><>>>><>><<<<>>>><<<<><>><<>>><><<<<>>>><<>><><<>>><<<<>>><<<>>><<<>><>>><<<<>>>><>>>><>>><<<>>>><<>>><>>>><<>>>><<<<><>>>><<>><<<><>>><<<>>><>>>><<<>>>><>><<<>>>><>><><<<<>>><<<<>><<<>>>><>>>><<<<>><<<>>>><<><<<><<<>>><<<<>>>><<<><<>><>>><<<<>>><<<>>>><>>>><<<<>>>><<>>><<<>><>>>><<<<>><<<<><>>><<<<>>><>>>><<<<>>><>>>><<><<>>>><<>>><<<<>>>><<<<>><<<<><<<>>><<<<><<<><>>>><>>><<>><<<<>><><>><<>>><>>>><><<<>>>><<>><<<>>>><<>><<><<>>>><<<<>>>><<<<>>><<<<>>><>><<<>>><<<>>><<<<>><<><>>>><<<>>><<>>>><<>>><<>>><<<<>>>><<<<>>>><>><<<>><<<<>>>><<<>>>><><><<<<>>><<<>>><<<<>>><<<><<<<>><<<>>><<<>><<>>>><<>>><<>>>><<<><<>>>><<><<<><<<>>><<<<>>><>>><<<<>>><>>><<<>>>><>>><<<><<><>>><<>>><<>>>><<<<>><<<<><><<<<>>>><<<<>><<<><<<>><<<>>>><<>><<>>>><<><<>>><<<<><<<>><<<>>><>><<>><<<<>>>><<>>>><<<><>>><<<><<>>><<<<>>>><<<>>><<>>>><<<<>>>><<<>>>><<<>><<<>><<<<><<<<><<>>><<<<>><<<>>><>><>><><<<>>>><<>>><>>><<><>>>><<>>><<>>><<><><<<<>><<<>><<<<>>>><>>>><>>><<<<>><<<><<<>>><<<<><<><>>>><<<>>><<<><<>>>><<<>><<<<>>><>><<<<>><<<>><><<<<><<<<>><>><<<>>>><<<>>><<<>>><>>><<<>><<>>>><<<>>><<<<>><>>><<>>>><>><<<<>>>><>><<<>>>><>>><<<<>>><><<<>><<<>>><>><<<<><<<><<<>>>><>>>><<<<><>>>><<>>>><>>><<>>><>>><<<<>>><<<><>>>><>><<>>><<<<>><<<><<<><>>>><<<<>><<>><<<<>>>><>>>><><<<<>><<<>>>><<<>>>><<<>>>><<>>>><<<><>>>><<<>>>><<<<><<<>><<<<><><<<>><<>><<>><<><<<<><<>><<<<>>>><<<>>><<<><>><<>>>><<>>><>><<><<<><>>><<><>><<<><>><<<><<<>><<>>><<<>>>><<<<>><<<>>><>>><<<<>>><><<<<>>><<<<>>>><>><<>>>><<<<>>><<<>>><><>>><><<<<>>>><<<>>>><<<<>><<><<<<>>>><<><>>><<<>>><<>>><<<><<>>>><<<<>>><<<>>>><<<<>><<<>>>><<<>><<>><><<<>><<<<>>>><>><<<<>><<><<<<>>>><>><<>>>><>>>><><<<<>>>><<>><>>>><<>>>><<><<<<><>>>><>>><<>>><<<><<>>><<>>>><>>>><>>><>>>><<<>><<<<>>><<>>><<<<>>><><<>><<>><<<>><<>><<<>>>><<>>>><<<<>>><<<>>>><<<>>>><><<<<>><<>><<<<><<<>>><<<>><<<<>><<<<>>>><<<<>><<<>><<>>>><<<<>>>><<<<>>><<<<>>>><<>>><<>>>><<>><<<>>><<>><><<<>><>>>><<<>>><>>><<<>>>><<<<><>><<<<>><<>>>><<>><<<<>>>><<<><<><>>>><>>>><<><<<>>>><<>><<<<>>>><<<<><>><<<>><>>><<<<><>>>><<><<>>>><<<<>>>><<<<><<<>><<>><<>><>>><<<><<<<>>>><<>><<>>>><>>>><<<><>>>><<<<>>>><<<<>>>><<>>><<<<>>><<<>><>>>><<<><<>>><<>>>><><<<>><>>>><<<><<>><<>><<<>>><>><>><<>>>><>>><<><<>><<<><>>>><<<<>>><<<>><<<><<<<>><<<><>><<<><<>>><<<>>><>><>><<<>>>><<<><>>>><>>><<>>>><<<>><<<<>>>><<<>><>>>><<><<<>><>>><<<>>><<>>>><><>><<<>>>><<<><>>>><<<>>><<<>><<>>><>><>>>><<<<>>>><<><><<<<>>><>><<<<><>><<<>>>><<<<>>><<<<>>><<>><><<<<>><<<<>>>><<<<>>><<<>>>><<<<>>><<<<>>><<<>><>><<>>><<<>><>>><<><<<>>><>>><<<><<>>><>><<<<><<<>>><>><<<<>>><><>>>><<>>>><<<>><<<>>>><<<<><<>><>><>>>><><<<<><<><<><<<>>><><<<<>><<>><<<>>>><<>>><<<>><>><>>>><<>>>><>>>><>><>>>><>>><>><<<<>><<<<>>>><<<><<<><<<<>><>>><<>><<<<>><<<<>>><<<<>>><<<>>>><<<<>><<<>><<<><<<><<<<>>>><<<><<>>><<<>>><<><<<<>><<>>><><<<<>><><<<>>>><<<>>>><<<<>>>><><<>>><><><<>><>><>><<<>>><>><<<>><<<<><<<<>>>><<<>>><>>><>>>><<><<>>>><>>><><>><<>>>><<<>><<<<>><>>>><>>><<<<>>><<>>>><><<<>>><>>><>>><<<<>>>><<>><<<>>>><<>><>>>><<>><<>>><<<<>>>><>>><>>>><<<<>>><<<<>><>>>><<<>><><<<<>>>><<>>><<><<><><<<>>>><<>>>><<>>><>>><<<>>><>>><<<>>>><<<>><<<<>><<>>>><<><<<><<<>><<<<>>>><<<>><<>>><<<<>>><<<>>><<<<><<><>>>><<<<>><><<><<<>><<<<><<<>><<<>>><<>><<<<>>><<<><<<<>><<<<>>><><<<>>><<<<>>><<>>><>>>><<>>><<>><<<><<<<>><<>>>><<<<>>><<<<>>>><<<>><<<<>>>><<<<>>><<<>>><>><<><<<<><<<<>>>><<<><<><<<<><<<<>><>><<<>><<><<<><<><<<>><<<<><>>>><<<>>>><>><<>>>><>>>><>>>><<<>><<<>><<<<>>>><>>>><<<>>><<>>>><<<>>><<>>>><<<<>>>><><>>><><<>>><<<<>>>><>>>><<<>>>><<>>><<>><<<>>><<<<>><<<<>><<>><<<>><<>>>><>>><><<<<><><>>>><<<>>>><<<<>>><<>><>><<<<><>>><<<<>>>><<<<>><>>><<<>>><>>>><><>><>>><><>>>><><>>><<>><<><<<<>><<<<><<<<><<>><<<<><<<>><<<>>>><><<<<>>><<><<<<>>>><>>>><>>><<<>>><<<>>>><<>>><<><>>><<<><>><<<<><<<<><><<<>><><<>>>><<<>><<><<<<>><<>>><>><<>>><>><<<><>>><<>>>><<<<>><>>>><<<>>><<<>>><<>>>><<<>>><<<><>><<>>>><<>><>><<<<>><<<>>>><><<<<>>><>>><<>>><<<<>><<<>>><<>>>><<<>>>><><<<>><<>>>><<<>>><<<<><<<<>><<<<>><<<<>><<>>><<<>><<<>>>><<<<>>><<>>><<<<>>><<<>><<<<><><>><<>>>><<>>><<<<><<<><<<<><<<><><<<>>><>>>><<<<>>>><<><<<>>>><<><<<><><<>><<<<>>><<<><<<>><<<>>><>>><<<<>>><>><<>>>><<<<><<<>><<><>>><<<<>>>><><<>><>>>><<<>>>><<<<>>>><<<>>>><>>><<<>>><<<<><<<<>>>><<<<>>><<><<<>><<<<>><<<>>>><<<<>><<>>><>>><<<<><><<<>><>><>><<<>><<<<>>>><<>><>><<<>>><><<<<>><<>><<<><<>>>><><<<><<>><>><<>>>><<<<>><>>><<<<><<<><<>><<<<>><<<<>>>><<>>><<<<>>><<<>>>><<>>><<<><>>>><<<<>><<<<>><<>>>><<<>>>><<>><<<>>><>><<<<>>>><>><<<<><<<>><<<<>><>><>>>><<<>>>><><><>>>><<<<>>><<<>>><<<<><<>><<><<>>>><<<><<<<>>>><<>>>><<>>><<>><<<<>><<<<>>><>>><>>>><>><<<>>>><<>><<<><<<<><<<>>><<<>>>><<<><<>><<>>><><<<<>><<><<<<><<<>>>><<<<>><<>><<><<<>>><<<<>><<>>><<<>>><<<>><<<>>>><>>>><<>>><<<<>>><>>>><<<><<<>>><<<>><>>><>><<<>>><>>><<<>>><<<<><<<><<<<>><<<<>><<>>><>>><<<<><<>>>><<<<><<>>>><>>>><>>><<>>>><<>>>><<<<><<>><<>><<><>><><<<<>>><>>><<>><<<<>>>><<<<>><>><<<<>><<<>>><<<<>><<>><<>>><<<>>><<<<>><<<<><>><<<<>>><>>>><>><<>>>><>>>><<<<>><<>>><<>>><<>>>><><<><<<<>><<<<><<>>><>>><<<<><<<<>>><<<<>><<<>><<>>>><<<<><<>>>><<<>>>><<<<>>>><<<<><<<<>><<<<>>>><<<>>>><>><>>><<<<>><<<>><<<>><<>><<<<><>><<>><><<<<><<<<>>><<><<<<>><<<>>>><<<><<<<><<>>>><>>><>>><<<>><<<>>>><>>>><<>>><<<<>>><<>><><<<>>>><<><>>>><>><>>>><<<>>>><>>><<>>><>><>>><<<<><<>><<>><>>><>>><<<>><<><<>><><<>>><<>><<>><<<>><<<>>>><>>>><><<<<>>>><<<><<<>>><>><<<<><<<<>>><>>>><<>>>><<>>><>>>><>>>><<<<>>>><<>>><<<<>>><<<>><>>><<<<>>><>><<>>>><<>><<>><><><<<>>>><><<<>>><<<>><<>>>><<>>>><<>>><<>>><<>>><<<><<<<>><<<<>><<<<>><<><<<<>>>><<><<<<>>>><<<<>><<<>>>><<<>><<<<><<<<>>>><<<>><>>>><><>>>><<<<>><<<>>>><<<><<<>>>><<<<><<<>><><<>>><<<<><<>><<<<>>><<<<><<<<>><<<<><<><<<<>><><<<>>><>>>><<<<>><<<>>><<>><<<><>>>><>>><<<<>><>>>><<<><<>>>><<<<>>><<<>>><<><<<<>>>><<<>>><<>>><>>>><<<>>>><>>><<<<><>>>><<>>><<<><>>>><>>>><<<><<><<>><<<<><>>><><<<>>><<>>>><<<<>>><>><<<>><<<>>><<>>><<>>><<<>><<<<>><<<>>><<<>><<<><<<<>>><>>><<<<><<<><<<>>><>>><<><<<<>><<<<><<>><<>>>><<<><>>>><<>>>><<<><>><><<<<><<<><>>><<<<>><>><<<><<><<<<>>><>>><<<>><<<>><>>><<><<<<>>>><<>>>><><><>>>><<>>><<>>><<<<>><<><<<<>><>>><><<<<>>><<<<><<<<>>><<<>>><<<<>>>><<>>>><<<<>><<<>>>><<>>><<<<>><<><<<<><<<><<<><<<<>>><<<<>>>><<>>><<<<>><>><<>><<>>>><>>><<<<><<<<>>><<<>><<><<>>><>>><<<><>>><>>><><>><<<>>>><<<<>>><>><<<><<<>>>><<><<<<>>><<<>>>><<<><<>>>><<<<>>><<><<>>><<>><<>>>><<>>>><<<>>><<<<>><<>><<<<><<<>>><>>>><<>>><>>><<>>><<<>>>><>>>><>>>><<<><>>><<<<>><<<>>><<<>>>><<<<>>>><>>><<<<>>>><<<>>>><>>><<>><<<>>>><<<>><<<>>>><><><>>><<><>>><>>>><>>>><<<<>>><<<><<>>>><<<>>>><<<><>><>>><<<>>>><>><<>><>><<<>>>><<<<>>><<<<><<<<>>>><<<<>>><<>>>><<<<>>>><<<<>><<<>><><<<>>><<<<>>><>>>><<<<>><<>><>><<>>><>>>><<<<><<<><<>>><<<>><<<<>>><<>>><<<>>><<<<><<<>>>><<<>>>><<<>>><>>>><<>>>><<>><<><<>>><><<>><<<<>>>><<<><>>>><<<>><<<<>>>><>>><<<><>>><<<<>>><><<>>>><<<<><<<<>><>>><<<><>>>><<<<>><>>>><<<><<<>>>><<<>>><<<><<<>><<<<>>>><<<>><<<<><<<><<><<<<>>><<<<>>>><<><<<<>>>><><<<<>>>><<<>>><<<>>><<<<><>><<<<>><<<><<<<>>><<><<<<><<>>><<<><<<<><>>><<<<><<<>>>><<<<>>>><<>>>><>><<<<>>><><<<<>>>><<<<>><<<>><<<><<>>><<>>>><>>><<>><<<>><<>>><>>>><<>>>><>>><<<<>><><>>>><<><<<<>>>><<<<>>>><<>><>>><<<>>><<>>>><>>>><>>>><<<<>><<<<>><<<<><<<>>>><>>>><<<<>>>><>><<>>>><<<<>>><<<>>><>><<><<<>>>><>>><<>><<<><>><<<<>><<><>>><<<<><<<>><>>><<<>>>><<>>>><<<<>>><<>><>>><<<<>><<>><<<<>>><><<>>><<><<<<><<<<>><>>><<><<<><<<<>><>>><<>>>><<<>><<<>><<<>>><<<>>>><>>>><<>>><<<>>><>><>>>><>>>><<<>><>>><<<>><<>><>><<<<>>><>>><<<><<<>>>><<<>><<<>>><>>><<<><>>>><<<<><>><<>><<<<><<<>><<>>><<<><>>><<<><<<<><<<<><>><<>>>><<<<>>><<<<>><<<<>>><<<<><<><<<>>><><<>><<>>>><<>>><><<<<><<<<>>><><<<>>>><<<<>>>><<<<>>>><<>><<>><<>><<<>><>>><>><<<<>><<><<<>>><>>><><>>><<<>>>><<>>><>><<<>>>><<<>><<><><<<><>><<<<>>><<<<>><<<<>><<<>>><<<<>>><<<<>>>><<<>>><<<<>><<<<><><<<>>><<<><<><<><<<>><<>>><<>><<><<>>>><>><<<<>>>><>>>><<<<>>><<<<>>><>><<<><<<>>>><<<<>><<<<><<><<<<>>>><<<<>><<<<>>>><<<<>><<<>><<>><<<<>><>><<>>>><<<<><<<><>>><>><<<>>>><><<<><>>>><<<>>><<<>><<<><<<>><<><>>>><<><>>><>><<<>>><<<<>><<><<>><<<<>><<<<><<<>>>><<<<>><<<>>>><>>><<<>><<>>>><<><><<<<><<<<>>>><<<>><<<<>><>>>><<><><<>>>><<<><<<<>><>><>>><<><<<<>>>><<>><<<<><<>>><<<>>><<<>>><<<>>>><>><<<>>>><<<>>>><><<<<>>>><>><<<<>><<<<>>>><<>>><<<>>>><<>><<><<<>>><<>>>><<<<>><<<<>>><>>>><<<>>><<>>>><<>>><><>><<<<>><<<>>>><<>><<<>><<<>>><<<>><>><><<<<>>><<<><<>>>><<>>>><<>><<<><<<><<<<><<><>>>><<<>>>><<<<>>><>><<>>>><>><<<>>>><<<>><<>>>><<<<>>><<<><><<>>><<>><<<>>><<<<>><<>>>><<<><<<<><<>>>><<>>><<<<><<<<><<<>>><<>>><>><<><<><<>>><><>>>><<<>><<>>>><>>>><<<<>><><<<<>><<<<>>>><<>>>><<<<><<<>>><<>>><<><<<<>><>><<><<><<><<<><<<<><>><<<>>>><<<<>>>><<<><>>>><<>>><><>>><<<>>><<<><<>>><<><><<><<<<><<<<>>>><><><<<<>>><>><<>>>><<<<>>>><<<>>><<<><<><<<<><<<<>>><<<<>>>><<<>><<<<>>>><<><<<>><><>>>><<<>>>><<<>>>><<><<<>><>>>><<<><>><<<<><<<<>>><<<<>>>><<><<<>>>><><><<<<><<<<><>><<><>>>><<<>><<>><

`;

// input = `
//
// >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
//
// `

const parsed = input.trim().split('');

const blocks = `
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
`.trim().split('\n\n').map((b) => {
  const grid = strToGrid(b);
  const height = getDimensions(grid)[1] - 1;
  return Object.entries(grid).filter(([_, v]) => v == '#').map(([k]) => hashToPoint(k)).map(([x, y]) => [x, y - height] as [number, number])
})

const width = 7;
const collides = (occupied: Set<string>, block: [number, number][]) => {
  if (
    Math.max(...block.map(([x]) => x)) >= width ||
    Math.min(...block.map(([x]) => x)) < 0
  ) {
    return true;
  }
  if (
    Math.max(...block.map(([, y]) => y)) >= 0
  ) {
    return true;
  }
  if (block.find((coord) => occupied.has(hash(coord)))) {
    return true;
  }
}

const getFloors = (occupied: Set<string>, floor: number) => {
  let floors = [];
  for (let x = 0; x < 7; x++) {
    for (let y = floor; y <= 0; y++) {
      if (occupied.has(hash([x, y])) || y == 0) {
        floors.push(floor - y);
        break;
      }
    }
  }
  return floors;
}

const runRocks = (maxRocks: number) => {
  let occupied = new Set<string>();
  let floor = 0;
  let floorsSeen = {};
  let j = 0;
  let skipped = 0;
  let jumped = false;
  for (let r = 1; r <= maxRocks; r++) {
    let block = blocks[(r - 1) % blocks.length].map(([x, y]) => [x + 2, floor - 4 + y]);
    let collided = false;
    while (!collided) {
      const jet = parsed[j];
      const dx = jet == '<' ? -1 : 1;
      const shifted = block.map(([x, y]) => [x + dx, y] as [number, number]);
      if (!collides(occupied, shifted)) {
        block = shifted;
      }

      const fallen = block.map(([x, y]) => [x, y + 1] as [number, number]);
      if (!collides(occupied, fallen)) {
        block = fallen;
      } else {
        collided = true;
      }

      j = (j + 1) % parsed.length;
    }

    block.forEach(([x, y]) => {
      floor = Math.min(floor, y);
      occupied.add(hash([x, y]));
    })

    const hashedFloors = JSON.stringify([r % blocks.length, j, getFloors(occupied, floor)]);
    if (!jumped && floorsSeen[hashedFloors] != undefined) {
      jumped = true; // just do 1 jump

      const [pastR, pastFloor] = floorsSeen[hashedFloors];
      const [dRocks, dFloor] = [r - pastR, floor - pastFloor];
      const numSkips = Math.floor((maxRocks - r) / dRocks)
      r += numSkips * dRocks;
      skipped = numSkips * dFloor;
    }
    floorsSeen[hashedFloors] = [r, floor];
  };

  return floor + skipped;
}

console.log(runRocks(2022));
console.log(runRocks(1000000000000));

