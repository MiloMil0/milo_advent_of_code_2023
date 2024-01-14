use std::collections::HashMap;

use crate::util::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Record {
    pub spring_conditions: Vec<SpringCondition>,
    pub damaged_springs: Vec<usize>,
}

impl Record {
    pub fn new(line: &str) -> Self {
        let mut spring_conditions = Vec::new();
        let mut damaged_springs = Vec::new();

        let parts = line.split_whitespace().collect::<Vec<_>>();
        for c in parts[0].chars() {
            spring_conditions.push(parse_char_to_condition(c))
        }

        let numbers_str = parts[1].split(',').collect::<Vec<_>>();
        for num in numbers_str {
            if let Ok(num) = num.parse::<usize>() {
                damaged_springs.push(num);
            }
        }

        let multiplier = 5;
        spring_conditions = (0..multiplier)
            .flat_map(|_| {
                spring_conditions
                    .iter()
                    .cloned()
                    .chain(std::iter::once(SpringCondition::Unknown))
            })
            .collect();

        spring_conditions.pop();

        damaged_springs = damaged_springs
            .iter()
            .cloned()
            .cycle()
            .take(damaged_springs.len() * multiplier)
            .collect();

        Record {
            spring_conditions,
            damaged_springs,
        }
    }

    pub fn possible_combinations(&self, mem: &mut HashMap<Record, usize>) -> usize {
        if let Some(&result) = mem.get(self) {
            return result;
        }

        if self.damaged_springs.is_empty() {
            let value = match self
                .spring_conditions
                .iter()
                .any(|spring| *spring == SpringCondition::Damaged)
            {
                true => 0,
                false => 1,
            };

            mem.insert(self.clone(), value);

            return value;
        }

        let needed_space =
            self.damaged_springs.iter().sum::<usize>() + self.damaged_springs.len() - 1;

        if self.spring_conditions.len() < needed_space {
            mem.insert(self.clone(), 0);

            return 0;
        }

        let first = self.spring_conditions[0];
        if first == SpringCondition::Operational {
            let new_record = Record {
                spring_conditions: self.spring_conditions[1..].to_vec(),
                damaged_springs: self.damaged_springs.clone(),
            };

            let result = new_record.possible_combinations(mem);

            mem.insert(self.clone(), result);

            return result;
        }

        let group = self.damaged_springs[0];
        let are_all_non_operational = self.spring_conditions[..group]
            .iter()
            .all(|spring| *spring != SpringCondition::Operational);
        let end = (group + 1).min(self.spring_conditions.len());

        let mut solutions: usize = 0;

        if are_all_non_operational
            && ((self.spring_conditions.len() > group
                && self.spring_conditions[group] != SpringCondition::Damaged)
                || self.spring_conditions.len() <= group)
        {
            let new_record = Record {
                spring_conditions: self.spring_conditions[end..].to_vec(),
                damaged_springs: self.damaged_springs[1..].to_vec(),
            };
            solutions += new_record.possible_combinations(mem);
        }

        if first == SpringCondition::Unknown {
            let new_record = Record {
                spring_conditions: self.spring_conditions[1..].to_vec(),
                damaged_springs: self.damaged_springs.clone(),
            };
            solutions += new_record.possible_combinations(mem);
        }

        mem.insert(self.clone(), solutions);

        solutions
    }
}
