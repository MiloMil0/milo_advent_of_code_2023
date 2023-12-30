use crate::util::*;

#[derive(Debug)]
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

        Record {
            spring_conditions,
            damaged_springs,
        }
    }

    pub fn possible_combinations(&self) -> usize {
        if let Some(index) = self
            .spring_conditions
            .iter()
            .position(|spring| *spring == SpringCondition::Unknown)
        {
            let mut as_damaged_spring = self.spring_conditions.clone();
            as_damaged_spring[index] = SpringCondition::Damaged;
            let as_damaged = Record {
                spring_conditions: as_damaged_spring,
                damaged_springs: self.damaged_springs.to_vec(),
            };

            let mut as_operational_spring = self.spring_conditions.clone();
            as_operational_spring[index] = SpringCondition::Operational;
            let as_operational = Record {
                spring_conditions: as_operational_spring,
                damaged_springs: self.damaged_springs.to_vec(),
            };

            as_damaged.possible_combinations() + as_operational.possible_combinations()
        } else {
            if self.is_valid() {
                1
            } else {
                0
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut iter = self.spring_conditions.iter().peekable();
        let mut group_counts = Vec::new();

        while let Some(_) = iter.peek() {
            let count = iter
                .clone()
                .take_while(|&&x| x == SpringCondition::Damaged)
                .count();
            iter.nth(count);

            group_counts.push(count);
        }

        group_counts.retain(|&x| x > 0);

        group_counts == self.damaged_springs
    }
}
