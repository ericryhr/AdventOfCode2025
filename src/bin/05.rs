advent_of_code::solution!(5);

use std::cmp::{max, min};

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let ranges = lines
        .iter()
        .filter(|&line| line.contains("-"))
        .map(|line| {
            line.split('-')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let ingredients = lines
        .iter()
        .filter(|&line| !line.contains("-") && !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut result = 0;
    for ingredient in ingredients {
        for range in &ranges {
            if ingredient >= range[0] && ingredient <= range[1] {
                result += 1;
                break;
            }
        }
    }

    return Some(result);
}

pub fn compress_ranges(ranges: &mut Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    loop {
        let mut merged_once = false;
        let mut new_ranges: Vec<Vec<u64>> = Vec::new();

        for range in ranges.iter() {
            let mut merged = false;
            for new_range in &mut new_ranges {
                if !(range[1] < new_range[0] || range[0] > new_range[1]) {
                    new_range[0] = min(new_range[0], range[0]);
                    new_range[1] = max(new_range[1], range[1]);
                    merged = true;
                    merged_once = true;
                    break;
                }
            }
            if !merged {
                new_ranges.push(range.clone());
            }
        }

        if !merged_once {
            return new_ranges;
        }

        *ranges = new_ranges;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut original_ranges = lines
        .iter()
        .filter(|&line| line.contains("-"))
        .map(|line| {
            line.split('-')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let ranges = compress_ranges(&mut original_ranges);

    let mut result = 0;
    for range in &ranges {
        result += range[1] - range[0] + 1;
    }

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
