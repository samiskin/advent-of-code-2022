export const _ = require('lodash');

type FixedLengthArray<N extends number, T, _Counter extends any[] = []> =
  _Counter['length'] extends N
    ? []
    : [T, ...FixedLengthArray<N, T, [null, ..._Counter]>];

export const splitOnce = (str: string, splitter: string) =>
  [ str.split(splitter)[0], str.slice(str.indexOf(splitter) + splitter.length) ]

export const eq = (a: any, b: any) => a === b;
export const neq = (a: any, b: any) => a !== b;
export const gt = (a: any, b: any) => a > b;
export const lt = (a: any, b: any) => a < b;
export const mid = (a: number, b: number) => ((b - a) / 2) + a;
export const midf = (...args: Parameters<typeof mid>) => Math.floor(mid(...args));

/** Returns an array of numbers from 0 to <num>, optionally skipping the first <skip> numbers */
export const range = (num: number, skip: number = 0) =>
  Array.from("x".repeat(num))
    .map((_x, i) => i)
    .filter(i => i >= skip);

/** Returns the numbers in the range [start, end) */
export const range2 = (start: number, end: number) =>
  Array.from("x".repeat(end - start)).map((_x, i) => i + start);

/** Simulates nested for loops each from 0 to args[i] */
export const rangeNested = (start: number, ...nums: number[]): Array<Array<number>> => {
  if (nums.length === 0) return range(start).map((i) => [i]);
  const nested = rangeNested(...(nums as [number, ...number[]]));
  return _.flatMap(range(start), ((i: number) => nested.map((nest: Array<number>) => [i, ...nest])));
};

/** Returns the permutations of size n of an array */
export const arrayPerm = <T>(n: number, arr: Array<T>): Array<Array<T>> => {
  if (n === 1) return arr.map((n) => [n]);
  const results: Array<Array<T>> = [];
  for (let i = 0; i < arr.length; i++) {
    let innerCombs = arrayPerm(n - 1, arr);
    for (let j = 0; j < innerCombs.length; j++) {
      results.push([arr[i], ...innerCombs[j]]);
    }
  }
  return results;
}

/** Returns the unique pairs in an array (n choose 2) */
export const arrayPairs = <T>(arr: Array<T>) => {
  let result: Array<[T, T]> = [];
  for (let i = 0; i < arr.length; i++) {
    for (let j = i + 1; j < arr.length; j++) {
      result.push([arr[i], arr[j]]);
    }
  }
  return result;
}

/** Returns the combinations of size k for an array (n choose k) */
export const arrayComb = <T>(k: number, arr: Array<T>, start = 0): T[][] =>
  k === 1 ? arr.slice(start).map((t) => [t]) :
  k === 2 ? arrayPairs(arr.slice(start)) :
    range2(start, arr.length)
        .flatMap(
          (i) => arrayComb(k - 1, arr, i + 1)
            .map((inner) => [arr[i], ...inner]));

/** Returns the combinations of size k for an array (n choose k) */
export const time = (...fns: Array<(...args: any) => any>) => fns.map((fn) => {
  const start = Date.now();
  const result = fn();
  return [Date.now() - start, result];
});

/** Prints colorized args */
export const print = (col: string, ...args: unknown[]) => {
  const getNum = (color = col) => {
    switch(color) {
      case 'r':
      case 'red': return 31;
      case 'g':
      case 'green': return 32;
      case 'y':
      case 'yellow': return 33;
      case 'b':
      case 'blue': return 34;
      case 'm':
      case 'magenta': return 35;
      case 'c':
      case 'cyan': return 36;
      case 'w':
      case 'white': return 37;
      case 'rst':
      case 'clr':
      case 'reset': return 0;
    }
  }
  console.log(`\x1b[${getNum()}m`, ...args, `\x1b[${getNum('reset')}m`);
}

/** hashes [x, y] into `{x} {y}`*/
type HashedCoord = `${number} ${number}`;
export const hash = ([x, y]: [number, number]) => `${x} ${y}` as const;

export const strToGrid = (arr: string) =>
  arr.trim().split('\n').reduce((map, line, y) => {
    line.split('').forEach((c, x) => {
      map[hash([x, y])] = c;
    });
    return map;
  }, {})

