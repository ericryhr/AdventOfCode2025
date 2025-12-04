advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut current_position: i64 = 50;
    let mut result = 0;

    for line in lines {
        let instruction = line.chars().next().unwrap();
        let value: i64 = line[1..].parse().unwrap();

        match instruction {
            'L' => {
                // Handle left turn
                current_position = (current_position - value + 100) % 100;
            }
            'R' => {
                // Handle right turn
                current_position = (current_position + value + 100) % 100;
            }
            _ => {}
        }

        // println!("Current position: {}", current_position);

        if current_position == 0 {
            result += 1;
        }
    }

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut current_position: i64 = 50;
    let mut new_position: i64 = 0;
    let mut result = 0;

    for line in lines {
        let instruction = line.chars().next().unwrap();
        let value: i64 = line[1..].parse().unwrap();

        match instruction {
            'L' => {
                // Handle left turn
                new_position = current_position - value;
            }
            'R' => {
                // Handle right turn
                new_position = current_position + value;
            }
            _ => {}
        }

        // Count number of 0 crossings
        let zero_crossings = (new_position / 100 - current_position / 100).abs();
        result += zero_crossings as u64;
        // Add 1 for landing exactly on 0
        if new_position % 100 == 0 && zero_crossings == 0 {
            result += 1;
        }
        // Add 1 for crossing from negative to positive or vice versa
        if (current_position < 0 && new_position > 0) || (current_position > 0 && new_position < 0) {
            result += 1;
        }

        current_position = ((new_position % 100) + 100) % 100;
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
        assert_eq!(result, Some(17));
    }
}
