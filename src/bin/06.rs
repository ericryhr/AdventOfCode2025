advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i == lines.len() - 1 {
            // Parse operations
            for ch in line.chars() {
                if !ch.is_whitespace() {
                    operations.push(ch);
                }
            }
        } else {
            // Parse numbers
            let nums: Vec<u64> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            numbers.push(nums);
        }
    }

    let mut result = 0;

    for (i, operation) in operations.iter().enumerate() {
        // Apply operation to each column
        let mut col_result = 0;
        if operation == &'*' {
            col_result = 1;
        }
        for col in 0..numbers.len() {
            match operation {
                '+' => col_result += numbers[col][i],
                '*' => col_result *= numbers[col][i],
                _ => (),
            };
        }
        result += col_result;
    }

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let chars: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Find indices of whitespace columns
    let mut separators = Vec::new();
    for col in 0..chars[0].len() {
        let mut is_separator = true;
        for row in 0..chars.len() {
            if !chars[row][col].is_whitespace() {
                is_separator = false;
                break;
            }
        }
        if is_separator {
            separators.push(col);
        }
    }

    let mut result: u64 = 0;
    let mut operation = '?';
    let mut current_result: u64 = 0;
    for col in 0..chars[0].len() {
        if separators.contains(&col) {
            operation = '?';
            result += current_result;
            current_result = 0;
            continue;
        }

        if operation == '?' {
            operation = chars[chars.len() - 1][col];
            if operation == '*' {
                current_result = 1;
            }
        }

        // Build number and add it
        let mut current_number: u64 = 0;
        let mut exponent = chars.len() - 2;
        for row in 0..chars.len() - 1 {
            if !chars[row][col].is_whitespace() {
                current_number +=
                    10_u64.pow(exponent as u32) * chars[row][col].to_digit(10).unwrap() as u64;
            }

            if exponent != 0 {
                exponent -= 1;
            }

            if row > 0 && chars[row][col].is_whitespace() {
                current_number /= 10;
            }
        }

        match operation {
            '+' => current_result += current_number,
            '*' => current_result *= current_number,
            _ => (),
        };
        // println!("current_result: {}", current_result);
        // println!("current_number: {}", current_number);
        // println!("operation: {}", operation);
    }
    result += current_result;

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
