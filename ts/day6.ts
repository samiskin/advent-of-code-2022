const inputs = [
  `mjqjpqmgbljsphdztnvjfqwrcgsmlb`,
  `bvwbjplbgvbhsrlpgdmjqwftvncz`,
  `nppdvjthqldpwncqszvftbrmjlhg`,
  `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`,
  `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`,
]

let checkUnique = (str: string, end: number, n: number): boolean => {
  if (n > end) return false;

  let set = new Set()
  for (let j = 0; j < n; j++) {
    if (set.has(str[end - j])) {
      return false
    }
    set.add(str[end - j])
  }
  return true
}

inputs.forEach((input) => {
  for (let i = 3; i < input.length; i++) {
    if (checkUnique(input, i, 4)) {
      console.log("Part 1:", i + 1)
      return
    }
  }
})

inputs.forEach((input) => {
  for (let i = 14; i < input.length; i++) {
    if (checkUnique(input, i, 14)) {
      console.log("Part 2:", i + 1)
      return
    }
  }
})
