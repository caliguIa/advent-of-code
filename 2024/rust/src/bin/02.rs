advent_of_code::solution!(2);

struct Report {
    levels: Vec<u32>,
}

fn generate_report(input: &str) -> Report {
    let levels: Vec<u32> = input
        .split(' ')
        .filter(|level| level.len() > 0)
        .map(|level| level.parse().unwrap())
        .collect();

    Report { levels }
}

fn extract_reports(input: &str) -> Vec<Report> {
    let reports = input
        .split(|char| char == '\n')
        .filter(|rep| rep.len() > 0)
        .map(|rep| generate_report(rep))
        .collect();

    reports
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = extract_reports(input);

    let mut safe_reports: u32 = 0;

    for report in reports {
        let is_increasing = &report.levels[0] < &report.levels[1];

        let is_safe = report.levels.iter().enumerate().all(|(index, &level)| {
            if index < report.levels.iter().len() - 1 {
                if is_increasing {
                    level < report.levels[index + 1] && report.levels[index + 1] - level <= 3
                } else {
                    level > report.levels[index + 1] && level - report.levels[index + 1] <= 3
                }
            } else {
                true
            }
        });

        if is_safe {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
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
