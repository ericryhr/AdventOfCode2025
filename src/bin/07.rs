use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut chars: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut result = 0;
    for row in 0..chars.len() {
        for col in 0..chars[row].len() {
            // If above is not ray continue
            if row <= 0 {
                continue;
            }

            if !(chars[row-1][col] == '|' || chars[row-1][col] == 'S') {
                continue;
            }

            if chars[row][col] == '.' {
                chars[row][col] = '|';
            } else if chars[row][col] == '^' {
                result += 1;
                if col > 0 {
                    chars[row][col-1] = '|';
                }
                if col < chars[row].len()-1 {
                    chars[row][col+1] = '|';
                }
            }
        }
    }

    return Some(result);
}

pub fn calculate_quantum_splits(row: usize, col: usize, chars: &Vec<Vec<char>>, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    // Base case
    if row == chars.len() {
        return 1;
    }

    if chars[row][col] == '.' {
        println!("Should never reach here");
        return 0;
    }

    if chars[row][col] == '|' {
        return calculate_quantum_splits(row + 1, col, chars, memo);
    }

    // Has to be a split
    if let Some(&cached) = memo.get(&(row, col)) {
        return cached;
    } else {
        let mut result = 0;
        if col > 0 {
            result += calculate_quantum_splits(row, col - 1, chars, memo);
        }
        if col < chars[row].len() - 1 {
            result += calculate_quantum_splits(row, col + 1, chars, memo);
        }
        memo.insert((row, col), result);
        return result;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut chars: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start = (0, 0);
    for row in 0..chars.len() {
        for col in 0..chars[row].len() {
            if chars[row][col] == 'S' {
                start = (row, col);
                continue;
            }

            // If above is not ray continue
            if row <= 0 {
                continue;
            }

            if !(chars[row-1][col] == '|' || chars[row-1][col] == 'S') {
                continue;
            }

            if chars[row][col] == '.' {
                chars[row][col] = '|';
            } else if chars[row][col] == '^' {
                if col > 0 {
                    chars[row][col-1] = '|';
                }
                if col < chars[row].len()-1 {
                    chars[row][col+1] = '|';
                }
            }
        }
    }

    let result = calculate_quantum_splits(start.0 + 1, start.1, &chars, &mut HashMap::new());
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
