import { Graph } from "./utils";

const input = `

Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II

`; // 1651, 1707

const parsed = input
  .trim()
  .split("\n")
  .map((l) => {
    let [valve, flow, ...tunnels] = [...l.matchAll(/([A-Z][A-Z])|(\d+)/g)].map(
      (r) => r[0]
    );
    return [valve, parseInt(flow, 10), tunnels] as const;
  });

const flows = {};
const graph = new Graph((name: string) => name);
for (let [valve, flow, tunnels] of parsed) {
  flows[valve] = flow;
  graph.addNode(valve);
  for (let tunnel of tunnels) {
    graph.addEdge(valve, tunnel, 1);
  }
}

let shortcutGraph = new Graph((name: string) => name);
for (let [start] of parsed) {
  if (start != "AA" && flows[start] == 0) {
    continue;
  }
  let visited = new Set([start]);
  let toVisit = [{ node: start, distance: 0 }];
  while (toVisit.length > 0) {
    let { node, distance } = toVisit.shift();
    let neighbors = graph.getNeighbors(node).filter((s) => !visited.has(s));
    for (let n of neighbors) {
      visited.add(n);
      if (flows[n] > 0) {
        shortcutGraph.addEdgeUni(start, n, distance + 1);
      }
      toVisit.push({ node: n, distance: distance + 1 });
    }
  }
}

type Node = {
  valve: string;
  opened: Array<string>;
  flow: number;
  daysLeft: number;
  withElephant: boolean;
};

const cache = {};
const elephantCache = {};

const getMaxPressure = (start: Node): number => {
  if (start.daysLeft <= 0) {
    return 0;
  }

  const hashed = JSON.stringify([start.valve, start.daysLeft, start.opened]);
  if (!start.withElephant && cache[hashed] != undefined) {
    return cache[hashed];
  } else if (start.withElephant && elephantCache[hashed] != undefined) {
    return elephantCache[hashed];
  }

  let max = 0;
  for (let [n, cost] of shortcutGraph.getNeighborsWithCost(start.valve)) {
    if (start.opened.includes(n) || start.daysLeft - cost < 0) {
      continue;
    }
    let newOpened = [...start.opened, n];
    newOpened.sort();
    max = Math.max(
      max,
      flows[n] * (start.daysLeft - (cost + 1)) +
        getMaxPressure({
          valve: n,
          daysLeft: start.daysLeft - (cost + 1),
          flow: 0,
          opened: newOpened,
          withElephant: start.withElephant,
        })
    );
  }
  if (start.withElephant) {
    max = Math.max(
      max,
      start.flow * start.daysLeft +
        getMaxPressure({
          valve: "AA",
          daysLeft: 26,
          flow: 0,
          opened: start.opened,
          withElephant: false,
        })
    );
  }
  if (max == 0) {
    max = start.flow * start.daysLeft;
  }

  if (!start.withElephant) {
    cache[hashed] = max;
  } else {
    elephantCache[hashed] = max;
  }
  return max;
};

const start: Node = {
  valve: "AA",
  opened: ["AA"],
  flow: 0,
  daysLeft: 30,
  withElephant: false,
};

console.log(getMaxPressure(start));
console.log(getMaxPressure({ ...start, daysLeft: 26, withElephant: true }));
