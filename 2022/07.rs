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
    let mut sizes: HashMap<String, u64> = HashMap::new();
    let mut last_cmd = None;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("$ ") {
            let args = line[2..].split(" ").collect::<Vec<_>>();
            last_cmd = None;
            match (args[0].clone(), args.get(1)) {
                ("cd", Some(&"/")) => path.clear(),
                ("cd", Some(&"..")) => _ = path.pop(),
                ("cd", Some(&new_path)) => _ = path.push(new_path.to_owned()),
                ("ls", None) => last_cmd = Some("ls"),
                _ => panic!("unknown: {:#?}", args),
            }
        } else {
            match last_cmd {
                Some("ls") => {
                    let (lhs, name) = sscanf::sscanf!(line, "{String} {String}").unwrap();
                    if lhs != "dir" {
                        path.push(name);
                        for i in 0..path.len() {
                            let cur_path = path[..i].join("/");
                            *sizes.entry(cur_path).or_default() += lhs.parse::<u64>().unwrap();
                        }
                        path.pop().unwrap();
                    }
                }
                _ => panic!("unknown: {:#?} {:#?}", line, last_cmd),
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
