let input = `

Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 15 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 3: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 4: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 4 ore and 15 obsidian.
Blueprint 5: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 6: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 7: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 7 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 9: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 18 obsidian.
Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 11: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 12: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 2 ore and 16 obsidian.
Blueprint 13: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 16 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 13 obsidian.
Blueprint 15: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 2 ore and 20 obsidian.
Blueprint 16: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 4 ore and 10 obsidian.
Blueprint 17: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 17 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 18: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 17 obsidian.
Blueprint 19: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 20: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 21: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 22: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 3 ore and 19 obsidian.
Blueprint 23: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 25: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 27: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 16 obsidian.
Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 8 obsidian.
Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 4 ore and 19 obsidian.
Blueprint 30: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.

`

// input = `
//
// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
// Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
//
// `;

const parsed = input.trim().split('\n').map((s) => {
  const [blueprint, oreOre, clayOre, obsidianOre, obsidianClay, geodeOre, geodeObsidian] = [...s.trim().matchAll(/\d+/g)].map((m) => parseInt(m[0], 10))
  let maxOre = Math.max(oreOre, clayOre, obsidianOre, geodeOre);
  let maxClay = Math.max(obsidianClay);
  let maxObsidian = geodeObsidian;
  return {
    blueprint, oreOre, clayOre, obsidianOre, obsidianClay, geodeOre, geodeObsidian,
    maxOre, maxClay, maxObsidian,
  }
})
console.log(parsed);

type Node = {
  ore: number;
  clay: number;
  obsidian: number;
  geodes: number;
  robots: {
    ore: number;
    clay: number;
    obsidian: number;
    geode: number;
  }
  minutesLeft: number;
}

const start: Node = {
  ore: 0,
  clay: 0,
  obsidian: 0,
  geodes: 0,
  robots: {
    ore: 1,
    clay: 0,
    obsidian: 0,
    geode: 0,
  },
  minutesLeft: 24,
}
let hash = ({ robots, ...rest }: Node) => JSON.stringify([...Object.values(rest), ...Object.values(robots)])


let totalQuality = 0;
let mulGeodes = 1;
for (let costs of parsed.slice(0, 3)) {
  let maxGeodes = 0;
  const cache = {};
  let toVisit = [start];
  while (toVisit.length > 0) {
    let node = toVisit.pop();
    // node.robots.ore = Math.min(node.robots.ore, costs.maxOre)
    // node.robots.clay = Math.min(node.robots.clay, costs.maxClay)
    // node.robots.obsidian = Math.min(node.robots.obsidian, costs.maxObsidian)
    // console.log(node);
    let hash = JSON.stringify(node);
    if (cache[hash] != undefined) {
      continue;
    }
    maxGeodes = Math.max(maxGeodes, node.geodes);
    // console.log(node);
    if (node.minutesLeft == 0) {
      continue;
    }

    const next: Node = {
      ...node,
      ore: node.ore + node.robots.ore,
      clay: node.clay + node.robots.clay,
      obsidian: node.obsidian + node.robots.obsidian,
      geodes: node.geodes + node.robots.geode,
      minutesLeft: node.minutesLeft - 1,
    }

    let geodeBots = next.robots.geode;
    let dreamGeodes = next.geodes;
    for (let i = 0; i <= next.minutesLeft; i++) {
      dreamGeodes += geodeBots;
      geodeBots += 1;
    }
    if (dreamGeodes < maxGeodes) {
      continue;
    }

    if (node.obsidian >= costs.geodeObsidian && node.ore >= costs.geodeOre) {
      toVisit.push({
        ...next,
        obsidian: next.obsidian - costs.geodeObsidian,
        ore: next.ore - costs.geodeOre,
        robots: {
          ...next.robots,
          geode: next.robots.geode + 1,
        },
      })
      continue;
    }

    toVisit.push(next);

    if (!(costs.maxObsidian <= node.robots.obsidian) && node.clay >= costs.obsidianClay && node.ore >= costs.obsidianOre) {
      toVisit.push({
        ...next,
        clay: next.clay - costs.obsidianClay,
        ore: next.ore - costs.obsidianOre,
        robots: {
          ...next.robots,
          obsidian: next.robots.obsidian + 1,
        },
      })
      continue;
    }


    if (!(costs.maxClay <= node.robots.clay) && node.ore >= costs.clayOre) {
      toVisit.push({
        ...next,
        ore: next.ore - costs.clayOre,
        robots: {
          ...next.robots,
          clay: next.robots.clay + 1,
        },
      })
    }

    if (!(costs.maxOre <= node.robots.ore) && node.ore >= costs.oreOre) {
      toVisit.push({
        ...next,
        ore: next.ore - costs.oreOre,
        robots: {
          ...next.robots,
          ore: next.robots.ore + 1,
        },
      })
    }
  }
  const quality = costs.blueprint * maxGeodes;
  console.log(costs.blueprint, quality, maxGeodes);
  totalQuality += quality;
  mulGeodes = mulGeodes * maxGeodes;
}
console.log(totalQuality);
console.log(mulGeodes);



