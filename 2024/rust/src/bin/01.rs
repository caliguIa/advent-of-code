use std::{num::TryFromIntError, u32};

advent_of_code::solution!(1);

struct Lists {
    left: Vec<u32>,
    right: Vec<u32>,
}

fn generate_sorted_id_lists(input: &str) -> Lists {
    let mut lists = Lists {
        left: Vec::new(),
        right: Vec::new(),
    };

    // split the input on newlines and spaces, filter out empty strings, then parse id strings into u32s
    let list: Vec<u32> = input
        .split(|char| char == ' ' || char == '\n')
        .filter(|str| str.len() > 0)
        .map(|id| id.parse().unwrap())
        .collect();

    // push left and right list ids to their respective struct field
    for (index, id) in list.iter().enumerate() {
        if index % 2 == 0 {
            lists.right.push(*id);
        } else {
            lists.left.push(*id);
        }
    }

    lists.left.sort();
    lists.right.sort();

    lists
}

pub fn part_one(input: &str) -> Option<u32> {
    let lists = generate_sorted_id_lists(input);

    let mut res: u32 = 0;

    // for each lists ids convert to i32 then calculate diff
    for (index, id) in lists.left.iter().enumerate() {
        let diff = match (i32::try_from(*id), i32::try_from(lists.right[index])) {
            (Ok(left_id), Ok(right_id)) => Ok(left_id - right_id),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        };

        // get absolute diff and then convert to u32
        let abs_diff: Result<u32, TryFromIntError> = match diff {
            Ok(diff) => diff.abs().try_into(),
            Err(e) => Err(e),
        };

        res = res + abs_diff.unwrap();
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lists = generate_sorted_id_lists(input);

    let mut repetitions: Vec<u32> = Vec::new();

    for left_id in lists.left {
        // get the number of times an id from the left list appears in the right
        let occurances: u32 = lists
            .right
            .iter()
            .filter(|right_id| *right_id == &left_id)
            .collect::<Vec<&u32>>()
            .len()
            .try_into()
            .unwrap();

        repetitions.push(occurances * left_id);
    }

    Some(repetitions.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
