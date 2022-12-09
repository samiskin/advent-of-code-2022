import { hash, clamp, repeat } from './utils';

const input_p1 = `

R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2

`
const cmds_p1 = input_p1.trim().split('\n').map((s) => s.split(' ')).map(([d, n]) => [d, parseInt(n, 10)] as [string, number])

const input_p2 = `

R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20

`

const cmds_p2 = input_p2.trim().split('\n').map((s) => s.split(' ')).map(([d, n]) => [d, parseInt(n, 10)] as [string, number])


const computeTrail = (cmds: [string, number][], chain: [number, number][]) => {
  const head = chain[0]
  const tail = chain[chain.length - 1]

  const visited = {};
  for (let [dir, n] of cmds) {
    for (let i = 0; i < n; i++) {
      switch(dir) {
        case 'R': head[0] += 1;
        break;
        case 'L': head[0] -= 1;
        break;
        case 'U': head[1] += 1;
        break;
        case 'D': head[1] -= 1;
      }

      for (let i = 1; i < chain.length; i++) {
        const [hx, hy] = chain[i - 1]
        const [tx, ty] = chain[i]
        const dx = Math.abs(tx - hx)
        const dy = Math.abs(ty - hy)
        if (dx > 1 && dy >= 1 || dy > 1 && dx >= 1) {
          chain[i][0] += clamp(hx - tx, -1, 1)
          chain[i][1] += clamp(hy - ty, -1, 1)
        } else if (dx > 1) {
          chain[i][0] += clamp(hx - tx, -1, 1)
        } else if (dy > 1) {
          chain[i][1] += clamp(hy - ty, -1, 1)
        }
      }

      visited[hash([tail[0], -tail[1]])] = '#'
    }
  }
  return Object.keys(visited).length
}

const chain_p1 = repeat([0, 0] as [number, number], 2);
console.log("Part 1:", computeTrail(cmds_p1, chain_p1))
const chain_p2 = repeat([0, 0] as [number, number], 10);
console.log("Part 2:", computeTrail(cmds_p2, chain_p2))
