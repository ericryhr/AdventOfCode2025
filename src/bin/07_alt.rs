// PER ALGUN MOTIU AQUESTA IMPLEMENTACIÓ NO FUNCIONA, TOT I QUE SEMBLA LÒGICA

use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn calculate_split(row: usize, col: usize, splitters: &mut HashMap<usize, Vec<(usize, bool)>>) {
    if let Some(col_splitters) = splitters.get_mut(&col) {
        // Find the first splitter below the current row, and mark it as used, then recurse
        if let Some(pos) = col_splitters.iter().position(|s| s.0 > row && !s.1) {
            println!("");
            println!("Looking at row {}, col {}", row, col);
            println!("Had {:?}", col_splitters);
            println!("Chosen splitter at ({}, {})", col_splitters[pos].0, col);
            println!("");
            let min_row = col_splitters[pos].0;
            col_splitters[pos].1 = true;
            calculate_split(min_row, col - 1, splitters);
            calculate_split(min_row, col + 1, splitters);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let chars: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start: (usize, usize) = (0, 0);
    let mut splitters: HashMap<usize, Vec<(usize, bool)>> = HashMap::new();

    for row in 0..chars.len() {
        for col in 0..chars[row].len() {
            let c = chars[row][col];
            match c {
                'S' => start = (row, col),
                '^' => splitters
                    .entry(col)
                    .or_insert(Vec::new())
                    .push((row, false)),
                _ => (),
            };
        }
    }

    calculate_split(start.0, start.1, &mut splitters);

    let mut result = 0;
    for (_, col_splitters) in splitters {
        for splitter in col_splitters {
            if splitter.1 {
                result += 1;
            }
        }
    }

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
