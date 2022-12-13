import { isEqual } from "lodash";

const input = `

[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]

`;

type Entry = number | Entry[]

const parsed = input
  .trim()
  .split('\n\n')
  .map((s2) => s2.split('\n').map((s) => eval(s) as Entry[]))

const checkList = (l: Entry[], r: Entry[], start: number = 0) => {
  const vl = l[start]
  const vr = r[start]
  if (vl == undefined && vr == undefined) {
    return 0;
  } else if (vl == undefined && vr != undefined) {
    return 1;
  } else if (vr == undefined && vl != undefined) {
    return -1;
  } else {
    if (typeof vl == 'number' && typeof vr == 'number') {
      if (vl < vr) {
        return 1;
      } else if (vr < vl) {
        return -1;
      } else {
        return checkList(l.slice(1), r.slice(1));
      }
    } else if (typeof vl == 'number') {
      return checkList([[vl], ...l.slice(1)], r);
    } else if (typeof vr == 'number') {
      return checkList(l, [[vr], ...r.slice(1)]);
    } else {
      const cmp = checkList(vl, vr);
      if (cmp == 0) {
        return checkList(l.slice(1), r.slice(1));
      } else {
        return cmp;
      }
    }
  }
}

console.log("Part 1:", parsed.map(([l, r], i) => {
  return [checkList(l, r), i + 1];
}).filter(([t]) => t == 1).reduce((s, [_, i]) => s + i, 0))

const allPackets = parsed.flatMap((l) => l).concat([[[2]], [[6]]]);
const sorted = allPackets.sort((a, b) => {
  return checkList(b, a);
});
const idx2 = sorted.findIndex((l) => isEqual(l, [[2]])) + 1
const idx6 = sorted.findIndex((l) => isEqual(l, [[6]])) + 1

console.log("Part 2:", idx2 * idx6)