/** Prints a grid of `{x} {y}`: {character} to the console */
export const printGrid = <T>(map: Record<HashedCoord, T>, valOverrides = {}, posOverrides = {}) => {
  const points = Object.keys(map).filter((hash) => !!map[hash]).map((hash) => hash.split(' ').map((i) => parseInt(i)));
  const min_x = Math.min(...points.map(([x, _y]) => x));
  const max_x = Math.max(...points.map(([x, _y]) => x));
  const min_y = Math.min(...points.map(([_x, y]) => y));
  const max_y = Math.max(...points.map(([_x, y]) => y));

  for (let y_base of range(max_y - min_y + 1)) {
    let output = ''
    for (let x_base of range(max_x - min_x + 1)) {
      let [x, y] = [x_base + min_x, y_base + min_y];
      const posOverride = posOverrides[hash([x, y])];
      if (posOverride) {
        output += posOverride;
      } else {
        const value = map[hash([x, y])] ?? ' ';
        output += valOverrides[value as string] ?? value;
      }
    }
    console.log(output);
  }
}

/** Promisified version of setTimeout */
export const timeout = (time: number) => new Promise((resolve) => setTimeout(resolve, time));

/** Ensures a val is within the range [lower, upper] */
export const clamp = (val: number, lower: number, upper: number) => Math.max(Math.min(val, upper), lower);

/** Greatest Common Divisor */
export const gcd = (...nums: number[]) =>
  nums.reduce((ans, n) => {
    let x = Math.abs(ans);
    let y = Math.abs(n);
    while(y) {
      var t = y;
      y = x % y;
      x = t;
    }
    return x;
  });

/** Lowest Common Multiple */
export const lcm = (...nums: number[]) =>
  nums.reduce((ans, n) => (n * ans) / gcd(ans, n));

/** Returns the 4 / 8 neighbors of a coordinate depending on if diagonals are included or not */
export function getNeighbors([x, y]: [number, number], includeDiag: true): FixedLengthArray<8, [number, number]>;
export function getNeighbors([x, y]: [number, number], includeDiag?: false): FixedLengthArray<4, [number, number]>;
export function getNeighbors([x, y]: [number, number], includeDiag: boolean = false): Array<[number, number]> {
  return rangeNested(3, 3)
    .map(([dx, dy]) => [dx - 1, dy - 1])
    .filter(([dx, dy]) => includeDiag || (Math.abs(dx) + Math.abs(dy) < 2))
    .map(([dx, dy]) => [x + dx, y + dy])
    .filter(([nx, ny]) => nx !== x || ny !== y) as any
}

/** Executes visitor on every node that a BFS from start would reach  */
export function bfs<T>(options: {
  map: Record<HashedCoord, T>,
  start: [number, number],
  isWall?: (val: T) => boolean,
  visitor?: (pos: [number, number], dist?: number, parent?: [number, number]) => unknown
}) {
  const defaults = { isWall: (val: T) => !!val, visitor: _.noop };
  const { map, start, isWall, visitor } = { ...defaults, ...options };

  const visited = new Set();
  let toVisit: Array<[number, number]> = [start];
  const data = {
    [hash(start)]: { length: 0, parent: null },
  };

  while(true) {
    const pos = toVisit.shift();
    if (!pos) break;
    const { length, parent } = data[hash(pos)];
    visitor(pos, length, parent);
    visited.add(hash(pos));
    const next = getNeighbors(pos)
      .filter((n) => !visited.has(hash(n)))
      .filter((n) => !(isWall(map[hash(n)])));
    next.forEach((n) => {
      data[hash(n)] = {
        length: length + 1,
        parent: pos
      };
    });
    toVisit = toVisit.concat(next);
  }

  return data;
}

/** Executes the A* algorithm */
export const bfsPq = <State>(options: {
  start: State;
  hashState: (s: State) => string | number;
  getNeighbors: (pos: State) => Array<State>;
  isGoal: (s: State) => boolean;
  compare: (a: State, b: State) => -1 | 0 | 1;
  visitor?: (state: State, parent?: State) => unknown;
}): State => {
  const defaultOptions = { visitor: _.noop };
  const { start, hashState, getNeighbors, isGoal, compare, visitor } = {
    ...defaultOptions,
    ...options
  };

  const toVisit = new PriorityQueue([start], compare);
  // const toVisit = new TinyQueue([start], compare);
  const visited = new Set();

  while (toVisit.peek() !== undefined) {
    const state = toVisit.pop() as State;
    const cacheKey = hashState(state);
    if (visited.has(cacheKey)) continue;
    visited.add(cacheKey);
    visitor(state);

    if (isGoal(state)) {
      return state;
    }

    for (let neighbor of getNeighbors(state)) {
      toVisit.push(neighbor);
    }
  }

  return null;
};


