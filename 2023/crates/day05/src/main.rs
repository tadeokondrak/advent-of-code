use std::{
    cmp::min,
    collections::HashMap,
    io::{stdin, Read},
    mem::swap,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

#[derive(Debug, Clone, Copy)]
struct Range1 {
    start: u64,
    len: u64,
}

#[derive(Debug, Clone, Copy)]
struct Range2 {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

const CHAIN: [&str; 8] = [
    "seed",
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];

fn parse(input: &str) -> (Vec<u64>, HashMap<(&str, &str), Vec<Range2>>) {
    let (_, rest) = input.split_once("seeds: ").unwrap();
    let (seeds, rest) = rest.split_once("\n").unwrap();
    let seeds = seeds
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut maps = HashMap::new();
    for map in rest.trim().split("\n\n") {
        let (name, map) = map.split_once(" map:\n").unwrap();
        let (from, to) = name.split_once("-to-").unwrap();

        let mut ranges = Vec::new();

        for line in map.lines() {
            let mut nums = line.split(" ");
            let dst_start = nums.next().unwrap().parse::<u64>().unwrap();
            let src_start = nums.next().unwrap().parse::<u64>().unwrap();
            let len = nums.next().unwrap().parse::<u64>().unwrap();
            ranges.push(Range2 {
                src_start,
                dst_start,
                len,
            });
        }
        ranges.sort_by_key(|x| x.src_start);
        maps.insert((from, to), ranges);
    }
    (seeds, maps)
}

fn solve_p1(input: &str) -> u64 {
    let (seeds, maps) = parse(input);

    let mut cur = Vec::new();

    for a in seeds.chunks(2) {
        let start = a[0];
        let len = a[1];
        cur.push(Range1 { start, len });
    }

    solve(
        maps,
        seeds
            .iter()
            .copied()
            .map(|seed| Range1 {
                start: seed,
                len: 1,
            })
            .collect(),
    )
}

fn solve_p2(input: &str) -> u64 {
    let (seed_ranges, maps) = parse(input);

    let mut seeds = Vec::new();

    for a in seed_ranges.chunks(2) {
        let start = a[0];
        let len = a[1];
        seeds.push(Range1 { start, len });
    }

    solve(maps, seeds)
}

fn solve(maps: HashMap<(&str, &str), Vec<Range2>>, mut seeds: Vec<Range1>) -> u64 {
    let mut next = Vec::new();

    for step in CHAIN.windows(2) {
        let map = &maps[&(step[0], step[1])];

        next.clear();

        for range in seeds.iter_mut() {
            'outer: while range.len > 0 {
                for item in map {
                    if (item.src_start..item.src_start + item.len).contains(&range.start) {
                        let end_of_item_range = item.src_start + item.len;
                        let end_of_our_range = range.start + range.len;
                        let applicable_len = min(end_of_item_range, end_of_our_range) - range.start;
                        assert_ne!(applicable_len, 0);
                        next.push(Range1 {
                            start: range.start - item.src_start + item.dst_start,
                            len: applicable_len,
                        });
                        range.start += applicable_len;
                        range.len -= applicable_len;
                        continue 'outer;
                    }
                }

                let next_available_range = map
                    .iter()
                    .filter(|item| item.src_start >= range.start)
                    .next();

                match next_available_range {
                    Some(item) => {
                        let applicable_len = min(item.src_start - range.start, range.len);
                        next.push(Range1 {
                            start: range.start,
                            len: applicable_len,
                        });
                        range.start += applicable_len;
                        range.len -= applicable_len;
                        continue 'outer;
                    }
                    None => {
                        next.push(*range);
                        break 'outer;
                    }
                }
            }
        }

        swap(&mut seeds, &mut next);
    }

    seeds.iter().copied().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    use super::*;
    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 35);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 46);
    }
}
