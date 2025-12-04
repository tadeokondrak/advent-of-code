#![allow(dead_code)] // probably unfinished?
#![feature(test, linked_list_cursors)]
use index_list::{IndexList, ListIndex};
use std::{
    io::{self, Read},
    iter::Map,
    str::Chars,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", dumbpart1(&input));
    println!("Part 2: {}", dumbpart2(&input));
}

#[derive(Debug, Clone)]
struct Iter<'a> {
    backing: Map<Chars<'a>, fn(char) -> u64>,
    fwd_state: Option<(u64, u64)>,
    fwd_next_id: u64,
    back_state: Option<(u64, u64)>,
    back_next_id: u64,
}

fn to_digit_unwrap(c: char) -> u64 {
    c.to_digit(10).unwrap().into()
}

impl Iterator for Iter<'_> {
    type Item = Option<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            break match self.fwd_state.as_mut() {
                Some((_, 0)) | None => {
                    let Some(count) = self.backing.next() else {
                        if let Some((digit, count)) = &mut self.back_state {
                            if *count > 0 {
                                *count -= 1;
                                return Some(Some(*digit));
                            }
                        }
                        return None;
                    };
                    let id = self.fwd_next_id;
                    if id % 2 == 0 {
                        assert_ne!(count, 0);
                    }
                    self.fwd_next_id += 1;
                    if count > 0 {
                        self.fwd_state = Some((id, count - 1));
                    } else {
                        self.fwd_state = None;
                        continue;
                    }
                    Some(if id % 2 == 0 { Some(id / 2) } else { None })
                }
                Some(&mut (id, ref mut count)) => {
                    *count -= 1;
                    Some(if id % 2 == 0 { Some(id / 2) } else { None })
                }
            };
        }
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        //eprintln!("-> {:?}", self);
        let result = loop {
            break match self.back_state.as_mut() {
                Some((_, 0)) | None => {
                    if self.back_state.is_some() {
                        _ = self.backing.next_back()?;
                    }
                    let count = self.backing.next_back()?;
                    assert_ne!(count, 0);
                    let id = self.back_next_id;
                    self.back_next_id -= 1;
                    if count == 0 {
                        self.back_state = Some((id, 0));
                        continue;
                    }
                    self.back_state = Some((id, count - 1));
                    Some(Some(id))
                }
                Some(&mut (id, ref mut count)) => {
                    if *count == 0 {
                        self.back_state = None;
                        return None;
                    }
                    *count -= 1;
                    Some(Some(id))
                }
            };
        };
        //eprintln!("-- {:?}", result);
        //eprintln!("<- {:?}", self);
        result
    }
}

fn dumbpart1(input: &str) -> u64 {
    let tape = dumbpart1tape(input);
    tape.iter()
        .enumerate()
        .filter_map(|(index, value)| {
            if let Some(value) = value {
                Some(index as u64 * value)
            } else {
                None
            }
        })
        .sum()
}

fn dumbpart2(input: &str) -> u64 {
    let tape = dumbpart2tape(input);

    let mut i = 0;
    let mut checksum = 0;
    for entry in &tape {
        match entry {
            &Entry::File { id, size } => {
                for _ in 0..size {
                    checksum += i * id;
                    i += 1;
                }
            }
            &Entry::Hole { size } => {
                for _ in 0..size {
                    i += 1;
                }
            }
        }
    }
    checksum
}

fn dumbpart2tape(input: &str) -> Vec<Entry> {
    let mut digits = input.chars().map::<_, fn(char) -> u64>(to_digit_unwrap);
    let mut tape = Vec::new();
    let mut next_file_id = 0;
    loop {
        let file_id = next_file_id;
        next_file_id += 1;
        let Some(file_size) = digits.next() else {
            break;
        };
        tape.push(Entry::File {
            id: file_id,
            size: file_size,
        });
        let Some(hole_size) = digits.next() else {
            break;
        };
        tape.push(Entry::Hole { size: hole_size });
    }
    for id in (0..next_file_id).rev() {
        //dump(&tape);
        let i = tape
            .iter()
            .copied()
            .position(|el| matches!(el, Entry::File { id: file_id, .. } if file_id == id))
            .unwrap();
        let &Entry::File {
            id: _,
            size: file_size,
        } = &tape[i]
        else {
            unreachable!()
        };

        let Some(j) = tape
            .iter()
            .copied()
            .position(|el| matches!(el, Entry::Hole { size: hole_size } if hole_size >= file_size))
        else {
            continue;
        };
        let &Entry::Hole { size: hole_size } = &tape[j] else {
            unreachable!()
        };
        if j >= i {
            continue;
        }
        tape[j] = tape[i];
        let leftover = hole_size - file_size;
        tape.insert(j + 1, Entry::Hole { size: leftover });
        tape[i + 1] = Entry::Hole { size: file_size };
    }
    tape
}

