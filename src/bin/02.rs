advent_of_code::solution!(2);

pub fn is_invalid_number_one(number: u64) -> bool {
    let chars = number.to_string().chars().collect::<Vec<char>>();

    if chars.len() % 2 != 0 {
        return false;
    }

    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() / 2 + i] {
            return false;
        }
    }

    return true;
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<Vec<u64>> = input
        .trim()
        .split(',')
        .map(|x| {
            x.split('-')
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let mut result = 0;

    for range in ranges {
        let start: u64 = range[0];
        let end: u64 = range[1];

        for number in start..=end {
            if is_invalid_number_one(number) {
                result += number;
            }
        }
    }

    return Some(result);
}

pub fn is_invalid_number_two(number: u64) -> bool {
    let chars = number.to_string().chars().collect::<Vec<char>>();

    // Find all divisors of num length
    for div in 2..=chars.len() {
        if chars.len() % div != 0 {
            continue;
        }

        let step = chars.len() / div;
        let mut is_div_invalid = true;
        for i in 0..step {
            // Check each consecutive div
            let first_digit = chars[i];
            for j in ((i + step)..chars.len()).step_by(step) {
                if chars[j] != first_digit {
                    is_div_invalid = false;
                    break;
                }
            }
        }

        if is_div_invalid {
            return true;
        }
    }

    return false;
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<Vec<u64>> = input
        .trim()
        .split(',')
        .map(|x| {
            x.split('-')
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let mut result = 0;

    for range in ranges {
        let start: u64 = range[0];
        let end: u64 = range[1];

        for number in start..=end {
            if is_invalid_number_two(number) {
                result += number;
            }
        }
    }

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
