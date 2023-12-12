#![feature(iter_intersperse)]
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    //eprintln!("p2: {}", solve_p2(&input));
}

fn solve_inner(out: &mut Vec<usize>, springs: &mut [u8], groups: &[u64], group_size: u64) {
    //eprintln!("{}", std::str::from_utf8(&springs).unwrap());

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
            }
            return;
        }
    }

    if springs[i] == b'.' {
        if brokens == group_size {
            out.push(i);
            return;
        }

        return;
    }

    assert_eq!(springs[i] as char, b'?' as char);

    springs[i] = b'.';
    solve_inner(out, springs, groups, group_size);
    if group_size > 0 {
        springs[i] = b'#';
        solve_inner(out, springs, groups, group_size);
    }
    springs[i] = b'?';

    //    let mut brokens = 0;
    //    loop {
    //        if brokens == depth {
    //            eprintln!("{} !!!", std::str::from_utf8(&springs).unwrap());
    //            return 1;
    //        }
    //
    //        if springs[i] == b'.' && brokens != 0 {
    //            eprintln!("{} bad", std::str::from_utf8(&springs).unwrap());
    //            return 0;
    //        }
    //
    //        if springs[i] == b'#' {
    //            brokens += 1;
    //            continue;
    //        }
    //
    //        if springs[i] == b'?' {

    //        }
    //    }
}

fn solve_real(springs: &mut [u8], groups: &[u64]) -> u64 {
    // Find all possible ways to place the first group of broken springs
    // Then for each possible length of those groups,
    //     find all possible ways to place the next group of broken springs starting from that length
    //     (that is, recur)

    if groups.is_empty() {
        return 1;
    }

    if springs.is_empty() {
        return 1;
    }

    //eprintln!("! {} {groups:?}", std::str::from_utf8(&springs).unwrap());

    let mut out = Vec::new();
    let mut sols = Vec::new();
    out.sort();
    let mut count = 0;
    solve_inner(&mut out, springs, &groups, groups[0]);
    eprintln!("out={out:?}");
    for i in 0..out.len() {
        let start_index = out[i];
        let x = solve_real(&mut springs[start_index..], &groups[1..]);
        sols.push(x);
        eprintln!(
            "! {} ::: {} -> {x}",
            std::str::from_utf8(&springs).unwrap(),
            std::str::from_utf8(&springs[start_index..]).unwrap()
        );
        count += x;
    }
    dbg!(sols);
    eprintln!(
        "! {} {groups:?} -> {count}",
        std::str::from_utf8(&springs).unwrap()
    );
    count
}

