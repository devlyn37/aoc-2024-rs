use std::collections::HashMap;

pub fn part_one() -> u32 {
    let (list1, list2) = parse_input();
    find_difference_score(list1, list2)
}

pub fn part_two() -> u32 {
    let (list1, list2) = parse_input();
    find_similarity_score(list1, list2)
}

fn find_difference_score(mut list_one: Vec<u32>, mut list_two: Vec<u32>) -> u32 {
    list_one.sort();
    list_two.sort();

    list_one
        .iter()
        .zip(list_two.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn find_similarity_score(list_one: Vec<u32>, list_two: Vec<u32>) -> u32 {
    let mut list_two_occurances: HashMap<u32, u32> = HashMap::new();
    for location_id in list_two {
        list_two_occurances
            .entry(location_id)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    }

    list_one
        .into_iter()
        .map(|id| id * list_two_occurances.get(&id).unwrap_or(&0))
        .sum()
}

// Assumptions
// location ids are positive
// no location ids are greater than max u32 size
// No errors / typos in the list
fn parse_input() -> (Vec<u32>, Vec<u32>) {
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    for line in include_str!("./input.txt").lines() {
        let split = line.split_whitespace();
        for (index, value) in split.enumerate() {
            match index {
                0 => list1.push(value.parse().expect("invalid location id")),
                1 => list2.push(value.parse().expect("invalid location id")),
                _ => panic!("Invalid number of rows within input file"),
            };
        }
    }

    assert!(
        list1.len() == list2.len(),
        "Input lists must have the same number of elements"
    );

    assert!(
        !list1.is_empty(),
        "Input lists must have at least one element"
    );

    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        assert_eq!(
            find_difference_score(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            11
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 1879048);
    }

    #[test]
    fn test_part_two_simple() {
        assert_eq!(
            find_similarity_score(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            31
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 21024792);
    }
}
