use std::collections::HashMap;

pub fn part_one() -> u32 {
    let (list1, list2) = parse_input();
    find_difference_score(list1, list2)
}

pub fn part_two() -> u32 {
    let (list1, list2) = parse_input();
    find_similarity_score(list1, list2)
}

fn find_difference_score(mut list1: Vec<u32>, mut list2: Vec<u32>) -> u32 {
    list1.sort();
    list2.sort();

    let mut difference = 0;
    for i in 0..list1.len() {
        difference += list1[i].abs_diff(list2[i])
    }

    difference
}

fn find_similarity_score(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    let mut second_list_number_frequency: HashMap<u32, u32> = HashMap::new();
    for location_id in list2 {
        second_list_number_frequency
            .entry(location_id)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    }

    let mut score = 0;
    for location_id in list1 {
        let frequency_in_second_list = second_list_number_frequency.get(&location_id).unwrap_or(&0);
        score += location_id * frequency_in_second_list;
    }

    score
}

// Assumptions
// location ids are positive
// no location ids are greater than max u32 size
// No errors / typos in the list
fn parse_input() -> (Vec<u32>, Vec<u32>) {
    let input = include_str!("./input.txt");
    let lines = input.lines();
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    for line in lines {
        let split = line.split_whitespace();
        for (index, value) in split.enumerate() {
            match index {
                0 => list1.push(value.parse().expect("invalid")),
                1 => list2.push(value.parse().expect("invalid")),
                _ => panic!("Invalid number of rows in input"),
            };
        }
    }

    assert!(
        list1.len() == list2.len(),
        "Input lists must have the same number of elements"
    );

    assert!(
        list1.len() > 0,
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
