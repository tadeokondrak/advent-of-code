#![feature(test)]
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn make_table() -> [u32; 64] {
    let mut t = [0u32; 64];
    let mut i = 1;
    loop {
        if i > 64 {
            break;
        }
        t[i - 1] = f64::floor(4294967296.0f64 * f64::abs(trig_const::sin(i as f64))) as u32;
        i += 1;
    }
    t
}

fn md5(table: &[u32; 64], data: &[u8]) -> [u8; 16] {
    let data_len = data.len();
    let mut data = data.to_vec();
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0x00);
    }
    data.extend(u64::to_le_bytes(data_len as u64 * 8));
    assert_eq!(data.len() % 64, 0);

    let mut a: u32 = u32::from_le_bytes([0x01, 0x23, 0x45, 0x67]);
    let mut b: u32 = u32::from_le_bytes([0x89, 0xab, 0xcd, 0xef]);
    let mut c: u32 = u32::from_le_bytes([0xfe, 0xdc, 0xba, 0x98]);
    let mut d: u32 = u32::from_le_bytes([0x76, 0x54, 0x32, 0x10]);

    for i in 0..data.len() / 64 {
        let chunk = &data[i * 64..][..64];

        let mut x = [0u32; 16];
        for j in 0..16 {
            x[j] = u32::from_le_bytes(chunk[j * 4..][..4].try_into().unwrap());
        }

        let aa = a;
        let bb = b;
        let cc = c;
        let dd = d;

        fn r(
            f: impl Fn(u32, u32, u32) -> u32,
            a: &mut u32,
            b: u32,
            c: u32,
            d: u32,
            x_k: u32,
            s: u32,
            t_i: u32,
        ) {
            *a = a.wrapping_add(f(b, c, d));
            *a = a.wrapping_add(x_k);
            *a = a.wrapping_add(t_i);
            *a = a.rotate_left(s);
            *a = a.wrapping_add(b);
        }

        const S: [u32; 64] = [
            7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20,
            5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
            6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
        ];

        for (i, xi) in (0..16).map(|i| (i, i)) {
            fn f(x: u32, y: u32, z: u32) -> u32 {
                (x & y) | (!x & z)
            }
            r(f, &mut a, b, c, d, x[xi], S[i], table[i]);
            [a, b, c, d] = [d, a, b, c];
        }

        for (i, xi) in (16..32).map(|i| (i, (5 * i + 1) % 16)) {
            fn f(x: u32, y: u32, z: u32) -> u32 {
                (x & z) | (y & !z)
            }
            r(f, &mut a, b, c, d, x[xi], S[i], table[i]);
            [a, b, c, d] = [d, a, b, c];
        }

        for (i, xi) in (32..48).map(|i| (i, (3 * i + 5) % 16)) {
            fn f(x: u32, y: u32, z: u32) -> u32 {
                x ^ y ^ z
            }
            r(f, &mut a, b, c, d, x[xi], S[i], table[i]);
            [a, b, c, d] = [d, a, b, c];
        }

        for (i, xi) in (48..64).map(|i| (i, (7 * i) % 16)) {
            fn f(x: u32, y: u32, z: u32) -> u32 {
                y ^ (x | !z)
            }
            r(f, &mut a, b, c, d, x[xi], S[i], table[i]);
            [a, b, c, d] = [d, a, b, c];
        }

        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    let mut result_bytes = [0u8; 16];
    result_bytes[0..][..4].copy_from_slice(&u32::to_le_bytes(a));
    result_bytes[4..][..4].copy_from_slice(&u32::to_le_bytes(b));
    result_bytes[8..][..4].copy_from_slice(&u32::to_le_bytes(c));
    result_bytes[12..][..4].copy_from_slice(&u32::to_le_bytes(d));
    result_bytes
}

fn solve(input: &str, n: usize) -> i64 {
    let table = make_table();
    let mut v = Vec::from(input);
    for i in 1.. {
        v.truncate(input.len());
        v.extend(i.to_string().as_bytes());
        let hash = format!("{:032x}", u128::from_be_bytes(md5(&table, &v)));
        if hash.chars().take_while(|&c| c == '0').count() >= n {
            return i;
        }
    }
    unreachable!()
}

fn part1(input: &str) -> i64 {
    solve(input, 5)
}

fn part2(input: &str) -> i64 {
    solve(input, 6)
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn test_md5() {
        let table = make_table();
        assert_eq!(
            md5(&table, "".as_bytes()),
            0xd41d8cd98f00b204e9800998ecf8427eu128.to_be_bytes()
        );
        assert_eq!(
            md5(&table, "a".as_bytes()),
            0x0cc175b9c0f1b6a831c399e269772661u128.to_be_bytes()
        );
        assert_eq!(
            md5(&table, "abc".as_bytes()),
            0x900150983cd24fb0d6963f7d28e17f72u128.to_be_bytes()
        );
        assert_eq!(
            md5(&table, "message digest".as_bytes()),
            0xf96b697d7cb7938d525a2f31aaf161d0u128.to_be_bytes()
        );
        assert_eq!(
            md5(&table, "abcdefghijklmnopqrstuvwxyz".as_bytes()),
            0xc3fcd3d76192e4007dfb496cca67e13bu128.to_be_bytes()
        );
        assert_eq!(
            md5(
                &table,
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes()
            ),
            0xd174ab98d277d9f5a5611c2c9f419d9fu128.to_be_bytes()
        );
        assert_eq!(
            md5(
                &table,
                "12345678901234567890123456789012345678901234567890123456789012345678901234567890"
                    .as_bytes()
            ),
            0x57edf4a22be3c955ac49da2e2107b67au128.to_be_bytes()
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdef"), 609043);
    }

    #[bench]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part1(test::black_box(input)), 117946));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part2(test::black_box(input)), 3938038));
    }
}
