import { addInterval, hash } from "./utils";

let input = `

Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3

`;

const p1_y = 10;
const p2_max_coord = 20;
// const p1_y = 2000000;
// const p2_max_coord = 4000000;

const occupied = {};
const parsed = input.trim().split('\n').map((l) => {
  const [sx, sy, bx, by] = [...l.matchAll(/-?\d+/g)].map((s) => parseInt(s[0], 10));
  occupied[hash([bx, by])] = true;
  occupied[hash([sx, sy])] = true;
  return [sx, sy, bx, by];
})

const occupied_y = {};
const intervals: Array<Array<[number, number]>> = [];

for (let [sx, sy, bx, by] of parsed) {
  const radius = (Math.abs(bx - sx) + Math.abs(by - sy));
  for (let iy = sy - radius; iy <= sy + radius; iy++) {
    let dy = Math.abs(iy - sy);
    if (iy == p1_y) {
      for (let ix = sx - (radius - dy); ix <= sx + (radius - dy); ix++) {
        if (!occupied[hash([ix, iy])]) {
          occupied_y[ix] = true;
        }
      }
    }

    if (iy < 0 || iy > p2_max_coord) {
      continue;
    }
    let clampedInterval = [Math.max(sx - (radius - dy), 0), Math.min(sx + (radius - dy), p2_max_coord)] as [number, number];
    intervals[iy] = addInterval(intervals[iy], clampedInterval);
  }
}
const gapInterval = Object.entries(intervals).find(([, ints_y]) => ints_y.length > 1);


console.log("Part 1:", Object.keys(occupied_y).length);
console.log("Part 2:", (gapInterval[1][0][1] + 1) * 4000000 + parseInt(gapInterval[0], 10));