fn solve(springs: &str, groups: &str) -> u64 {
    let mut springs = springs.as_bytes().to_owned();
    let groups = groups
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    solve_real(&mut springs, &groups)
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
        assert_eq!(solve_p1("???.### 1,1,3"), 1);
        assert_eq!(solve_p1("?###???????? 3,2,1"), 10);
        assert_eq!(solve_p1("????.#...#... 4,1,1"), 1);
        assert_eq!(solve_p1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(solve_p1(".??..??...?##. 1,1,3"), 4);

        assert_eq!(solve_p1("????.######..#####. 1,6,5"), 4);
        assert_eq!(solve_p1("?###???????? 3,2,1",), 0);

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
/*

//fn go(springs: &mut Vec<u8>, groups: &mut Vec<i32>) -> i32 {
//    eprintln!("{springs:?} {groups:?}");
//    let mut group_index = 0;
//    let mut group_count = 0;
//    let mut spring_index = 0;
//    loop {
//        if spring_index >= springs.len() {
//            break 0;
//        }
//
//        eprintln!("{:?}", springs[spring_index] as char);
//        eprintln!("group {group_index:?} index {group_count:?}");
//        match springs[spring_index] {
//            b'?' => {
//                spring_index += 1;
//            }
//            b'.' => {
//                spring_index += 1;
//            }
//            b'#' => {
//                group_count += 1;
//                spring_index += 1;
//
//                if groups[group_index] == group_count {
//                    group_index += 1;
//                    group_count = 0;
//                }
//            }
//            _ => todo!(),
//        }
//    }
//}

fn go(springs: &mut [u8], groups: &[i32]) -> i32 {
    eprintln!("{}", std::str::from_utf8(&springs).unwrap());
    match rfind_groups2(springs) {
        Ok(got_groups) => {
            //eprintln!("{got_groups:?}, {groups:?}!");
            if groups == got_groups {
                return 1;
            } else {
                return 0;
            }
        }
        Err(got_groups) => {
            //eprintln!("{got_groups:?}, {groups:?}");
            let unknown = springs.iter().copied().rposition(|c| c == b'?').unwrap();

            for (x, y) in groups.iter().zip(got_groups.iter()) {
                if x != y {
                    return 0;
                }
            }

            let mut count = 0;

            springs[unknown] = b'#';
            count += go(springs, groups);

            springs[unknown] = b'.';
            count += go(springs, groups);

            springs[unknown] = b'?';

            count
        }
    }
}

fn solve_p1(input: &str) -> i32 {
    let mut total_count = 0;
    for line in input.lines() {
        let (springs, groups) = line.split_once(" ").unwrap();
        let mut want_groups = groups
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut springs = springs.as_bytes().to_owned();
        want_groups.reverse();
        total_count += go(&mut springs, &want_groups);
    }
    total_count
}

fn solve_p2(input: &str) -> i32 {
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

        dbg!(&springs, &groups);
        let mut want_groups = groups
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut springs = springs.as_bytes().to_owned();

        want_groups.reverse();
        total_count += go(&mut springs, &want_groups);
    }
    total_count
}

fn find_groups(springs: &[u8], i: i32) -> Vec<i32> {
    eprintln!("{i:03b}");
    let mut springs_copy = springs.to_owned();
    let mut unknown_index = 0;
    for spring_index in 0..springs_copy.len() {
        if springs_copy[spring_index] == b'?' {
            let c = if i & (1 << unknown_index) != 0 {
                b'#'
            } else {
                b'.'
            };
            springs_copy[spring_index] = c;
            unknown_index += 1;
        }
    }

    let mut groups = vec![0];
    for spring_index in 0..springs_copy.len() {
        if springs_copy[spring_index] == b'.' {
            if *groups.last().unwrap() != 0 {
                groups.push(0);
            }
        } else {
            *groups.last_mut().unwrap() += 1;
        }
    }

    if *groups.last().unwrap() == 0 {
        groups.pop();
    }

    groups
}

fn find_groups2(springs: &[u8]) -> Result<Vec<i32>, (Vec<i32>, usize)> {
    let mut groups = vec![0];
    for spring_index in 0..springs.len() {
        match springs[spring_index] {
            b'.' => {
                if *groups.last().unwrap() != 0 {
                    groups.push(0);
                }
            }
            b'#' => {
                *groups.last_mut().unwrap() += 1;
            }
            b'?' => {
                if *groups.last().unwrap() == 0 {
                    groups.pop();
                }
                groups.pop();

                return Err((groups, spring_index));
            }
            _ => panic!(),
        }
    }

    if *groups.last().unwrap() == 0 {
        groups.pop();
    }

    Ok(groups)
}

fn rfind_groups2(springs: &[u8]) -> Result<Vec<i32>, Vec<i32>> {
    let mut groups = vec![0];
    for spring_index in (0..springs.len()).rev() {
        match springs[spring_index] {
            b'.' => {
                if *groups.last().unwrap() != 0 {
                    groups.push(0);
                }
            }
            b'#' => {
                *groups.last_mut().unwrap() += 1;
            }
            b'?' => {
                if *groups.last().unwrap() == 0 {
                    groups.pop();
                }
                groups.pop();

                return Err(groups);
            }
            _ => panic!(),
        }
    }

    if *groups.last().unwrap() == 0 {
        groups.pop();
    }

    Ok(groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
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
 */
