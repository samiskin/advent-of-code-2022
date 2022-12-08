const input = `

$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k

`;

const parsed = input
  .trim()
  .split("$ ")
  .filter((s) => !!s)
  .map((s) => s.trim());

type Directory = number | { [path: string]: Directory };

let root: Directory = {};
let parentStack = [];
let activeDir: Directory = root;
for (let cmd of parsed) {
  if (cmd.startsWith("cd")) {
    let [_, path] = cmd.split(" ");
    if (path == "..") {
      activeDir = parentStack.pop();
    } else {
      if (activeDir[path] == undefined) {
        activeDir[path] = {};
      }
      parentStack.push(activeDir);
      activeDir = activeDir[path];
    }
  } else if (cmd.startsWith("ls")) {
    let [_, ...files] = cmd.split("\n");
    for (let file of files) {
      const [size, name] = file.split(" ");
      if (size == "dir") {
        activeDir[name] = {};
      } else {
        activeDir[name] = parseInt(size, 10);
      }
    }
  }
}

const sums = [];
let getSize = (root: Object): number => {
  const size = Object.values(root)
    .map((v) => (typeof v === "object" ? getSize(v) : v))
    .reduce((sum, v) => sum + v);
  sums.push(size);
  return size;
};
const rootSize = getSize(root);

console.log(
  "Part 1:",
  sums.filter((s) => s <= 100000).reduce((sum, s) => sum + s)
);

const remaining = 70000000 - rootSize;
const required = 30000000 - remaining;
sums.sort((a, b) => a - b);
console.log(
  "Part 2:",
  sums.find((s) => s >= required)
);


