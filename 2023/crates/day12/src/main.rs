#![feature(iter_intersperse)]
use std::{
    collections::HashMap,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn solve_inner(out: &mut Vec<usize>, springs: &mut [u8], group_size: u64) {
    if springs.is_empty() {
        return;
    }

    let mut i = 0;
    while springs[i] == b'.' {
        i += 1;
        if i == springs.len() {
            return;
        }
    }

    let mut brokens = 0;
    while springs[i] == b'#' {
        if brokens > group_size {
            return;
        }

        i += 1;
        brokens += 1;

        if i == springs.len() {
            if brokens == group_size {
                out.push(i);
                return;
            }
            return;
        }
    }

    if brokens > group_size {
        return;
    }

    if springs[i] == b'.' {
        if brokens == group_size {
            out.push(i);
            return;
        }

        return;
    }

    assert_eq!(springs[i] as char, b'?' as char);

    if brokens == group_size {
        springs[i] = b'.';
        solve_inner(out, springs, group_size);
        springs[i] = b'?';

        return;
    }

    assert!(brokens < group_size);

    springs[i] = b'#';
    solve_inner(out, springs, group_size);
    springs[i] = b'.';
    solve_inner(out, springs, group_size);
    springs[i] = b'?';
}

fn solve_real(
    springs: &mut [u8],
    groups: &[u64],
    cache: &mut HashMap<(Vec<u8>, Vec<u64>), u64>,
) -> u64 {
    if let Some(&result) = cache.get(&(springs.to_owned(), groups.to_owned())) {
        return result;
    }

    if groups.is_empty() {
        if springs.contains(&b'#') {
            return 0;
        }
        return 1;
    }

    let mut out = Vec::new();
    solve_inner(&mut out, springs, groups[0]);
    out.sort();
    let mut counts: HashMap<usize, u64> = HashMap::new();
    for &i in &out {
        *counts.entry(i).or_default() += 1;
    }

    let mut total = 0;

    for (i, count) in counts {
        if i == springs.len() {
            total += count * solve_real(&mut springs[i..], &groups[1..], cache);
        } else {
            let tmp = springs[i];
            assert_ne!(tmp, b'#');
            springs[i] = b'.';
            total += count * solve_real(&mut springs[i..], &groups[1..], cache);
            springs[i] = tmp;
        }
    }

    cache.insert((springs.to_owned(), groups.to_owned()), total);
    total
}

fn solve(springs: &str, groups: &str) -> u64 {
    let mut springs = springs.as_bytes().to_owned();
    let groups = groups
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    solve_real(&mut springs, &groups, &mut HashMap::new())
}

fn solve_p1(input: &str) -> u64 {
    let mut total_count = 0;
    for line in input.lines() {
        let (springs, groups) = line.split_once(" ").unwrap();
        total_count += solve(springs, groups);
    }
    total_count
}

fn solve_p2(input: &str) -> u64 {
    let mut total_count = 0;
    for line in input.lines() {
        let (springs, groups) = line.split_once(" ").unwrap();
        let springs: String = std::iter::once(springs)
            .cycle()
            .take(5)
            .intersperse("?")
            .collect();
        let groups: String = std::iter::once(groups)
            .cycle()
            .take(5)
            .intersperse(",")
            .collect();
        total_count += solve(&springs, &groups);
    }
    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_p1("?? 1"), 2);
        assert_eq!(solve_p1("### 3"), 1);
        assert_eq!(solve_p1(".### 3"), 1);
        assert_eq!(solve_p1("#.### 1,3"), 1);
        assert_eq!(solve_p1("?.### 1,3"), 1);
        assert_eq!(solve_p1("??.### 1,3"), 2);
        assert_eq!(solve_p1("??.### 2,3"), 1);
        assert_eq!(solve_p1("??? 1,1"), 1);
        assert_eq!(solve_p1("???.### 1,1,3"), 1);
        assert_eq!(solve_p1("?###???????? 3,2,1"), 10);
        assert_eq!(solve_p1("????.#...#... 4,1,1"), 1);
        assert_eq!(solve_p1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(solve_p1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(solve_p1(".??????? 4"), 4);
        assert_eq!(solve_p1(".???????. 4"), 4);
        assert_eq!(solve_p1("??????? 4"), 4);
        assert_eq!(solve_p1("#?? 1"), 1);
        assert_eq!(solve_p1(".#?? 1"), 1);
        assert_eq!(solve_p1(".??#?? 1,1,1"), 1);
        assert_eq!(solve_p1(".??#?? 1"), 1);
        assert_eq!(solve_p1("????#?? 1,1"), 4);
        assert_eq!(solve_p1(".???????#?? 4,1"), 4);
        assert_eq!(solve_p1("????.######..#####. 1,6,5"), 4);
        assert_eq!(solve_p1("?###???????? 3,2,1",), 10);

        assert_eq!(
            solve_p1(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            21
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            ),
            525152
        );
    }
}
