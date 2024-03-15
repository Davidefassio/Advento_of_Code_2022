use std::fs::read_to_string;

const SNAFU_DIGITS: [char; 5] = ['=', '-', '0', '1', '2'];

fn number_from_snafu(snafu: &str) -> i64 {
    snafu
        .chars()
        .map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unknown digit"),
        })
        .fold(0, |sum, d| sum * 5 + d)
}

fn count_snafu_digits(n: i64, base: i64, max: i64) -> usize {
    let mut digit_count = 1;
    let mut upper_limit = 2;
    while n > upper_limit {
        digit_count += 1;
        upper_limit = upper_limit * base + max;
    }

    digit_count
}

fn compute_envelope(digit: i64, base: i64, count: usize) -> i64 {
    (0..count).fold(0, |sum, _| sum * base + digit)
}

fn snafu_from_number(n: i64) -> String {
    let num_digits = count_snafu_digits(n, 5, 2);

    let mut snafu = String::new();

    let mut remaining = n;
    let mut env_min = compute_envelope(-2, 5, num_digits);
    let mut env_max = compute_envelope(2, 5, num_digits);
    let mut digit_value = i64::pow(5, num_digits as u32 - 1);
    for _ in 0..num_digits {
        let digit = (remaining - env_min) / digit_value;
        let snafu_digit = SNAFU_DIGITS[digit as usize];
        snafu.push(snafu_digit);

        remaining -= (digit + -2) * digit_value;
        env_min = (env_min - -2) / 5;
        env_max = (env_max - -2) / 5;
        digit_value /= 5;
    }

    snafu
}

fn main() {
    let mut sum = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let n = number_from_snafu(line);
        sum += n;
    }

    println!("{}", snafu_from_number(sum));
}
