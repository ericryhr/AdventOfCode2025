advent_of_code::solution!(3);

use std::cmp::max;

pub fn get_max_joltage_one(battery: &str) -> u64 {
    let mut first_largest: i64 = battery.chars().nth(0).unwrap().to_digit(10).unwrap() as i64;
    let mut second_largest: i64 = -1;
    for i in 1..battery.len() {
        let digit: i64 = battery.chars().nth(i).unwrap().to_digit(10).unwrap() as i64;

        if digit > first_largest && i < battery.len() - 1 {
            first_largest = digit;
            second_largest = -1;
        } else {
            second_largest = max(second_largest, digit);
        }
    }

    return (first_largest * 10 + second_largest) as u64;
}

pub fn part_one(input: &str) -> Option<u64> {
    let batteries: Vec<&str> = input.trim().lines().collect();

    let mut result = 0;
    for battery in batteries {
        let joltage = get_max_joltage_one(battery);
        result += joltage;
    }

    return Some(result);
}

pub fn get_max_joltage_two(battery: &str) -> u64 {
    let mut joltage: Vec<i64> = vec![-1; 12];
    joltage[0] = battery.chars().nth(0).unwrap().to_digit(10).unwrap() as i64;

    let mut current_index = 1;
    for i in 1..battery.len() {
        let digit: i64 = battery.chars().nth(i).unwrap().to_digit(10).unwrap() as i64;
        let mut found_better = false;
        for j in 0..current_index {
            if digit > joltage[j] && i < battery.len() - (12 - j - 1) {
                joltage[j] = digit;
                current_index = j + 1;
                found_better = true;
                break;
            }
        }

        if !found_better && current_index < joltage.len() {
            joltage[current_index] = digit;
            current_index += 1;
        }
    }

    // Build joltage
    let mut result: u64 = 0;
    for (i, num) in joltage.iter().enumerate() {
        let power = 10_u64.pow((joltage.len() - 1 - i) as u32);
        result += power * (*num as u64);
    }
    return result;
}

pub fn part_two(input: &str) -> Option<u64> {
    let batteries: Vec<&str> = input.trim().lines().collect();

    let mut result = 0;
    for battery in batteries {
        let joltage = get_max_joltage_two(battery);
        result += joltage;
    }

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
