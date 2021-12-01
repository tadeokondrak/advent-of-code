fn main() {
    aoc_driver::aoc_complete! {
        session_file: ".session"
        input_dir: "input"
        challenges: [
            {
                "2021-1-1": day1part1,
                tests: []
            }
            {
                "2021-1-2": day1part2,
                tests: []
            }
        ]
    }
}

fn day1part1(input: &str) -> i32 {
    let mut n = 0;
    let mut last_val = None;
    for val in input.lines().map(|x| x.parse::<i32>().unwrap()) {
        if let Some(last_val) = last_val {
            if val > last_val {
                n += 1;
            }
        }
        last_val = Some(val);
    }
    n
}

fn day1part2(input: &str) -> i32 {
    let mut last_sum = None;
    let mut n = 0;
    let vals: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    for arr in vals.windows(3) {
        let sum = arr[0] + arr[1] + arr[2];
        if let Some(last_sum) = last_sum {
            if sum > last_sum {
                n += 1;
            }
        }
        last_sum = Some(sum);
    }
    n
}
