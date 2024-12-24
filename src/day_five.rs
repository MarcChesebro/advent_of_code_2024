use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

type RuleMap = HashMap<isize, Vec<isize>>;

pub fn part_one() -> isize {
    let rule_comparer = RuleComparer::new("inputs/day_five_rule_input.txt");

    let lists = process_lists("inputs/day_five_list_input.txt");

    lists.iter()
        // turn the list into a tuple of the original list and a new sorted list. This sorted list
        // will be sorted using the RuleComparer
        .map(|list| {
            let mut sorted_list = list.clone();
            sorted_list.sort_by(|x, y| rule_comparer.compare(*x, *y));

            (list, sorted_list)
        })
        // filter only the lists that match. these are already correctly sorted lists.
        .filter(|(list, sorted_list)| list_matches(list, &sorted_list))
        // get the middle number in the list and sum them up
        .map(|(_, sorted_list)| {
            let middle_index = sorted_list.len() / 2;
            *sorted_list.get(middle_index).unwrap()
        })
        .sum()
}

pub fn part_two() -> isize {
    let rule_comparer = RuleComparer::new("inputs/day_five_rule_input.txt");

    let lists = process_lists("inputs/day_five_list_input.txt");

    lists.iter()
        // turn the list into a tuple of the original list and a new sorted list. This sorted list
        // will be sorted using the RuleComparer
        .map(|list| {
            let mut sorted_list = list.clone();
            sorted_list.sort_by(|x, y| rule_comparer.compare(*x, *y));

            (list, sorted_list)
        })
        // filter only the lists that match. these are already correctly sorted lists.
        .filter(|(list, sorted_list)| !list_matches(list, &sorted_list))
        // get the middle number in the list and sum them up
        .map(|(_, sorted_list)| {
            let middle_index = sorted_list.len() / 2;
            *sorted_list.get(middle_index).unwrap()
        })
        .sum()
}



fn list_matches(list_one: &Vec<isize>, list_two: &Vec<isize>) -> bool {
    let matching = list_one
        .iter()
        .zip(list_two.iter())
        .filter(|&(a, b)| a == b)
        .count();
    matching == list_one.len()
}

struct RuleComparer {
    rules: RuleMap,
}

impl RuleComparer {
    fn new(file: &str) -> RuleComparer {
        RuleComparer {
            rules: process_rules(file),
        }
    }

    fn compare(&self, left: isize, right: isize) -> Ordering {
        if left == right {
            return Ordering::Equal;
        }

        match self.rules.get(&left) {
            Some(v) => {
                match v.contains(&right) {
                    true => Ordering::Less,
                    false => Ordering::Greater
                }
            }
            None => Ordering::Equal
        }
    }
}

fn process_rules(file: &str) -> RuleMap {
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");

    let rule_pairs = contents
        .split_whitespace()
        .map(|string| {
            string.splitn(2, '|').map(|x| x.parse::<isize>().unwrap())
        })
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()));

    let mut rule_map: RuleMap = HashMap::new();

    for (left, right) in rule_pairs {
        if rule_map.contains_key(&left) {
            rule_map.get_mut(&left).unwrap().push(right);
        } else {
            rule_map.insert(left, vec![right]);
        }
    }

    rule_map
}

fn process_lists(file: &str) -> Vec<Vec<isize>> {
    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");

    contents
        .split('\n')
        .map(|list| {
            list
                .split(',')
                .map(|n| n.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        assert_eq!(part_one(), 5391);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(), 6142);
    }
}