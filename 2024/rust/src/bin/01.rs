use std::num::TryFromIntError;

advent_of_code::solution!(1);

struct Lists {
    left: Vec<u32>,
    right: Vec<u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut res: u32 = 0;

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

    return Some(res);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
