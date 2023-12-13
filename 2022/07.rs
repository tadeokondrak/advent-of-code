use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn process(input: &str) -> HashMap<String, u64> {
    let mut path = Vec::new();
    let mut sizes = HashMap::new();
    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        if line.starts_with("$ ") {
            let args: Vec<&str> = line[2..].split(" ").collect();
            match (args[0], &args[1..]) {
                ("cd", ["/"]) => path.clear(),
                ("cd", [".."]) => _ = path.pop(),
                ("cd", [dir]) => _ = path.push(dir.to_owned()),
                ("ls", []) => {}
                _ => panic!("unknown: {:#?}", args),
            }
        } else {
            let (lhs, _) = sscanf::sscanf!(line, "{String} {String}").unwrap();
            if lhs != "dir" {
                for i in 0..=path.len() {
                    let cur_path = path[..i].join("/");
                    let size = lhs.parse::<u64>().unwrap();
                    *sizes.entry(cur_path).or_default() += size;
                }
            }
        }
    }
    sizes
}

fn part1(input: &str) -> u64 {
    process(input)
        .values()
        .copied()
        .filter(|&size| size <= 100000)
        .sum()
}

fn part2(input: &str) -> u64 {
    let sizes = process(input);
    sizes
        .values()
        .copied()
        .filter(|&size| sizes[""] - size <= 40000000)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "
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
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 24933642);
    }
}
