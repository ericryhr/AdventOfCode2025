use itertools::Itertools;
use std::cmp::{max, min};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<(i64, i64)> = input
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut result = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let area = ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1);
            result = max(result, area);
        }
    }

    return Some(result as u64);
}

pub fn is_valid_rectangle(p1: (i64, i64), p2: (i64, i64), points: &Vec<(i64, i64)>) -> bool {
    let high_x = max(p1.0, p2.0);
    let low_x = min(p1.0, p2.0);

    let high_y = max(p1.1, p2.1);
    let low_y = min(p1.1, p2.1);

    // Check that no other point is inside (except in border)
    for k in 0..points.len() {
        let p3 = points[k];
        if p3.0 > low_x && p3.0 < high_x && p3.1 > low_y && p3.1 < high_y {
            println!(
                "Point {}-{}, inside {}-{} and {}-{}",
                p3.0, p3.1, p1.0, p1.1, p2.0, p2.1
            );
            return false;
        }
    }

    return true;
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<(i64, i64)> = input
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut result = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            if !is_valid_rectangle(p1, p2, &points) {
                continue;
            }

            let area = ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1);
            println!(
                "p1: {}, {}; p2: {}, {}; area: {}",
                p1.0, p1.1, p2.0, p2.1, area
            );
            result = max(result, area);
        }
    }

    return Some(result as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