fn dumbpart1tape(input: &str) -> Vec<Option<u64>> {
    let mut digits = input.chars().map::<_, fn(char) -> u64>(to_digit_unwrap);
    let mut tape = Vec::new();
    let mut next_file_id = 0;
    loop {
        let file_id = next_file_id;
        next_file_id += 1;
        let Some(file_size) = digits.next() else {
            break;
        };
        for _ in 0..file_size {
            tape.push(Some(file_id));
        }
        let Some(hole_size) = digits.next() else {
            break;
        };
        for _ in 0..hole_size {
            tape.push(None);
        }
    }
    let mut i = 0;
    while i < tape.len() - 1 {
        if tape[i].is_some() {
            i += 1;
            continue;
        }
        let filler = tape.pop().unwrap();
        tape[i] = filler;
    }
    tape
}

fn part1(input: &str) -> u64 {
    let mut input = input.trim();
    if input.len() % 2 == 0 {
        input = &input[..input.len() - 1];
    }
    let digits = input.chars().map::<_, fn(char) -> u64>(to_digit_unwrap);
    let mut iter = Iter {
        backing: digits,
        fwd_next_id: 0,
        fwd_state: None,
        back_next_id: input.len() as u64 / 2,
        back_state: None,
    };
    let mut checksum = 0;
    'outer: for i in 0.. {
        let Some(next) = iter.next() else {
            break;
        };
        match next {
            Some(id) => {
                checksum += i * id;
            }
            None => {
                let mut stuck = 0;
                let id = loop {
                    match iter.next_back() {
                        Some(Some(id)) => break id,
                        Some(None) => unreachable!(),
                        None => {
                            stuck += 1;
                            if stuck == 100 {
                                break 'outer;
                            }
                            continue;
                        }
                    }
                };
                checksum += i * id;
            }
        }
    }
    checksum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    File { id: u64, size: u64 },
    Hole { size: u64 },
}

fn part2(input: &str) -> u64 {
    let mut input = input.trim();
    if input.len() % 2 == 0 {
        input = &input[..input.len() - 1];
    }
    let digits = input.chars().map::<_, fn(char) -> u64>(to_digit_unwrap);
    let mut iter = Iter {
        backing: digits,
        fwd_next_id: 0,
        fwd_state: None,
        back_next_id: input.len() as u64 / 2,
        back_state: None,
    };
    let mut checksum = 0;
    'outer: for i in 0.. {
        let Some(next) = iter.next() else {
            break;
        };
        match next {
            Some(id) => {
                checksum += i * id;
            }
            None => {
                let mut stuck = 0;
                let id = loop {
                    match iter.next_back() {
                        Some(Some(id)) => break id,
                        Some(None) => unreachable!(),
                        None => {
                            stuck += 1;
                            if stuck == 100 {
                                break 'outer;
                            }
                            continue;
                        }
                    }
                };
                checksum += i * id;
            }
        }
    }
    checksum
}

fn replace_with_hole(tape: &mut IndexList<Entry>, mut idx: ListIndex) {
    let &Entry::File {
        id: _file_id,
        size: mut file_size,
    } = tape.get(idx).unwrap()
    else {
        unreachable!();
    };
    *tape.get_mut(idx).unwrap() = Entry::Hole { size: file_size };
    let prev = tape.prev_index(idx);
    if let Some(Entry::Hole {
        size: prev_hole_size,
    }) = tape.get_mut(prev)
    {
        *prev_hole_size += file_size;
        file_size = *prev_hole_size;
        idx = prev;
        tape.remove(tape.next_index(prev));
    }
    let next = tape.next_index(idx);
    if let Some(Entry::Hole {
        size: next_hole_size,
    }) = tape.get_mut(next)
    {
        *next_hole_size += file_size;
        tape.remove(tape.prev_index(next));
    }
}

fn dump(l: &Vec<Entry>) {
    for entry in l {
        match entry {
            &Entry::File { id, size } => {
                for _ in 0..size {
                    eprint!("{id}");
                }
            }
            &Entry::Hole { size } => {
                for _ in 0..size {
                    eprint!(".");
                }
            }
        }
    }
    eprintln!();
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(dumbpart2(TEST_INPUT), 2858);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 0));
        // not 6356278839323
        // not 6158323629116
        // not 6158916893734
        // not 6159510158352
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 0));
    }
}