/** Given a record of coord hash */
export const hashToPoint = (hash: string) =>
  hash.split(' ').map((i) => parseInt(i)) as [number, number];

/** Returns the [x, y] coords in a {`{x} {y}`: val} map */
export const getMapPoints = (map: Record<string, unknown>): Array<[number, number]> =>
  Object.keys(map).filter((hash) => !!map[hash]).map(hashToPoint) as any;

/** Returns [maxX - minX, maxY - minY] for a grid of coordinates */
export const getDimensions = (map: Record<string, unknown>) => {
  const points = getMapPoints(map);
  const min_x = Math.min(...points.map(([x, _y]) => x));
  const max_x = Math.max(...points.map(([x, _y]) => x));
  const min_y = Math.min(...points.map(([_x, y]) => y));
  const max_y = Math.max(...points.map(([_x, y]) => y));
  return [max_x - min_x, max_y - min_y];
}

/** Element-wise addition of two positions */
export const addPos = (p1: [number, number], p2: [number, number]): [number, number] => 
  [p1[0] + p2[0], p1[1] + p2[1]];

const heapify = <T>(arr: Array<T>, compare: (a: T, b: T) => -1 | 0 | 1, i: number = 0) => {
  const curr = arr[i];
  const [ left, right ] = [ 2 * i + 1, 2 * i + 2];
  if (arr[left] && compare(arr[left], curr) > 0) {
    arr[i] = arr[left];
    arr[left] = curr;
    heapify(arr, compare, left);
  } else if (arr[right] && compare(arr[right], curr) > 0) {
    arr[i] = arr[right];
    arr[right] = curr;
    heapify(arr, compare, right);
  }
}

/** Copied from stack overflow */
export class PriorityQueue<T> {
  data: Array<T>;
  comparator: (a: T, b: T) => -1 | 0 | 1;
  constructor(data: Array<T> = [], comparator: (a: T, b: T) => -1 | 0 | 1 = PriorityQueue.defaultCompare) {
    this.data = data;
    this.comparator = comparator;
  }

  peek = () => this.data[0];

  push = (e: T) => {
    this.data.push(e);
    this._bubbleUp(this.data.length - 1);
  }

  pop = (): T => {
    if (this.data.length === 0) return undefined;
    const top = this.data[0];
    const bottom = this.data.pop();
    if (this.data.length > 0) {
        this.data[0] = bottom;
        this._bubbleDown(0);
    }

    return top;
  }

  _bubbleUp = (pos: number) => {
    const compare = this.comparator;
    const item = this.data[pos];

    while (pos > 0) {
        const parent = (pos - 1) >> 1;
        const current = this.data[parent];
        if (compare(item, current) >= 0) break;
        this.data[pos] = current;
        pos = parent;
    }

    this.data[pos] = item;
  }

  _bubbleDown = (pos: number) => {
    const { data, comparator } = this;
    if (pos >= this.data.length) return;

    const halfLength = this.data.length >> 1;
    const item = data[pos];

    while (pos < halfLength) {
      let left = (pos << 1) + 1;
      let best = data[left];
      const right = left + 1;

      if (right < this.length && comparator(data[right], best) < 0) {
          left = right;
          best = data[right];
      }
      if (comparator(best, item) >= 0) break;

      data[pos] = best;
      pos = left;
    }

    data[pos] = item;
  }

  print = () => {
    const toIndex = (depth: number, offset: number) => (Math.pow(2, depth) + offset) - 1;
    const toLevel = (i: number) => {
      const depth = Math.ceil(Math.log2(i + 1));
      const offset = (i + 1) % Math.pow(2, depth);
      return [depth, offset];
    }
    const maxDepth = toLevel(this.data.length - 1)[0];
    for (let depth = 0; depth < maxDepth; depth++) {
      console.log(range(Math.pow(2, depth)).map((offset) => this.data[toIndex(depth, offset)]));
    }
  }

  get length() {
    return this.data.length;
  }

  static defaultCompare<T>(a: T, b: T) {
      return a < b ? -1 : a > b ? 1 : 0;
  }
}

export const intersperse = <T, V>(arr: Array<T>, separator: V) => {
  return arr.reduce((acc, el, i) => {
    acc.push(el);
    if (i < arr.length - 1)  {
      acc.push(separator)
    }
    return acc;
  }, [] as Array<T | V>);
}
