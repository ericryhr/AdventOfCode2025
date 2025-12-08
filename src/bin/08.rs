use itertools::{sorted, Itertools};

advent_of_code::solution!(8);

pub struct Box {
    x: i64,
    y: i64,
    z: i64,
}

pub fn distance(box0: &Box, box1: &Box) -> f64 {
    return (((box0.x - box1.x) * (box0.x - box1.x)
        + (box0.y - box1.y) * (box0.y - box1.y)
        + (box0.z - box1.z) * (box0.z - box1.z)) as f64)
        .sqrt();
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.trim().lines();
    let boxes: Vec<Box> = lines
        .map(|line| {
            let values: Vec<&str> = line.split(',').collect();
            return Box {
                x: values[0].parse::<i64>().unwrap(),
                y: values[1].parse::<i64>().unwrap(),
                z: values[2].parse::<i64>().unwrap(),
            };
        })
        .collect();

    let mut distances: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..boxes.len() {
        for j in i..boxes.len() {
            if i == j {
                continue;
            }
            distances.push((distance(&boxes[i], &boxes[j]), i, j));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut connected_components: Vec<usize> = (0..boxes.len()).collect();
    const CONNECTIONS_TO_MAKE: usize = 1000;
    for i in 0..CONNECTIONS_TO_MAKE {
        let connection = distances[i];

        // Make connection & update connected components
        let comp_a = connected_components[connection.1];
        let comp_b = connected_components[connection.2];
        for comp in connected_components.iter_mut() {
            if *comp == comp_b {
                *comp = comp_a;
            }
        }
    }

    // Get sizes of 3 largest connected components
    let component_ids = sorted(connected_components.clone()).dedup().collect::<Vec<usize>>();
    let mut component_sizes: Vec<u64> = Vec::new();
    for id in &component_ids {
        let size = connected_components.iter().filter(|&c| c == id).count();
        component_sizes.push(size as u64);
    }
    component_sizes.sort();

    let result = component_sizes[component_sizes.len() - 1]
        * component_sizes[component_sizes.len() - 2]
        * component_sizes[component_sizes.len() - 3];
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.trim().lines();
    let boxes: Vec<Box> = lines
        .map(|line| {
            let values: Vec<&str> = line.split(',').collect();
            return Box {
                x: values[0].parse::<i64>().unwrap(),
                y: values[1].parse::<i64>().unwrap(),
                z: values[2].parse::<i64>().unwrap(),
            };
        })
        .collect();

    let mut distances: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..boxes.len() {
        for j in i..boxes.len() {
            if i == j {
                continue;
            }
            distances.push((distance(&boxes[i], &boxes[j]), i, j));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut connected_components: Vec<usize> = (0..boxes.len()).collect();
    for i in 0..distances.len() {
        let connection = distances[i];

        // Make connection & update connected components
        let comp_a = connected_components[connection.1];
        let comp_b = connected_components[connection.2];
        for comp in connected_components.iter_mut() {
            if *comp == comp_b {
                *comp = comp_a;
            }
        }

        // If theres a single connected component return it
        let first_comp = connected_components[0];
        if connected_components.iter().all(|&c| c == first_comp) {
            let result = (boxes[connection.1].x * boxes[connection.2].x) as u64;
            return Some(result);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
