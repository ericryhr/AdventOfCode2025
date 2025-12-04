advent_of_code::solution!(4);

pub fn check_roll(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut count = 0;

    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0
                && ni < grid.len() as isize
                && nj >= 0
                && nj < grid[0].len() as isize
                && grid[ni as usize][nj as usize] == '@'
            {
                count += 1;
            }
        }
    }

    return count < 4;
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '@' {
                continue;
            }

            if check_roll(&grid, i, j) {
                result += 1;
            }
        }
    }
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut result = 0;
    loop {
        let mut removed_one = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != '@' {
                    continue;
                }

                if check_roll(&grid, i, j) {
                    grid[i][j] = 'x';
                    removed_one = true;
                    result += 1;
                }
            }
        }
        if !removed_one {
            break;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
